# p2ping

ping a libp2p peer via the ping protocol (`/ipfs/ping/1.0.0`)

start a local peer

```
$ p2ping
Local peer id: PeerId("12D3KooWAPdkjL5PdpZwXUM3nrQ4BntYCxkCrsD4zh9yWaa2VYck")!
Listening on "/ip4/127.0.0.1/tcp/34619"
Listening on "/ip4/172.17.0.10/tcp/34619"
...
```

ping a local peer

```
$ p2ping /ip4/127.0.0.1/tcp/36017
Local peer id: PeerId("12D3KooWKYZJ2me7H5zCkCb7f7Qm2cpuY2CVh6E9vtA7ipwFJJmS")!
Dialed /ip4/127.0.0.1/tcp/36017
Listening on "/ip4/127.0.0.1/tcp/35751"
Listening on "/ip4/172.17.0.10/tcp/35751"
Unhandled: ConnectionEstablished { peer_id: PeerId("12D3KooWL6xRw3oM6BsufNvYjsAo2RMYgrdZZopDDXgiUSmrHTGz"), endpoint: Dialer { address: "/ip4/127.0.0.1/tcp/36017", role_override: Dialer }, num_established: 1, concurrent_dial_errors: Some([]), established_in: 2.530644ms }
BehaviourEvent: Event { peer: PeerId("12D3KooWL6xRw3oM6BsufNvYjsAo2RMYgrdZZopDDXgiUSmrHTGz"), result: Ok(Pong) }
BehaviourEvent: Event { peer: PeerId("12D3KooWL6xRw3oM6BsufNvYjsAo2RMYgrdZZopDDXgiUSmrHTGz"), result: Ok(Ping { rtt: 387.365µs }) }
BehaviourEvent: Sent { peer_id: PeerId("12D3KooWL6xRw3oM6BsufNvYjsAo2RMYgrdZZopDDXgiUSmrHTGz") }
BehaviourEvent: Received { peer_id: PeerId("12D3KooWL6xRw3oM6BsufNvYjsAo2RMYgrdZZopDDXgiUSmrHTGz"), info: Info { public_key: Ed25519(PublicKey(compressed): 98d4e6f82ab4d22ac6fce58b2f9ada1f590db98effe1a93644af77b3312217), protocol_version: "/p2ping/0.0.0", agent_version: "rust-libp2p/0.42.2", listen_addrs: ["/ip4/127.0.0.1/tcp/36017", "/ip4/172.17.0.10/tcp/36017"], protocols: ["/ipfs/ping/1.0.0", "/ipfs/id/1.0.0", "/ipfs/id/push/1.0.0"], observed_addr: "/ip4/127.0.0.1/tcp/53934" } }
...
```

ping a local substrate node

