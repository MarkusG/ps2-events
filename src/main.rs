use websocket::{ClientBuilder, Message};

fn main() {
    let url = "wss://push.planetside2.com/streaming?environment=ps2&service-id=s:SpandexBot";
    // connect
    let mut client = ClientBuilder::new(url)
        .unwrap()
        .connect(None)
        .unwrap();

    // construct and send a message to subscribe to death events for all
    // characters on Connery (world 1)
    let message = Message::text("{ \"service\":\"event\", \"action\":\"subscribe\", \"characters\":[\"all\"], \"worlds\":[\"1\"], \"eventNames\":[\"Death\"] }");
    client.send_message(&message).unwrap();

    // listen for messages and print them
    loop {
        let m = client.recv_message().unwrap();
        println!("Recv: {:?}", m);
    }
}
