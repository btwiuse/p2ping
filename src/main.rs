use libp2p::futures::StreamExt;
use libp2p::swarm::{keep_alive, NetworkBehaviour, SwarmBuilder, SwarmEvent};
use libp2p::{identify, identity::Keypair, identity::PublicKey, ping, Multiaddr, PeerId};
use std::error::Error;

// const ZERO_KEY: [u8; 32] = [0u8; 32];
const P2PING_PROTOCOL_VERSION: &str = "/p2ping/0.0.0";
const LISTEN_ADDRS: &[&str] = &["/ip4/0.0.0.0/tcp/0", "/ip4/0.0.0.0/tcp/0/ws"];

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let local_key_pair = Keypair::generate_ed25519();
    // let local_key_pair = Keypair::ed25519_from_bytes(ZERO_KEY)?;
    let local_peer_id = PeerId::from(local_key_pair.public());
    println!("Local peer id: {local_peer_id:?}!");
    let transport = p2ping::dev_transport(local_key_pair.clone()).await?;
    let behaviour = Behaviour::new(P2PING_PROTOCOL_VERSION, local_key_pair.public());
    let mut swarm =
        SwarmBuilder::with_async_std_executor(transport, behaviour, local_peer_id).build();

    // Tell the swarm to listen on all interfaces and a random, OS-assigned
    // port.
    for addr in LISTEN_ADDRS {
        swarm.listen_on(addr.parse()?)?;
    }

    // Dial the peer identified by the multi-address given as the second
    // command-line argument, if any.
    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("Dialed {addr}")
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
            SwarmEvent::Behaviour(event) => println!("{event:?}"),
            x => println!("Unhandled: {x:?}"),
        }
    }
}

/// Our network behaviour.
///
/// For illustrative purposes, this includes the [`KeepAlive`](behaviour::KeepAlive) behaviour so a continuous sequence of
/// pings can be observed.
#[derive(NetworkBehaviour)]
struct Behaviour {
    keep_alive: keep_alive::Behaviour,
    ping: ping::Behaviour,
    identify: identify::Behaviour,
}

impl Behaviour {
    fn new(protocol: &str, pubkey: PublicKey) -> Self {
        Self {
            keep_alive: Default::default(),
            ping: Default::default(),
            identify: identify::Behaviour::new(identify::Config::new(protocol.to_string(), pubkey)),
        }
    }
}