```
$ p2ping /ip4/127.0.0.1/tcp/30333/ws
Local peer id: PeerId("12D3KooWBFv1PYQhEbnqQ2iRbimWkQ4WtcXoz97jrgUDu6CjaWem")!
Dialed /ip4/127.0.0.1/tcp/30333/ws
Listening on "/ip4/127.0.0.1/tcp/43503/ws"
Listening on "/ip4/172.25.225.112/tcp/43503/ws"
Listening on "/ip4/10.42.1.0/tcp/43503/ws"
Listening on "/ip4/10.42.1.1/tcp/43503/ws"
Unhandled: ConnectionEstablished { peer_id: PeerId("12D3KooWAbEVvwBD1XMewYAmUxLyv2pPBQYfvVgSKqece8wmuzb3"), endpoint: Dialer { address: "/ip4/127.0.0.1/tcp/30333/ws", role_override: Dialer }, num_established: 1, concurrent_dial_errors: Some([]), established_in: 1.357398ms }
BehaviourEvent: Event { peer: PeerId("12D3KooWAbEVvwBD1XMewYAmUxLyv2pPBQYfvVgSKqece8wmuzb3"), result: Ok(Pong) }
BehaviourEvent: Event { peer: PeerId("12D3KooWAbEVvwBD1XMewYAmUxLyv2pPBQYfvVgSKqece8wmuzb3"), result: Ok(Ping { rtt: 185.129µs }) }
BehaviourEvent: Sent { peer_id: PeerId("12D3KooWAbEVvwBD1XMewYAmUxLyv2pPBQYfvVgSKqece8wmuzb3") }
BehaviourEvent: Received { peer_id: PeerId("12D3KooWAbEVvwBD1XMewYAmUxLyv2pPBQYfvVgSKqece8wmuzb3"), info: Info { public_key: Ed25519(PublicKey(compressed): b7e76fe7196a9ab3bb5fae19a5aa8928f18564abb5d79252d3ccb73f49289a), protocol_version: "/substrate/1.0", agent_version: "Substrate Node/v3.0.0-dev-416b0f50bba (overrated-curve-2441)", listen_addrs: ["/ip4/127.0.0.1/tcp/30333/ws"], protocols: ["/5b5d336a176e90a66918c742376f96bdfeccc43b1da64b69ccf2b3ac7a01a84a/block-announces/1", "/sup/block-announces/1", "/5b5d336a176e90a66918c742376f96bdfeccc43b1da64b69ccf2b3ac7a01a84a/transactions/1", "/sup/transactions/1", "/5b5d336a176e90a66918c742376f96bdfeccc43b1da64b69ccf2b3ac7a01a84a/grandpa/1", "/paritytech/grandpa/1", "/ipfs/ping/1.0.0", "/ipfs/id/1.0.0", "/ipfs/id/push/1.0.0", "/5b5d336a176e90a66918c742376f96bdfeccc43b1da64b69ccf2b3ac7a01a84a/kad", "/sup/kad", "/5b5d336a176e90a66918c742376f96bdfeccc43b1da64b69ccf2b3ac7a01a84a/sync/2", "/sup/sync/2", "/5b5d336a176e90a66918c742376f96bdfeccc43b1da64b69ccf2b3ac7a01a84a/light/2", "/sup/light/2", "/5b5d336a176e90a66918c742376f96bdfeccc43b1da64b69ccf2b3ac7a01a84a/sync/warp", "/sup/sync/warp", "/5b5d336a176e90a66918c742376f96bdfeccc43b1da64b69ccf2b3ac7a01a84a/state/2", "/sup/state/2"], observed_addr: "/ip4/127.0.0.1/tcp/53560/ws" } }
...
```

ping a polkadot node

```
$ p2ping /dns/polkadot-connect-0.parity.io/tcp/443/wss
Local peer id: PeerId("12D3KooWBn8V34vXTrPQUi2TzD2W4af56YJrR5JzS8CW6YwLvCP4")!
Dialed /dns/polkadot-connect-0.parity.io/tcp/443/wss
Listening on "/ip4/127.0.0.1/tcp/36287/ws"
Listening on "/ip4/172.17.0.10/tcp/36287/ws"
Unhandled: ConnectionEstablished { peer_id: PeerId("12D3KooWEPmjoRpDSUuiTjvyNDd8fejZ9eNWH5bE965nyBMDrB4o"), endpoint: Dialer { address: "/dns/polkadot-connect-0.parity.io/tcp/443/wss", role_override: Dialer }, num_established: 1, concurrent_dial_errors: Some([]), established_in: 997.157414ms }
BehaviourEvent: Event { peer: PeerId("12D3KooWEPmjoRpDSUuiTjvyNDd8fejZ9eNWH5bE965nyBMDrB4o"), result: Ok(Pong) }
BehaviourEvent: Event { peer: PeerId("12D3KooWEPmjoRpDSUuiTjvyNDd8fejZ9eNWH5bE965nyBMDrB4o"), result: Ok(Ping { rtt: 141.852165ms }) }
BehaviourEvent: Sent { peer_id: PeerId("12D3KooWEPmjoRpDSUuiTjvyNDd8fejZ9eNWH5bE965nyBMDrB4o") }
BehaviourEvent: Received { peer_id: PeerId("12D3KooWEPmjoRpDSUuiTjvyNDd8fejZ9eNWH5bE965nyBMDrB4o"), info: Info { public_key: Ed25519(PublicKey(compressed): 43fd90a5a2643f69542b1c2fe8d9483f41196affdc5b20d016b86357be436ac), protocol_version: "/substrate/1.0", agent_version: "Parity Polkadot/v0.9.42-9b1fc27cec4 (polkadot-connect-0)", listen_addrs: ["/ip4/35.234.84.149/tcp/30333", "/ip4/35.234.84.149/tcp/30334/ws", "/dns/polkadot-connect-0.parity.io/tcp/30333", "/dns/polkadot-connect-0.parity.io/tcp/30334/ws", "/ip4/127.0.0.1/tcp/30333", "/ip4/127.0.0.1/tcp/30334/ws", "/ip4/127.0.0.1/tcp/30333", "/ip4/10.16.0.61/tcp/30333", "/ip4/127.0.0.1/tcp/30334/ws"], protocols: ["/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/block-announces/1", "/dot/block-announces/1", "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/transactions/1", "/dot/transactions/1", "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/grandpa/1", "/paritytech/grandpa/1", "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/validation/1", "/polkadot/validation/1", "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/collation/1", "/polkadot/collation/1", "/ipfs/ping/1.0.0", "/ipfs/id/1.0.0", "/ipfs/id/push/1.0.0", "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/kad", "/dot/kad", "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/send_dispute/1", "/polkadot/send_dispute/1", "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/sync/warp", "/dot/sync/warp", "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/req_collation/1", "/polkadot/req_collation/1", "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/req_available_data/1", "/polkadot/req_available_data/1", "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/req_chunk/1", "/polkadot/req_chunk/1", "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/sync/2", "/dot/sync/2", "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/light/2", "/dot/light/2", "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/state/2", "/dot/state/2", "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/req_statement/1", "/polkadot/req_statement/1", "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/req_pov/1", "/polkadot/req_pov/1"], observed_addr: "/ip4/127.0.0.1/tcp/50538/ws" } }
...
```

