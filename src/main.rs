use clap::Parser;
use hex::FromHex;
use libp2p::futures::StreamExt;
use libp2p::swarm::SwarmEvent;
use libp2p::SwarmBuilder;
use libp2p::{identity::Keypair, Multiaddr, PeerId};
use std::error::Error;

#[derive(clap::Parser)]
pub struct Config {
    #[clap(long, default_value = "/p2ping/0.0.0")]
    /// P2ping identify protocol version
    protocol_version: String,
    #[clap(long, default_value = "p2ping/dev")]
    /// P2ping identify agent version
    agent_version: String,
    #[clap(long, short = 'k')]
    /// The 32-byte secret key to use for libp2p networking. If unsupplied, a random one will be generated,
    ///   e.g.
    ///   0x0000000000000000000000000000000000000000000000000000000000000000
    node_key: Option<String>,
    #[clap(long, short = 'l')]
    /// Listening address,
    ///   e.g.
    ///   /ip4/0.0.0.0/tcp/0,
    ///   /ip4/0.0.0.0/tcp/0/ws
    listen_addr: Vec<String>,
    /// Peer address,
    ///   e.g.
    ///   /ip4/127.0.0.1/tcp/30333,
    ///   /dns/polkadot-connect-0.parity.io/tcp/443/wss
    peer_addr: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::parse();

    if config.peer_addr.is_none() && config.listen_addr.is_empty() {
        println!("[WARN]: no listen-addr or peer-addr specified, please refer to `p2ping --help` for help")
    }

    let local_key_pair = match config.node_key {
        Some(key) => {
            let key_bytes = <[u8; 32]>::from_hex(key.trim_start_matches("0x"))?;
            Keypair::ed25519_from_bytes(key_bytes)?
        }
        None => Keypair::generate_ed25519(),
    };
    let local_peer_id = PeerId::from(local_key_pair.public());
    println!("Local peer id: {local_peer_id:?}");
    let behaviour = p2ping::Behaviour::new(
        &config.protocol_version,
        &config.agent_version,
        local_key_pair.public(),
    );
    let mut swarm = SwarmBuilder::with_existing_identity(local_key_pair)
        .with_tokio()
        .with_tcp(
            Default::default(),
            (libp2p_tls::Config::new, libp2p_noise::Config::new),
            libp2p_yamux::Config::default,
        )?
        .with_quic()
        .with_dns()?
        .with_websocket(
            (libp2p_tls::Config::new, libp2p_noise::Config::new),
            libp2p_yamux::Config::default,
        )
        .await?
        .with_relay_client(
            (libp2p_tls::Config::new, libp2p_noise::Config::new),
            libp2p_yamux::Config::default,
        )?
        .with_behaviour(|_key, _relay| behaviour)?
        .with_swarm_config(|cfg| {
            cfg.with_idle_connection_timeout(std::time::Duration::from_secs(60))
        })
        .build();

    // Tell the swarm to listen on specified multiaddrs.
    for addr in &config.listen_addr {
        swarm.listen_on(addr.parse()?)?;
    }

    // Dial the peer identified by the peer_addr, if any.
    if let Some(peer_addr) = config.peer_addr {
        let remote: Multiaddr = peer_addr.parse()?;
        swarm.dial(remote)?;
        println!("Dialed {peer_addr}")
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
            SwarmEvent::Behaviour(event) => println!("{event:?}"),
            x => println!("Unhandled: {x:?}"),
        }
    }
}
