mod messages;

use websocket::{ClientBuilder, Message, OwnedMessage};

use ps2_events::parse_socket_message;

fn main() {
    let url = "wss://push.planetside2.com/streaming?environment=ps2&service-id=s:SpandexBot";
    // connect
    let mut client = ClientBuilder::new(url)
        .unwrap()
        .connect(None)
        .unwrap();

    // construct and send a message to subscribe to death events for all
    // characters on Connery (world 1)
    let message = Message::text(
        "{ 
            \"service\":\"event\",
            \"action\":\"subscribe\",
            \"characters\":[\"all\"],
            \"worlds\":[\"all\"],
            \"eventNames\":[\"\"]
         }");
    client.send_message(&message).unwrap();

    loop {
        if let OwnedMessage::Text(m) = client.recv_message().unwrap() {
            println!("{}", m);
            if let Ok(message) = parse_socket_message(&m) {
                println!("{:#?}", message);
                println!();
            }
        }
    }
}