ping a kusama node

```
$ p2ping /dns/kusama-connect-0.parity.io/tcp/443/wss
Local peer id: PeerId("12D3KooWGdJxhNULZg8fUMG7hfXfKxhVjmRSH6fVebY1TeGm7xMw")!
Dialed /dns/kusama-connect-0.parity.io/tcp/443/wss
Listening on "/ip4/127.0.0.1/tcp/46653/ws"
Listening on "/ip4/172.17.0.10/tcp/46653/ws"
Unhandled: ConnectionEstablished { peer_id: PeerId("12D3KooWBjxpFhVNM9poSsMEfdnXJaSWSZQ7otK9aV1SPA9zJp5W"), endpoint: Dialer { address: "/dns/kusama-connect-0.parity.io/tcp/443/wss", role_override: Dialer }, num_established: 1, concurrent_dial_errors: Some([]), established_in: 1.008873122s }
BehaviourEvent: Event { peer: PeerId("12D3KooWBjxpFhVNM9poSsMEfdnXJaSWSZQ7otK9aV1SPA9zJp5W"), result: Ok(Pong) }
BehaviourEvent: Event { peer: PeerId("12D3KooWBjxpFhVNM9poSsMEfdnXJaSWSZQ7otK9aV1SPA9zJp5W"), result: Ok(Ping { rtt: 141.629363ms }) }
BehaviourEvent: Sent { peer_id: PeerId("12D3KooWBjxpFhVNM9poSsMEfdnXJaSWSZQ7otK9aV1SPA9zJp5W") }
BehaviourEvent: Received { peer_id: PeerId("12D3KooWBjxpFhVNM9poSsMEfdnXJaSWSZQ7otK9aV1SPA9zJp5W"), info: Info { public_key: Ed25519(PublicKey(compressed): 1c96a51d575dd29f73270ef65da537783e4c84d60e8fd80a695b9d3fa8d19), protocol_version: "/substrate/1.0", agent_version: "Parity Polkadot/v0.9.42-9b1fc27cec4 (kusama-connect-0)", listen_addrs: ["/ip4/34.89.208.238/tcp/30333", "/dns/kusama-connect-0.parity.io/tcp/30333", "/dns/kusama-connect-0.parity.io/tcp/30334/ws", "/ip4/34.89.208.238/tcp/30334/ws", "/ip4/127.0.0.1/tcp/30334/ws", "/ip4/127.0.0.1/tcp/30333", "/ip4/10.16.0.73/tcp/30333"], protocols: ["/b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe/block-announces/1", "/ksmcc3/block-announces/1", "/b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe/transactions/1", "/ksmcc3/transactions/1", "/b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe/grandpa/1", "/paritytech/grandpa/1", "/b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe/validation/1", "/polkadot/validation/1", "/b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe/collation/1", "/polkadot/collation/1", "/ipfs/ping/1.0.0", "/ipfs/id/1.0.0", "/ipfs/id/push/1.0.0", "/b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe/kad", "/ksmcc3/kad", "/b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe/light/2", "/ksmcc3/light/2", "/b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe/state/2", "/ksmcc3/state/2", "/b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe/send_dispute/1", "/polkadot/send_dispute/1", "/b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe/req_pov/1", "/polkadot/req_pov/1", "/b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe/sync/warp", "/ksmcc3/sync/warp", "/b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe/req_available_data/1", "/polkadot/req_available_data/1", "/b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe/req_collation/1", "/polkadot/req_collation/1", "/b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe/sync/2", "/ksmcc3/sync/2", "/b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe/req_chunk/1", "/polkadot/req_chunk/1", "/b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe/req_statement/1", "/polkadot/req_statement/1"], observed_addr: "/ip4/127.0.0.1/tcp/41890/ws" } }
...
```

