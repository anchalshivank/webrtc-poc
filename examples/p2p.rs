// Cargo.toml dependencies:
// [dependencies]
// webrtc = "0.7"
// tokio = { version = "1.0", features = ["full"] }
// mainline = "2.0"
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"

use webrtc::{api::APIBuilder, peer_connection::sdp::session_description::RTCSessionDescription};
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::ice_transport::ice_server::RTCIceServer;
use webrtc::data_channel::RTCDataChannel;
use std::sync::Arc;
use tokio::net::{TcpStream, TcpListener};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
enum SignalingMessage {
    Offer { sdp: String },
    Answer { sdp: String },
    IceCandidate { candidate: String },
}

// Simple signaling server (run separately)
async fn signaling_server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Signaling server listening on :8080");
    
    // Store connected clients
    let mut clients = std::collections::HashMap::new();
    
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Client connected: {}", addr);
        // Handle client in separate task
        // Relay messages between clients
    }
}

// WebRTC Peer
struct WebRTCPeer {
    peer_connection: Arc<webrtc::peer_connection::RTCPeerConnection>,
    data_channel: Option<Arc<RTCDataChannel>>,
}

impl WebRTCPeer {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Configure WebRTC with STUN servers
        let config = RTCConfiguration {
            ice_servers: vec![
                RTCIceServer {
                    urls: vec![
                        "stun:stun.l.google.com:19302".to_owned(),
                        "stun:stun1.l.google.com:19302".to_owned(),
                    ],
                    ..Default::default()
                }
            ],
            ..Default::default()
        };

        let api = APIBuilder::new().build();
        let peer_connection = Arc::new(api.new_peer_connection(config).await?);

        Ok(Self {
            peer_connection,
            data_channel: None,
        })
    }

    // Node A - Create offer (caller)
    async fn create_offer(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Create data channel for sending data
        let data_channel = self.peer_connection.create_data_channel("data", None).await?;
        self.data_channel = Some(data_channel.clone());

        // Set up data channel handlers
        data_channel.on_open(Box::new(move || {
            Box::pin(async move {
                println!("Data channel opened!");
            })
        }));

        data_channel.on_message(Box::new(move |msg| {
            Box::pin(async move {
                println!("Received: {}", String::from_utf8_lossy(&msg.data));
            })
        }));

        // Create offer
        let offer = self.peer_connection.create_offer(None).await?;
        self.peer_connection.set_local_description(offer.clone()).await?;

        // Send offer via signaling server
        self.send_signaling_message(SignalingMessage::Offer {
            sdp: offer.sdp,
        }).await?;

        Ok(())
    }

    // Node B - Create answer (callee)
    async fn create_answer(&mut self, offer_sdp: String) -> Result<(), Box<dyn std::error::Error>> {
        // Set remote description (the offer)
        let offer = RTCSessionDescription {
            sdp_type: webrtc::peer_connection::sdp::sdp_type::RTCSdpType::Offer,
            sdp: offer_sdp,
        };
        self.peer_connection.set_remote_description(offer).await?;

        // Handle incoming data channel
        self.peer_connection.on_data_channel(Box::new(move |data_channel| {
            Box::pin(async move {
                println!("Data channel received!");
                
                data_channel.on_message(Box::new(move |msg| {
                    Box::pin(async move {
                        println!("Received: {}", String::from_utf8_lossy(&msg.data));
                    })
                }));
            })
        }));

        // Create answer
        let answer = self.peer_connection.create_answer(None).await?;
        self.peer_connection.set_local_description(answer.clone()).await?;

        // Send answer via signaling server
        self.send_signaling_message(SignalingMessage::Answer {
            sdp: answer.sdp,
        }).await?;

        Ok(())
    }

    async fn send_signaling_message(&self, msg: SignalingMessage) -> Result<(), Box<dyn std::error::Error>> {
        // Connect to signaling server and send message
        let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
        let json = serde_json::to_string(&msg)?;
        // Send to signaling server
        // Implementation depends on your signaling protocol
        Ok(())
    }

    async fn send_data(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(data_channel) = &self.data_channel {
            data_channel.send(&webrtc::data_channel::data_channel_message::DataChannelMessage {
                data: data.to_vec().into(),
                ..Default::default()
            }).await?;
        }
        Ok(())
    }
}

// Main application combining DHT discovery + WebRTC
async fn node_a() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Announce via DHT (you already have this)
    println!("Step 1: Announcing on DHT...");
    // Your existing DHT announce code here
    
    // Step 2: Create WebRTC peer and offer
    println!("Step 2: Creating WebRTC offer...");
    let mut peer = WebRTCPeer::new().await?;
    peer.create_offer().await?;
    
    // Step 3: Wait for connection, then send data
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    peer.send_data(b"Hello from Node A!").await?;
    
    Ok(())
}

async fn node_b() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Discover via DHT (you already have this)
    println!("Step 1: Discovering peers on DHT...");
    // Your existing DHT discovery code here
    
    // Step 2: Connect to signaling server and wait for offer
    println!("Step 2: Waiting for WebRTC offer...");
    // Receive offer from signaling server
    let offer_sdp = "..."; // Received from signaling
    
    let mut peer = WebRTCPeer::new().await?;
    peer.create_answer(offer_sdp.to_string()).await?;
    
    // Step 3: Connection established, can send/receive data
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    peer.send_data(b"Hello from Node B!").await?;
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Run as Node A or Node B based on command line args
    let args: Vec<String> = std::env::args().collect();
    
    match args.get(1).map(|s| s.as_str()) {
        Some("server") => signaling_server().await,
        Some("node-a") => node_a().await,
        Some("node-b") => node_b().await,
        _ => {
            println!("Usage: cargo run [server|node-a|node-b]");
            Ok(())
        }
    }
}