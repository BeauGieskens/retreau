//! Shared OIDC sign-in and profile sync. No app credentials are issued here.

mod entra;
mod profile;

use std::{sync::Arc, time::Duration};

use axum::{
    Extension,
    extract::{Query, rejection::QueryRejection},
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
};
use base64::{Engine, engine::general_purpose::STANDARD};
use cookie::{Cookie, CookieJar, Key, SameSite, time::Duration as SignedDuration};
pub use entra::EntraProvider;
use jiff::Timestamp;
use openidconnect::{core::*, *};
use profile::AvatarUpdate;
use serde::{Deserialize, Serialize};
use worker::{Env, Fetch, Request, RequestInit, RequestRedirect};

const TRANSACTION_SECONDS: i64 = Duration::from_mins(10).as_secs() as i64;
type AuthResult<T> = Result<T, &'static str>;

type OidcClient<C> = Client<
    C,
    CoreAuthDisplay,
    CoreGenderClaim,
    CoreJweContentEncryptionAlgorithm,
    CoreJsonWebKey,
    CoreAuthPrompt,
    StandardErrorResponse<CoreErrorResponseType>,
    StandardTokenResponse<
        IdTokenFields<
            C,
            EmptyExtraTokenFields,
            CoreGenderClaim,
            CoreJweContentEncryptionAlgorithm,
            CoreJwsSigningAlgorithm,
        >,
        CoreTokenType,
    >,
    CoreTokenIntrospectionResponse,
    CoreRevocableToken,
    CoreRevocationErrorResponse,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
    EndpointNotSet,
>;

/// Settings and identity mapping supplied by an OIDC provider.
pub trait OidcProvider: Sized {
    type Claims: AdditionalClaims;

    const ID: &str;
    const FALLBACK_NAME: &str;
    const SCOPES: &[&str];
    const START_PATH: &str;
    const CALLBACK_PATH: &str;

    /// Load deployment configuration for this provider.
    fn load(env: &Env) -> AuthResult<Self>;

    /// Supply endpoints and credentials for the shared OIDC flow.
    fn oidc(&self) -> &OidcConfig;

    /// Map verified claims to a stable provider-scoped database subject.
    fn subject(&self, claims: &Self::Claims, sub: &str) -> AuthResult<String>;

    /// Cookie name is slightly different in local development vs secure deployments.
    fn cookie_name(&self) -> String {
        let id = Self::ID;
        if self.oidc().redirect.url().scheme() == "https" {
            format!("__Host-retreau-{id}")
        } else {
            format!("retreau-{id}-local")
        }
    }

    /// Fetch a user's photo using their freshly issued delegated token.
    async fn fetch_photo(&self, token: &AccessToken) -> AuthResult<AvatarUpdate> {
        let _ = token;
        Ok(AvatarUpdate::Keep)
    }

    /// Start browser authorization for this provider.
    async fn start(Extension(env): Extension<Arc<Env>>) -> Response {
        start_for::<Self>(&env).unwrap_or_else(|step: &str| {
            worker::console_error!("{} sign-in start failed: {step}", Self::ID);
            redirect("/login?status=failed", None)
        })
    }

    /// Verify browser binding and token claims before fetching profile data or touching D1.
    async fn finish(
        &self,
        env: &Env,
        headers: &HeaderMap,
        query: Result<Query<CallbackQuery>, QueryRejection>,
    ) -> AuthResult<bool> {
        let Query(query) = query.map_err(|_| "callback query")?;
        let config = self.oidc();
        let transaction = read_transaction(headers, &self.cookie_name(), &config.transaction_key)?;
        if query.state.map(CsrfToken::new).as_ref() != Some(&transaction.state) {
            return Err("state mismatch");
        }
        let code = match (query.code, query.error.as_deref()) {
            (Some(code), None) => AuthorizationCode::new(code),
            (None, Some("access_denied")) => return Ok(false),
            _ => return Err("authorization denied or missing code"),
        };
        let keys = CoreJsonWebKeySet::fetch_async(&config.jwks, &oidc_fetch)
            .await
            .map_err(|_| "fetch signing keys")?;
        let client = config.client::<Self::Claims>(keys);
        let token = client
            .exchange_code(code)
            .set_pkce_verifier(transaction.verifier)
            .request_async(&oidc_fetch)
            .await
            .map_err(|_| "code exchange")?;
        let verifier = client
            .id_token_verifier()
            .set_allowed_algs([config.signing_alg.clone()]);
        let id_token = token.id_token().ok_or("missing ID token")?;
        let claims = id_token
            .claims(&verifier, &transaction.nonce)
            .map_err(|_| "verify ID token")?;
        if let Some(expected) = claims.access_token_hash() {
            let alg = id_token
                .signing_alg()
                .map_err(|_| "ID token signing algorithm")?;
            let key = id_token
                .signing_key(&verifier)
                .map_err(|_| "ID token signing key")?;
            let actual = AccessTokenHash::from_token(token.access_token(), alg, key)
                .map_err(|_| "hash access token")?;
            if &actual != expected {
                return Err("access token mismatch");
            }
        }

        let subject = self.subject(claims.additional_claims(), claims.subject().as_str())?;
        let name = claims
            .name()
            .and_then(|name| name.get(None))
            .map(|name| name.as_str());
        let photo = self
            .fetch_photo(token.access_token())
            .await
            .inspect_err(|reason| {
                worker::console_warn!("{} profile photo sync skipped: {reason}", Self::ID)
            })
            .unwrap_or(AvatarUpdate::Keep);
        profile::sync(env, Self::ID, &subject, name, Self::FALLBACK_NAME, photo)
            .await
            .map_err(|_| "persist profile")?;
        Ok(true)
    }