ping a vara node

```
$ p2ping /dns4/vara-connect-1.vara-network.io/tcp/30333/ws
Local peer id: PeerId("12D3KooWGsfAuVaaAAWogPZXbmqtutq36PUEPvu1ESC7Dj4ZU47i")!
Dialed /dns4/vara-connect-1.vara-network.io/tcp/30333/ws
Listening on "/ip4/127.0.0.1/tcp/46347/ws"
Listening on "/ip4/172.17.0.10/tcp/46347/ws"
Unhandled: ConnectionEstablished { peer_id: PeerId("12D3KooWLDpZ5sWtSmZtiHXBstoQVwMAZ5yRWpDUBdjQHV7vBLHy"), endpoint: Dialer { address: "/dns4/vara-connect-1.vara-network.io/tcp/30333/ws", role_override: Dialer }, num_established: 1, concurrent_dial_errors: Some([]), established_in: 181.385946ms }
BehaviourEvent: Event { peer: PeerId("12D3KooWLDpZ5sWtSmZtiHXBstoQVwMAZ5yRWpDUBdjQHV7vBLHy"), result: Ok(Pong) }
BehaviourEvent: Event { peer: PeerId("12D3KooWLDpZ5sWtSmZtiHXBstoQVwMAZ5yRWpDUBdjQHV7vBLHy"), result: Ok(Ping { rtt: 21.969799ms }) }
BehaviourEvent: Sent { peer_id: PeerId("12D3KooWLDpZ5sWtSmZtiHXBstoQVwMAZ5yRWpDUBdjQHV7vBLHy") }
BehaviourEvent: Received { peer_id: PeerId("12D3KooWLDpZ5sWtSmZtiHXBstoQVwMAZ5yRWpDUBdjQHV7vBLHy"), info: Info { public_key: Ed25519(PublicKey(compressed): 9a97fcad3326b27b51c898b32478dad67952820b335e0f781c0c63083e03b24), protocol_version: "/substrate/1.0", agent_version: "Gear Node/v0.1.4-e2dcdb474e7 (vara-connect-1)", listen_addrs: ["/ip4/54.219.136.79/tcp/30333/ws", "/ip4/172.31.9.64/tcp/30333/ws", "/dns4/vara-connect-1.vara-network.io/tcp/30333/ws", "/ip4/127.0.0.1/tcp/30333/ws", "/ip4/172.23.0.2/tcp/30333/ws"], protocols: ["/fe1b4c55fd4d668101126434206571a7838a8b6b93a6d1b95d607e78e6c53763/block-announces/1", "/vara/block-announces/1", "/fe1b4c55fd4d668101126434206571a7838a8b6b93a6d1b95d607e78e6c53763/transactions/1", "/vara/transactions/1", "/fe1b4c55fd4d668101126434206571a7838a8b6b93a6d1b95d607e78e6c53763/grandpa/1", "/paritytech/grandpa/1", "/ipfs/ping/1.0.0", "/ipfs/id/1.0.0", "/ipfs/id/push/1.0.0", "/fe1b4c55fd4d668101126434206571a7838a8b6b93a6d1b95d607e78e6c53763/kad", "/vara/kad", "/fe1b4c55fd4d668101126434206571a7838a8b6b93a6d1b95d607e78e6c53763/sync/2", "/vara/sync/2", "/fe1b4c55fd4d668101126434206571a7838a8b6b93a6d1b95d607e78e6c53763/sync/warp", "/vara/sync/warp", "/fe1b4c55fd4d668101126434206571a7838a8b6b93a6d1b95d607e78e6c53763/state/2", "/vara/state/2", "/fe1b4c55fd4d668101126434206571a7838a8b6b93a6d1b95d607e78e6c53763/light/2", "/vara/light/2"], observed_addr: "/ip4/34.168.120.6/tcp/44442/ws" } }
...
```

