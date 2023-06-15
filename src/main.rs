use clap::Parser;
use hex::FromHex;
use libp2p::futures::StreamExt;
use libp2p::swarm::{SwarmBuilder, SwarmEvent};
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

#[async_std::main]
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
    let transport = p2ping::dev_transport(local_key_pair.clone()).await?;
    let behaviour = p2ping::Behaviour::new(
        &config.protocol_version,
        &config.agent_version,
        local_key_pair.public(),
    );
    let mut swarm =
        SwarmBuilder::with_async_std_executor(transport, behaviour, local_peer_id).build();

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