    /// Convert provider callback results into local redirects.
    #[worker::send]
    async fn callback(
        Extension(env): Extension<Arc<Env>>,
        headers: HeaderMap,
        query: Result<Query<CallbackQuery>, QueryRejection>,
    ) -> Response {
        let provider = match Self::load(&env) {
            Ok(provider) => provider,
            Err(step) => {
                worker::console_warn!("{} sign-in callback failed: {step}", Self::ID);
                return redirect("/login?status=failed", None);
            }
        };
        let location = match provider.finish(&env, &headers, query).await {
            Ok(true) => "/login?status=complete",
            Ok(false) => "/login?status=cancelled",
            Err(step) => {
                worker::console_warn!("{} sign-in callback failed: {step}", Self::ID);
                "/login?status=failed"
            }
        };
        let cookie = provider
            .oidc()
            .transaction_cookie(provider.cookie_name(), String::new(), 0);
        redirect(location, Some(&cookie.to_string()))
    }
}

/// Protocol settings used independently of provider-specific claims.
pub struct OidcConfig {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect: RedirectUrl,
    transaction_key: Key,
    issuer: IssuerUrl,
    auth: AuthUrl,
    token: TokenUrl,
    jwks: JsonWebKeySetUrl,
    auth_type: AuthType,
    signing_alg: CoreJwsSigningAlgorithm,
}

impl OidcConfig {
    /// Use the same attributes when setting and clearing the transaction cookie.
    fn transaction_cookie(&self, name: String, value: String, max_age: i64) -> Cookie<'static> {
        Cookie::build((name, value))
            .path("/")
            .secure(self.redirect.url().scheme() == "https")
            .http_only(true)
            .same_site(SameSite::Lax)
            .max_age(SignedDuration::seconds(max_age))
            .build()
    }

    /// Build a client that uses this provider's claims and issuer.
    fn client<C: AdditionalClaims>(&self, keys: CoreJsonWebKeySet) -> OidcClient<C> {
        Client::new(self.client_id.clone(), self.issuer.clone(), keys)
            .set_auth_uri(self.auth.clone())
            .set_token_uri(self.token.clone())
            .set_auth_type(self.auth_type.clone())
            .set_client_secret(self.client_secret.clone())
            .set_redirect_uri(self.redirect.clone())
    }
}

/// Load a fixed redirect URI, allowing HTTP only for local development.
fn load_redirect_uri(env: &Env, path: &str) -> AuthResult<RedirectUrl> {
    let host = env
        .var("APP_HOST")
        .map_err(|_| "APP_HOST is required")?
        .to_string();
    let host = host
        .parse::<axum::http::uri::Authority>()
        .map_err(|_| "APP_HOST must be a hostname with optional port, without a scheme or path")?;
    let insecure = env
        .var("DISABLE_SECURE")
        .is_ok_and(|flag| flag.to_string() == "true");
    let scheme = if insecure { "http" } else { "https" };
    let redirect =
        RedirectUrl::new(format!("{scheme}://{host}{path}")).map_err(|_| "bad redirect URI")?;
    let url = redirect.url();
    if !url.username().is_empty()
        || url.password().is_some()
        || url.path() != path
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err("APP_HOST must contain only a hostname and optional port");
    }
    if insecure && !matches!(url.host_str(), Some("localhost" | "127.0.0.1" | "::1")) {
        return Err("DISABLE_SECURE is only allowed for localhost");
    }
    Ok(redirect)
}

