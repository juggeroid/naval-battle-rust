mod guts;
use serde;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

#[derive(Serialize, Deserialize, Debug)]
enum ServerMessage {
    // This message is sent after the server has started.
    InitializeMessage(guts::Field, guts::Field),
    DisconnectMessage,

    // Turn messages.
    SenderMissMessage,
    SenderHitMessage,
    ReceiverMissMessage(usize, usize),
    ReceiverHitMessage(usize, usize),

    InvalidMessage, // e.g. incorrect format
    // Game state messages.
    GameWonMessage,
    GameLostMessage,
}

enum InvalidMessageType {
    OutOfBounds,
    OutOfOrder,
    AlreadyHit,
    IncorrectFormat,
}

#[derive(Serialize, Deserialize, Debug)]
enum ClientMessage {
    TurnMessage(usize, usize),
}

#[derive(Debug)]
enum InternalServerError {
    FaultyString,
    NetError,
}
struct Server {
    users: Vec<(usize, usize)>,
}

impl Server {
    pub fn read_json(stream: &mut TcpStream) -> Result<ClientMessage, InternalServerError> {
        let mut reader = BufReader::new(stream);
        let mut string = String::new();
        if let Ok(0) | Err(_) = reader.read_line(&mut string) {
            return Err(InternalServerError::NetError);
        }
        let parsed = serde_json::from_str(&string).map_err(|_| InternalServerError::FaultyString);
        parsed
    }

    pub fn handle_connection(mut stream: TcpStream) {
        loop {
            let msg = Server::read_json(&mut stream);
            println!("{:?}", msg);
            match msg {
                Ok(_) | Err(InternalServerError::FaultyString) => (),
                Err(InternalServerError::NetError) => break,
            }
        }
    }

    pub fn create_listener(ip: &'static str) -> TcpListener {
        let listener = TcpListener::bind(ip).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            std::thread::spawn(move || {
                Server::handle_connection(stream);
            });
        }
        todo!()
    }
}

fn main() {
    Server::create_listener("127.0.0.1:80");
}
