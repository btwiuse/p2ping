# p2ping

ping a libp2p peer via the ping protocol (`/ipfs/ping/1.0.0`)

```
$ cargo run /dns/kusama-connect-0.parity.io/tcp/443/wss
Local peer id: PeerId("12D3KooWEEPQNzmwmwSB63rAT1pnJLso8GprdNtBM5F8sg92SsxL")!
Dialed /dns/kusama-connect-0.parity.io/tcp/443/wss
Listening on "/ip4/127.0.0.1/tcp/46823/ws"
Listening on "/ip4/172.17.0.10/tcp/46823/ws"
Unhandled: ConnectionEstablished { peer_id: PeerId("12D3KooWBjxpFhVNM9poSsMEfdnXJaSWSZQ7otK9aV1SPA9zJp5W"), endpoint: Dialer { address: "/dns/kusama-connect-0.parity.io/tcp/443/wss", role_override: Dialer }, num_established: 1, concurrent_dial_errors: Some([]), established_in: 1.269186947s }
BehaviourEvent: Event { peer: PeerId("12D3KooWBjxpFhVNM9poSsMEfdnXJaSWSZQ7otK9aV1SPA9zJp5W"), result: Ok(Pong) }
BehaviourEvent: Event { peer: PeerId("12D3KooWBjxpFhVNM9poSsMEfdnXJaSWSZQ7otK9aV1SPA9zJp5W"), result: Ok(Ping { rtt: 143.03252ms }) }
BehaviourEvent: Event { peer: PeerId("12D3KooWBjxpFhVNM9poSsMEfdnXJaSWSZQ7otK9aV1SPA9zJp5W"), result: Ok(Pong) }
BehaviourEvent: Event { peer: PeerId("12D3KooWBjxpFhVNM9poSsMEfdnXJaSWSZQ7otK9aV1SPA9zJp5W"), result: Ok(Ping { rtt: 142.90243ms }) }
...
```

Note: only WS/WSS transport is supported, if you want TCP, follow the first referenced tutorial

References:
- https://docs.rs/libp2p/latest/libp2p/tutorials/ping/index.html
- https://github.com/libp2p/specs/blob/master/ping/ping.md
- https://github.com/libp2p/specs/tree/master/identify
