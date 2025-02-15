use crate::{PortalMessage, TcpPortalWorker, TcpRouterHandle};
use ockam_core::{async_trait, Result, Routed, Worker};
use ockam_node::Context;
use ockam_transport_core::TransportError;
use tracing::debug;

/// A TCP Portal Outlet listen worker
///
/// TCP Portal Outlet listen workers are created by `TcpTransport`
/// after a call is made to
/// [`TcpTransport::create_outlet`](crate::TcpTransport::create_outlet).
pub(crate) struct TcpOutletListenWorker {
    peer: String,
}

impl TcpOutletListenWorker {
    /// Create a new `TcpOutletListenWorker`
    pub(crate) fn new(peer: String) -> Self {
        Self { peer }
    }
}

#[async_trait]
impl Worker for TcpOutletListenWorker {
    type Context = Context;
    type Message = PortalMessage;

    async fn handle_message(
        &mut self,
        ctx: &mut Self::Context,
        msg: Routed<Self::Message>,
    ) -> Result<()> {
        let return_route = msg.return_route();

        if let PortalMessage::Ping = msg.body() {
        } else {
            return Err(TransportError::Protocol.into());
        }

        let (peer_addr, _) = TcpRouterHandle::resolve_peer(self.peer.clone())?;

        let address =
            TcpPortalWorker::start_new_outlet(ctx, peer_addr, return_route.clone()).await?;

        debug!("Created Tcp Outlet at {}", &address);

        Ok(())
    }
}