/// Load the provider's AES-GCM cookie key without exposing it in error messages.
fn load_transaction_key(env: &Env, setting: &'static str) -> AuthResult<Key> {
    let value = env.secret(setting).map_err(|_| setting)?;
    let key = STANDARD
        .decode(value.to_string().trim())
        .map_err(|_| setting)?;
    if key.len() != 64 {
        return Err(setting);
    }
    Key::try_from(key.as_slice()).map_err(|_| setting)
}

/// Encrypted browser-bound state for one short-lived authorization attempt.
#[derive(Deserialize, Serialize)]
struct Transaction {
    state: CsrfToken,
    nonce: Nonce,
    verifier: PkceCodeVerifier,
    expires_at: i64,
}

/// Only consume callback fields needed for completion; never reflect provider error text.
#[derive(Deserialize)]
pub struct CallbackQuery {
    state: Option<String>,
    code: Option<String>,
    error: Option<String>,
}

/// Begin an OIDC transaction using the selected provider's config and scopes.
fn start_for<P: OidcProvider>(env: &Env) -> AuthResult<Response> {
    let provider = P::load(env)?;
    let config = provider.oidc();
    let client = config.client::<P::Claims>(CoreJsonWebKeySet::default());
    let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();
    let (url, state, nonce) = client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .add_scopes(P::SCOPES.iter().map(|scope| Scope::new((*scope).into())))
        .set_pkce_challenge(challenge)
        .url();
    let transaction = Transaction {
        state,
        nonce,
        verifier,
        expires_at: Timestamp::now().as_second() + TRANSACTION_SECONDS,
    };
    let payload = serde_json::to_string(&transaction).map_err(|_| "encode transaction")?;
    let mut jar = CookieJar::new();
    jar.private_mut(&config.transaction_key)
        .add(config.transaction_cookie(provider.cookie_name(), payload, TRANSACTION_SECONDS));
    let cookie = jar.delta().next().ok_or("seal transaction")?;
    Ok(redirect(url.as_str(), Some(&cookie.to_string())))
}

/// Decode the transaction only if its cookie authenticates and is still within its lifetime.
fn read_transaction(headers: &HeaderMap, name: &str, key: &Key) -> AuthResult<Transaction> {
    let mut cookies = headers
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .flat_map(Cookie::split_parse)
        .filter_map(Result::ok)
        .filter(|cookie| cookie.name() == name);
    let cookie = cookies.next().ok_or("missing transaction cookie")?;
    if cookies.next().is_some() {
        return Err("duplicate transaction cookie");
    }
    let mut jar = CookieJar::new();
    jar.add_original(cookie.into_owned());
    let cookie = jar
        .private(key)
        .get(name)
        .ok_or("invalid transaction cookie")?;
    let transaction: Transaction =
        serde_json::from_str(cookie.value()).map_err(|_| "invalid transaction payload")?;
    if transaction.expires_at <= Timestamp::now().as_second() {
        return Err("expired transaction");
    }
    Ok(transaction)
}

fn redirect(location: &str, cookie: Option<&str>) -> Response {
    // Skip caching and referrer forwarding for redirects that may carry OAuth parameters.
    let mut response = (
        StatusCode::SEE_OTHER,
        [
            (header::LOCATION, location),
            (header::CACHE_CONTROL, "no-store"),
            (header::REFERRER_POLICY, "no-referrer"),
        ],
    )
        .into_response();
    if let Some(cookie) = cookie {
        match axum::http::HeaderValue::from_str(cookie) {
            Ok(cookie) => {
                response.headers_mut().insert(header::SET_COOKIE, cookie);
            }
            Err(_) => worker::console_error!("Invalid OIDC transaction Set-Cookie header"),
        }
    }
    response
}

/// Adapt OIDC's HTTP client to Worker Fetch; never follow redirects carrying credentials.
async fn oidc_fetch(request: HttpRequest) -> worker::Result<HttpResponse> {
    let (parts, body) = request.into_parts();
    let headers = worker::Headers::new();
    for (name, value) in &parts.headers {
        headers.append(
            name.as_str(),
            value.to_str().map_err(|_| "invalid OIDC request header")?,
        )?;
    }
    let mut init = RequestInit::new();
    init.with_method(parts.method.to_string().into())
        .with_headers(headers)
        .with_redirect(RequestRedirect::Error);
    if !body.is_empty() {
        let body = String::from_utf8(body).map_err(|_| "invalid OIDC request body")?;
        init.with_body(Some(body.into()));
    }
    let request = Request::new_with_init(&parts.uri.to_string(), &init)?;
    let mut response = Fetch::Request(request).send().await?;
    let mut builder = axum::http::Response::builder().status(response.status_code());
    for (name, value) in response.headers().entries() {
        builder = builder.header(name, value);
    }
    Ok(builder.body(response.bytes().await?)?)
}