ping a gossamer/westend node

```
$ p2ping /ip4/127.0.0.1/tcp/7001
Local peer id: PeerId("12D3KooWSDch7HgzwyYMZx1KKDMs7WrxJPHjVSPB7VkFEztti8FZ")!
Dialed /ip4/127.0.0.1/tcp/7001
Listening on "/ip4/127.0.0.1/tcp/46565"
Listening on "/ip4/172.17.0.10/tcp/46565"
Unhandled: ConnectionEstablished { peer_id: PeerId("12D3KooWEkYjHhH4VSrL8poAj1rXP3ELTcRvSiKC9a1TWefbhvCH"), endpoint: Dialer { address: "/ip4/127.0.0.1/tcp/7001", role_override: Dialer }, num_established: 1, concurrent_dial_errors: Some([]), established_in: 2.555127ms }
BehaviourEvent: Event { peer: PeerId("12D3KooWEkYjHhH4VSrL8poAj1rXP3ELTcRvSiKC9a1TWefbhvCH"), result: Ok(Ping { rtt: 329.651µs }) }
BehaviourEvent: Sent { peer_id: PeerId("12D3KooWEkYjHhH4VSrL8poAj1rXP3ELTcRvSiKC9a1TWefbhvCH") }
BehaviourEvent: Received { peer_id: PeerId("12D3KooWEkYjHhH4VSrL8poAj1rXP3ELTcRvSiKC9a1TWefbhvCH"), info: Info { public_key: Ed25519(PublicKey(compressed): 4950d895b317a62b72170eb9176113f7e94671626c6c6f79c51b44aeea8e2), protocol_version: "ipfs/0.1.0", agent_version: "github.com/ChainSafe/gossamer@9688f6c25", listen_addrs: ["/ip4/127.0.0.1/tcp/7001", "/ip4/34.168.120.6/tcp/7001", "/ip4/34.168.120.6/tcp/7001"], protocols: ["/ipfs/ping/1.0.0", "/ipfs/id/1.0.0", "/ipfs/id/push/1.0.0", "/e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e/grandpa/1", "/wnd2/sync/2", "/wnd2/light/2", "/wnd2/block-announces/1", "/wnd2/transactions/1", "/wnd2/kad"], observed_addr: "/ip4/127.0.0.1/tcp/38352" } }
...
```

ping an ipfs node

