use libp2p::swarm::{keep_alive, NetworkBehaviour};
use libp2p::{identity::Keypair, ping, PeerId};
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
    Ok(())
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
