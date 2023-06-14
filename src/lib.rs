use either::Either;
use libp2p::core;
use libp2p::dns;
use libp2p::identity;
use libp2p::noise;
use libp2p::tcp;
use libp2p::websocket;
use libp2p::yamux;
use libp2p::PeerId;
use libp2p::Transport;

// libp2p::development_transport modified to support either tcp or websockets
//
// https://docs.rs/libp2p/latest/libp2p/fn.development_transport.html
pub async fn dev_transport(
    keypair: identity::Keypair,
    is_ws: bool,
) -> std::io::Result<core::transport::Boxed<(PeerId, core::muxing::StreamMuxerBox)>> {
    let dns_tcp = dns::DnsConfig::system(tcp::async_io::Transport::new(
        tcp::Config::new().nodelay(true),
    ))
    .await?;
    let transport = if !is_ws {
        Either::Left(dns_tcp)
    } else {
        let ws_dns_tcp = websocket::WsConfig::new(dns_tcp);
        Either::Right(ws_dns_tcp)
    };

    let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
        .into_authentic(&keypair)
        .unwrap();
    let noise_config = noise::NoiseConfig::xx(noise_keys).into_authenticated();

    Ok(transport
        .upgrade(core::upgrade::Version::V1)
        .authenticate(noise_config)
        .multiplex(yamux::Config::default())
        .timeout(std::time::Duration::from_secs(20))
        .boxed())
}