```
$ p2ping /dnsaddr/bootstrap.libp2p.io
Local peer id: PeerId("12D3KooWQM6NQ9FEZ7YJUTxbbxfMB13ujgvTjW1LVd44n4juZLNX")!
Dialed /dnsaddr/bootstrap.libp2p.io
Listening on "/ip4/127.0.0.1/tcp/34381"
Listening on "/ip4/172.17.0.10/tcp/34381"
Unhandled: ConnectionEstablished { peer_id: PeerId("QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN"), endpoint: Dialer { address: "/dnsaddr/bootstrap.libp2p.io", role_override: Dialer }, num_established: 1, concurrent_dial_errors: Some([]), established_in: 204.634288ms }
BehaviourEvent: Sent { peer_id: PeerId("QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN") }
BehaviourEvent: Event { peer: PeerId("QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN"), result: Ok(Ping { rtt: 17.9902ms }) }
BehaviourEvent: Received { peer_id: PeerId("QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN"), info: Info { public_key: Rsa(PublicKey(PKCS1): 30821a282110d7ab1ee4135d7cecd99635b146155d24f5c8dddfa5d9bc513a92ff37ad797a6135f149c8124c6e2e3fff416d891eaf733f554d5f3bda8c6ac1f92c7ece4fb5abe72ba413ecf298080c1dc7e6b422b385a577f4b191da528c95111c7fed7baac2aaff729b40c248b9eedf5d7691e4bd31a4f79e8312a236c1e902254143e254f9ded1d087254082812b267aaeb699e5f9337ecd86853b5367bd765cc3bf67b8c3cdedd3e799aa23a26fecdf1cc3ee2ae87df33a83ff862f17f931996a3e7811ce044f6db71ec6ecc65306f6051fefd5cdaa65b229a3b8d535c234df8f5ac1237a045c26da5b414c85357962576dba787fa49d962ca446f323101), protocol_version: "ipfs/0.1.0", agent_version: "kubo/0.19.0/1963219", listen_addrs: ["/ip4/139.178.91.71/udp/4001/quic", "/ip6/2604:1380:45e3:6e00::1/udp/4001/quic", "/ip4/139.178.91.71/tcp/4001", "/ip6/2604:1380:45e3:6e00::1/tcp/4001", "/dns4/sv15.bootstrap.libp2p.io/tcp/443/wss/p2p/QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN", "/dns6/sv15.bootstrap.libp2p.io/tcp/443/wss/p2p/QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN"], protocols: ["/ipfs/ping/1.0.0", "/libp2p/circuit/relay/0.2.0/stop", "/ipfs/kad/1.0.0", "/ipfs/lan/kad/1.0.0", "/libp2p/autonat/1.0.0", "/ipfs/id/1.0.0", "/ipfs/id/push/1.0.0", "/ipfs/bitswap/1.2.0", "/ipfs/bitswap/1.1.0", "/ipfs/bitswap/1.0.0", "/ipfs/bitswap", "/x/", "/libp2p/circuit/relay/0.2.0/hop", "/libp2p/dcutr"], observed_addr: "/ip4/34.168.120.6/tcp/43476" } }
...
```

ping a starcoin node

```
$ p2ping /dns4/main3.seed.starcoin.org/tcp/9840
Local peer id: PeerId("12D3KooWC2tk5DzM1Z4V3e1RXJ11py5QEEgPdSqtxZZnotx3puLw")!
Dialed /dns4/main3.seed.starcoin.org/tcp/9840
Listening on "/ip4/127.0.0.1/tcp/46763"
Listening on "/ip4/172.17.0.10/tcp/46763"
Unhandled: ConnectionEstablished { peer_id: PeerId("12D3KooWB9vGtpgqyD2cG4PTEU1SHSuWV6PErMPJFbbi5vYpkj3H"), endpoint: Dialer { address: "/dns4/main3.seed.starcoin.org/tcp/9840", role_override: Dialer }, num_established: 1, concurrent_dial_errors: Some([]), established_in: 395.840971ms }
BehaviourEvent: Event { peer: PeerId("12D3KooWB9vGtpgqyD2cG4PTEU1SHSuWV6PErMPJFbbi5vYpkj3H"), result: Ok(Pong) }
BehaviourEvent: Event { peer: PeerId("12D3KooWB9vGtpgqyD2cG4PTEU1SHSuWV6PErMPJFbbi5vYpkj3H"), result: Ok(Ping { rtt: 89.343772ms }) }
BehaviourEvent: Sent { peer_id: PeerId("12D3KooWB9vGtpgqyD2cG4PTEU1SHSuWV6PErMPJFbbi5vYpkj3H") }
BehaviourEvent: Received { peer_id: PeerId("12D3KooWB9vGtpgqyD2cG4PTEU1SHSuWV6PErMPJFbbi5vYpkj3H"), info: Info { public_key: Ed25519(PublicKey(compressed): 13de653e65863dc138933bf775aa2d98c05de5a7acc5287a601c5a4bb4), protocol_version: "/starcoin/1.0", agent_version: "starcoin/1.13.5 (build:v1.13.5) (nutty-cap-5609)", listen_addrs: ["/dns4/main3.seed.starcoin.org/tcp/9840", "/ip4/192.168.122.163/tcp/9840", "/ip4/192.168.69.212/tcp/9840", "/ip4/192.168.249.155/tcp/9840", "/ip4/192.168.224.230/tcp/9840", "/ip4/192.168.233.251/tcp/9840", "/ip4/192.168.211.151/tcp/9840", "/ip4/127.0.0.1/tcp/9840", "/ip4/192.168.108.198/tcp/9840"], protocols: ["/starcoin/block/1", "/starcoin/txn/1", "/starcoin/announcement/1", "/ipfs/ping/1.0.0", "/ipfs/id/1.0.0", "/ipfs/id/push/1.0.0", "/chain/1/kad", "/starcoin/rpc/get_txns", "/starcoin/rpc/get_state_node_by_node_hash", "/starcoin/rpc/get_block_infos", "/starcoin/rpc/get_accumulator_node_by_node_hash", "/starcoin/rpc/get_txns_with_hash_from_pool", "/starcoin/rpc/get_block_ids", "/starcoin/rpc/ping", "/starcoin/rpc/get_headers_by_hash", "/starcoin/rpc/get_blocks", "/starcoin/rpc/get_txn_infos", "/starcoin/rpc/get_account_state", "/starcoin/rpc/get_headers_by_number", "/starcoin/rpc/get_txns_from_pool", "/starcoin/rpc/get_state_with_table_item_proof", "/starcoin/rpc/get_state_with_proof", "/starcoin/rpc/get_bodies_by_hash"], observed_addr: "/ip4/192.168.99.244/tcp/58470" } }
...
```

