use tokio::net::UdpSocket;
#[tokio::main]
async fn main(){

    let peer_addr = "103.95.82.235:6991";
let signaling_socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
match signaling_socket.connect(peer_addr).await{
    Ok(e) => print!("done"),
    Err(err) => println!("{:?}", err),
}

}