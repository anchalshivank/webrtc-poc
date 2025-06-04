use mainline::Dht;

fn main() {

    let dht = Dht::server().unwrap();

    dht.bootstrapped();
    let info = dht.info();

    println!("{:?}", info);


}