ping a waku node

```
$ p2ping /dns4/node-01.do-ams3.go-waku.prod.statusim.net/tcp/30303
Local peer id: PeerId("12D3KooWDzV6JX2HBek34cAXA1H8YxEPmUA9jeMPR7LVrV53VPts")!
Dialed /dns4/node-01.do-ams3.go-waku.prod.statusim.net/tcp/30303
Listening on "/ip4/127.0.0.1/tcp/36903"
Listening on "/ip4/172.17.0.10/tcp/36903"
Unhandled: ConnectionEstablished { peer_id: PeerId("16Uiu2HAkyScd7DiwgMwzfw8CFFhznH3wRzciqEUfjDzn7vyimR8c"), endpoint: Dialer { address: "/dns4/node-01.do-ams3.go-waku.prod.statusim.net/tcp/30303", role_override: Dialer }, num_established: 1, concurrent_dial_errors: Some([]), established_in: 608.588839ms }
BehaviourEvent: Sent { peer_id: PeerId("16Uiu2HAkyScd7DiwgMwzfw8CFFhznH3wRzciqEUfjDzn7vyimR8c") }
BehaviourEvent: Event { peer: PeerId("16Uiu2HAkyScd7DiwgMwzfw8CFFhznH3wRzciqEUfjDzn7vyimR8c"), result: Ok(Ping { rtt: 133.555147ms }) }
BehaviourEvent: Received { peer_id: PeerId("16Uiu2HAkyScd7DiwgMwzfw8CFFhznH3wRzciqEUfjDzn7vyimR8c"), info: Info { public_key: Secp256k1(PublicKey(compressed): 23bac887ca65afab7ed4d2abe2a7e78ab7c2056faae429497f8604c3d37cfc9), protocol_version: "ipfs/0.1.0", agent_version: "go-waku", listen_addrs: ["/dns4/node-01.do-ams3.go-waku.prod.statusim.net/tcp/30303", "/dns4/node-01.do-ams3.go-waku.prod.statusim.net/tcp/443/wss"], protocols: ["/ipfs/ping/1.0.0", "/libp2p/circuit/relay/0.2.0/stop", "/ipfs/id/1.0.0", "/ipfs/id/push/1.0.0", "/meshsub/1.1.0", "/meshsub/1.0.0", "/floodsub/1.0.0", "/vac/waku/relay/2.0.0", "/vac/waku/store/2.0.0-beta4", "/vac/waku/lightpush/2.0.0-beta1", "/vac/waku/filter-subscribe/2.0.0-beta1", "/vac/waku/filter-push/2.0.0-beta1", "/libp2p/dcutr", "/libp2p/autonat/1.0.0", "/libp2p/circuit/relay/0.2.0/hop"], observed_addr: "/ip4/34.168.120.6/tcp/40798" } }
...
```

Note: only TCP/WS/WSS transport is supported

References:
- https://docs.rs/libp2p/latest/libp2p/tutorials/ping/index.html
- https://github.com/libp2p/specs/blob/master/ping/ping.md
- https://github.com/libp2p/specs/tree/master/identify
