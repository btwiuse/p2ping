use libp2p::identify;
use libp2p::identity::PublicKey;
use libp2p::ping;
use libp2p::relay;
use libp2p::swarm::NetworkBehaviour;
use libp2p::PeerId;

/// Our network behaviour.
///
/// For illustrative purposes, this includes the [`KeepAlive`](behaviour::KeepAlive) behaviour so a continuous sequence of
/// pings can be observed.
#[derive(NetworkBehaviour)]
pub struct Behaviour {
    ping: ping::Behaviour,
    identify: identify::Behaviour,
    relay: relay::Behaviour,
}

impl Behaviour {
    pub fn new(protocol: &str, agent_version: &str, pubkey: PublicKey) -> Self {
        let identify_config = identify::Config::new(protocol.to_string(), pubkey.clone())
            .with_agent_version(agent_version.to_string());
        Self {
            ping: Default::default(),
            relay: relay::Behaviour::new(PeerId::from(pubkey), Default::default()),
            identify: identify::Behaviour::new(identify_config),
        }
    }
}
