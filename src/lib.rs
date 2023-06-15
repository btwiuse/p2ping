use either::Either;
use libp2p::core;
use libp2p::dns;
use libp2p::identify;
use libp2p::noise;
use libp2p::ping;
use libp2p::swarm::{keep_alive, NetworkBehaviour};
use libp2p::tcp;
use libp2p::websocket;
use libp2p::yamux;
use libp2p::PeerId;
use libp2p::Transport;
use libp2p::{relay, identity, identity::PublicKey};

// libp2p::development_transport modified to support either tcp or websockets
//
// https://docs.rs/libp2p/latest/libp2p/fn.development_transport.html
// https://github.com/paritytech/substrate/blob/2cc1667545f5e715fb94d492a2a2385ec8e00c7c/client/network/src/transport.rs#L62-L78
pub async fn dev_transport(
    keypair: identity::Keypair,
) -> std::io::Result<core::transport::Boxed<(PeerId, core::muxing::StreamMuxerBox)>> {
    let transport = {
        // Main transport: DNS(TCP)
        let tcp_config = tcp::Config::new().nodelay(true);
        let tcp_trans = tcp::async_io::Transport::new(tcp_config.clone());
        let dns_init = dns::DnsConfig::system(tcp_trans).await;

        if let Ok(dns) = dns_init {
            // WS + WSS transport
            //
            // Main transport can't be used for `/wss` addresses because WSS transport needs
            // unresolved addresses (BUT WSS transport itself needs an instance of DNS transport to
            // resolve and dial addresses).
            let tcp_trans = tcp::async_io::Transport::new(tcp_config);
            let dns_for_wss = dns::DnsConfig::system(tcp_trans)
                .await
                .expect("same system_conf & resolver to work");
            Either::Left(websocket::WsConfig::new(dns_for_wss).or_transport(dns))
        } else {
            // In case DNS can't be constructed, fallback to TCP + WS (WSS won't work)
            let tcp_trans = tcp::async_io::Transport::new(tcp_config.clone());
            let desktop_trans = websocket::WsConfig::new(tcp_trans)
                .or_transport(tcp::async_io::Transport::new(tcp_config));
            Either::Right(desktop_trans)
        }
    };

    // https://github.com/paritytech/substrate/blob/2cc1667545f5e715fb94d492a2a2385ec8e00c7c/client/network/src/transport.rs#L83
    let noise_config = noise::Config::new(&keypair).expect("Can create noise config. qed");

    Ok(transport
        .upgrade(core::upgrade::Version::V1)
        .authenticate(noise_config)
        .multiplex(yamux::Config::default())
        .timeout(std::time::Duration::from_secs(20))
        .boxed())
}

/// Our network behaviour.
///
/// For illustrative purposes, this includes the [`KeepAlive`](behaviour::KeepAlive) behaviour so a continuous sequence of
/// pings can be observed.
#[derive(NetworkBehaviour)]
pub struct Behaviour {
    keep_alive: keep_alive::Behaviour,
    ping: ping::Behaviour,
    identify: identify::Behaviour,
    relay: relay::Behaviour,
}

impl Behaviour {
    pub fn new(protocol: &str, agent_version: &str, pubkey: PublicKey) -> Self {
        let identify_config = identify::Config::new(protocol.to_string(), pubkey.clone())
            .with_agent_version(agent_version.to_string());
        Self {
            keep_alive: Default::default(),
            ping: Default::default(),
            relay: relay::Behaviour::new(PeerId::from(pubkey), Default::default()),
            identify: identify::Behaviour::new(identify_config),
        }
    }
}
