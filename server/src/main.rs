use std::thread;
use websocket::sync::Server;
use websocket::OwnedMessage;
use protocol::message::Message;

fn main() {
	let server = Server::bind("localhost:2794").unwrap();

	for request in server.filter_map(Result::ok) {
		// Spawn a new thread for each connection.
		thread::spawn(|| {

			let mut client = request.use_protocol("dungeon_game_protocol").accept().unwrap();

			let ip = client.peer_addr().unwrap();

			println!("Connection from {}", ip);

			// ask the client a username
			let message: OwnedMessage = OwnedMessage::Text(Message::Connect(String::new()).encode());
			client.send_message(&message).unwrap();

			let (mut receiver, mut sender) = client.split().unwrap();

			for message in receiver.incoming_messages() {
				let message = message.unwrap();

				match message {
					OwnedMessage::Close(_) => {
						let message = OwnedMessage::Close(None);
						sender.send_message(&message).unwrap();
						println!("Client {} disconnected", ip);
						return;
					}
					OwnedMessage::Ping(ping) => {
						let message = OwnedMessage::Pong(ping);
						sender.send_message(&message).unwrap();
					}
					OwnedMessage::Text(data) => {
						println!("message from client: {:?}", Message::decode(data));
						sender.send_message(&OwnedMessage::Text(Message::ChatMessage(String::from("SERVER"), String::from("you"), String::from("this is a chat message")).encode())).unwrap();
					},
					_ => {
						println!("unknwon message format");
					}
				}
			}
		});
	}
}