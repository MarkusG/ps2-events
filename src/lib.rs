mod messages;

use crate::messages::SocketMessage;

pub fn parse_socket_message(payload: &str) -> Result<SocketMessage, ()> {
    match serde_json::from_str::<SocketMessage>(payload) {
        Ok(event) => Ok(event),
        Err(e) => {
            println!("{:?}", e);
            return Err(());
        }
    }
}
