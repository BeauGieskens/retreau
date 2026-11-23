//! Microsoft Entra identity and delegated Graph profile data.

use base64::{Engine, engine::general_purpose::STANDARD};
use futures_util::StreamExt;
use openidconnect::{
    AccessToken, AdditionalClaims, AuthType, AuthUrl, ClientId, ClientSecret, IssuerUrl,
    JsonWebKeySetUrl, TokenUrl, core::CoreJwsSigningAlgorithm,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use worker::{Env, Fetch, Headers, Method, Request, RequestInit, RequestRedirect};

use super::{
    AuthResult, OidcConfig, OidcProvider, load_redirect_uri, load_transaction_key,
    profile::AvatarUpdate,
};

const PHOTO_URL: &str = "https://graph.microsoft.com/v1.0/me/photos/48x48/$value";
const MAX_PHOTO_BYTES: usize = 64 * 1024;

/// Directory identity claims, distinct from OIDC's app-specific `sub`.
#[derive(Debug, Deserialize, Serialize)]
pub struct EntraClaims {
    tid: Uuid,
    oid: Uuid,
}
impl AdditionalClaims for EntraClaims {}

/// Configuration for the single Entra tenant assigned to this deployment.
pub struct EntraProvider {
    tenant: Uuid,
    oidc: OidcConfig,
}

impl OidcProvider for EntraProvider {
    type Claims = EntraClaims;

    const ID: &str = "entra";
    const FALLBACK_NAME: &str = "Entra user";
    const SCOPES: &[&str] = &["profile", "https://graph.microsoft.com/User.Read"];
    const START_PATH: &str = "/auth/entra/start";
    const CALLBACK_PATH: &str = "/auth/entra/callback";

    fn load(env: &Env) -> AuthResult<Self> {
        let setting = |name| env.var(name).map(|var| var.to_string()).map_err(|_| name);
        let secret = |name| {
            env.secret(name)
                .map(|var| var.to_string())
                .map_err(|_| name)
        };
        let tenant =
            Uuid::parse_str(&setting("ENTRA_TENANT_ID")?).map_err(|_| "ENTRA_TENANT_ID")?;
        let client_id =
            Uuid::parse_str(&setting("ENTRA_CLIENT_ID")?).map_err(|_| "ENTRA_CLIENT_ID")?;
        // Pin issuer and protocol endpoints to the tenant, never `common`.
        let authority = format!("https://login.microsoftonline.com/{tenant}");
        Ok(Self {
            tenant,
            oidc: OidcConfig {
                client_id: ClientId::new(client_id.to_string()),
                client_secret: ClientSecret::new(secret("ENTRA_CLIENT_SECRET")?),
                redirect: load_redirect_uri(env, Self::CALLBACK_PATH)?,
                transaction_key: load_transaction_key(env, "ENTRA_TRANSACTION_KEY")?,
                issuer: IssuerUrl::new(format!("{authority}/v2.0")).map_err(|_| "issuer URL")?,
                auth: AuthUrl::new(format!("{authority}/oauth2/v2.0/authorize"))
                    .map_err(|_| "authorize URL")?,
                token: TokenUrl::new(format!("{authority}/oauth2/v2.0/token"))
                    .map_err(|_| "token URL")?,
                jwks: JsonWebKeySetUrl::new(format!("{authority}/discovery/v2.0/keys"))
                    .map_err(|_| "JWKS URL")?,
                auth_type: AuthType::RequestBody,
                signing_alg: CoreJwsSigningAlgorithm::RsaSsaPkcs1V15Sha256,
            },
        })
    }

    fn oidc(&self) -> &OidcConfig {
        &self.oidc
    }

    fn subject(&self, claims: &Self::Claims, _sub: &str) -> AuthResult<String> {
        if claims.tid != self.tenant || claims.oid.is_nil() {
            return Err("directory identity mismatch");
        }
        Ok(format!("{}:{}", claims.tid, claims.oid))
    }

    async fn fetch_photo(&self, token: &AccessToken) -> AuthResult<AvatarUpdate> {
        let headers = Headers::new();
        headers
            .set("Authorization", &format!("Bearer {}", token.secret()))
            .map_err(|_| "could not set request headers")?;
        let mut init = RequestInit::new();
        init.with_method(Method::Get)
            .with_headers(headers)
            .with_redirect(RequestRedirect::Error);
        let request = Request::new_with_init(PHOTO_URL, &init)
            .map_err(|_| "could not build Graph request")?;
        let mut response = Fetch::Request(request)
            .send()
            .await
            .map_err(|_| "Graph request failed")?;
        match response.status_code() {
            200 => (),
            404 => return Ok(AvatarUpdate::Clear),
            401 | 403 => return Err("Graph denied photo access"),
            _ => return Err("unexpected Graph photo status"),
        }

        let content_type = response
            .headers()
            .get("Content-Type")
            .map_err(|_| "could not read photo content type")?
            .ok_or("missing photo content type")?;
        let media_type = content_type.split(';').next().unwrap_or_default().trim();
        let (mime, signature): (&str, &[u8]) = if media_type.eq_ignore_ascii_case("image/jpeg") {
            ("image/jpeg", b"\xff\xd8\xff")
        } else if media_type.eq_ignore_ascii_case("image/png") {
            ("image/png", b"\x89PNG\r\n\x1a\n")
        } else {
            return Err("unsupported photo content type");
        };

        let mut stream = response.stream().map_err(|_| "missing photo body")?;
        let mut bytes = Vec::new();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|_| "photo body read failed")?;
            if chunk.len() > MAX_PHOTO_BYTES - bytes.len() {
                return Err("photo exceeded size limit");
            }
            bytes.extend_from_slice(&chunk);
        }
        if !bytes.starts_with(signature) {
            return Err("photo signature did not match content type");
        }

        let mut url = format!("data:{mime};base64,");
        STANDARD.encode_string(&bytes, &mut url);
        Ok(AvatarUpdate::Replace(url))
    }
}
