use futures::prelude::*;
use libp2p::{
    swarm::{SwarmBuilder, SwarmEvent},
    identity, mdns, noise::{Keypair as NoiseKeypair, NoiseAuthenticated, X25519Spec, NoiseConfig}, 
    tcp::TokioTcpConfig, yamux, PeerId, Transport, Multiaddr,
};
use std::error::Error;
use tokio::io::{self, AsyncBufReadExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Generate an identity keypair for the local peer.
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    println!("Local peer id: {:?}", local_peer_id);

    // Create a TCP transport with noise encryption and yamux multiplexing.
    let noise_keys = NoiseKeypair::<X25519Spec>::new().into_authentic(&local_key).unwrap();
    let transport = TokioTcpConfig::new()
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(NoiseAuthenticated::xx(noise_keys).unwrap())
        .multiplex(yamux::YamuxConfig::default())
        .boxed();

    // Set up mDNS for local network discovery.
    let mdns = mdns::Mdns::new(mdns::MdnsConfig::default()).await?;
    let mut swarm = SwarmBuilder::new(transport, mdns, local_peer_id)
        .executor(Box::new(|fut| {
            tokio::spawn(fut);
        }))
        .build();

    // Start chat input handling.
    let mut stdin = io::BufReader::new(io::stdin()).lines();

    loop {
        tokio::select! {
            line = stdin.next_line() => {
                let line = line?.unwrap();
                println!("Sending: {}", line);
                // Handle sending the message to peers (you can add your implementation here).
            }
            event = swarm.next() => {
                match event {
                    Some(SwarmEvent::Behaviour(mdns::MdnsEvent::Discovered(peers))) => {
                        for (peer_id, _addr) in peers {
                            println!("Discovered peer: {:?}", peer_id);
                        }
                    }
                    Some(SwarmEvent::Behaviour(mdns::MdnsEvent::Expired(peers))) => {
                        for (peer_id, _addr) in peers {
                            println!("Expired peer: {:?}", peer_id);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
