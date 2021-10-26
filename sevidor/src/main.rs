extern crate tungstenite;

use std::net::{TcpListener, TcpStream};
use std::thread;
//use tokio_tungstenite::accept_async;
//use tokio_tungstenite::tungstenite::accept;
use tungstenite::{accept, Message, WebSocket};

fn main() { //estabelece o servidor e recebe mensagens do cliente
    let server = TcpListener::bind("[::]:3012").unwrap(); //ouvindo a porta 3012
    for stream in server.incoming() { //avalia solicitaçoes de conexão
        match &stream {
            Ok(_stream) => { //conexão valida
                println!("Nova Conexão!");
            }

            Err(_e) => { //conexão invalida
                println!("Erro ao estabelecer conexão");
                continue
            }
        }

        let mut websocket = accept(stream.unwrap()).unwrap(); //aceita conexão
        loop {
            let message = websocket.read_message().unwrap();
            _process_message(&mut websocket, message);
        }
    }
}

fn _process_message(websocket:&mut WebSocket<TcpStream>, message:Message){ //responde o cliente
    let _ = websocket.write_message(message);
    let _ = websocket.write_pending();
    
}