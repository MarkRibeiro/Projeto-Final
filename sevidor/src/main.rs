extern crate tokio_tungstenite;

use tokio::net::{TcpListener, TcpStream};
use futures_util::{future, SinkExt};
use std::{thread};
use futures_util::stream::{SplitSink, SplitStream};
use tokio_tungstenite::{accept_async, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;
use futures_util::StreamExt;
use futures_util::TryStreamExt;
//use tokio_tungstenite::tungstenite::accept;
//use tungstenite::{accept, Message, WebSocket};

#[tokio::main]
async fn main() { //estabelece o servidor e recebe mensagens do cliente
    let server = TcpListener::bind("[::1]:3012").await.unwrap(); //ouvindo a porta 3012

    while let Ok((stream, addr)) = server.accept().await { //avalia solicitaçoes de conexão
        let mut websocket = accept_async(stream).await.unwrap(); //aceita conexão
        let (mut outgoing, mut incoming) = websocket.split();
        tokio::spawn(handle_connection(incoming, outgoing));

    }
}

async fn handle_connection(mut incoming: SplitStream<WebSocketStream<TcpStream>>, mut outgoing: SplitSink<WebSocketStream<TcpStream>, Message>){
    loop {
        let msg = incoming.next().await.unwrap().unwrap();
        _process_message(&mut outgoing, msg).await;
    }
}

async fn _process_message(websocket:&mut SplitSink<WebSocketStream<TcpStream>, Message>, message:Message){ //responde o cliente
    let _ = websocket.send(message).await;
    let _ = websocket.flush().await;
}