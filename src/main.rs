use libp2p::futures::StreamExt;
use libp2p::swarm::{keep_alive, NetworkBehaviour, SwarmBuilder, SwarmEvent};
use libp2p::{identity::Keypair, ping, Multiaddr, PeerId};
use std::error::Error;

const ZERO_KEY: [u8; 32] = [0u8; 32];

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let local_key_pair = Keypair::generate_ed25519();
    let local_key_pair = Keypair::ed25519_from_bytes(ZERO_KEY)?;
    let local_peer_id = PeerId::from(local_key_pair.public());
    println!("Local peer id: {local_peer_id:?}!");
    let transport = libp2p::development_transport(local_key_pair).await?;
    let behaviour = Behaviour::default();
    let mut swarm =
        SwarmBuilder::with_async_std_executor(transport, behaviour, local_peer_id).build();

    // Tell the swarm to listen on all interfaces and a random, OS-assigned
    // port.
    swarm.listen_on("/ip4/0.0.0.0/tcp/33333".parse()?)?;

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
            _ => {}
        }
    }
}

/// Our network behaviour.
///
/// For illustrative purposes, this includes the [`KeepAlive`](behaviour::KeepAlive) behaviour so a continuous sequence of
/// pings can be observed.
#[derive(NetworkBehaviour, Default)]
struct Behaviour {
    keep_alive: keep_alive::Behaviour,
    ping: ping::Behaviour,
}
