use worker::*;

#[durable_object]
pub struct Board {
    state: State,
    env: Env,
}

impl DurableObject for Board {
    fn new(state: State, env: Env) -> Self {
        // load from something: board ID?
        Self { state, env }
    }

    async fn fetch(&self, _req: Request) -> Result<Response> {
        // This DO is only doing WebSocket stuff for a board
        let WebSocketPair { client, server } = WebSocketPair::new()?;
        self.state.accept_web_socket(&server);
        Response::from_websocket(client)
    }

    async fn websocket_message(
        &self,
        ws: WebSocket,
        message: WebSocketIncomingMessage,
    ) -> Result<()> {
        Ok(())
    }

    async fn websocket_close(
        &self,
        ws: WebSocket,
        code: usize,
        reason: String,
        _was_clean: bool,
    ) -> Result<()> {
        let code = code.try_into().ok();
        ws.close(code, Some(reason))
    }

    async fn websocket_error(&self, ws: WebSocket, error: Error) -> Result<()> {
        ws.close(None, Some(format!("WebSocket error: {error}")))
    }
}
