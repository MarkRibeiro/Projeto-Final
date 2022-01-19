extern crate tokio_tungstenite;

use tokio::net::{TcpListener, TcpStream};
use futures_util::{future, SinkExt};
use std::{thread};
use std::os::raw::c_uchar;
use std::sync::{Arc, Mutex};
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
    let mut current_state = Arc::new(Mutex::new(Message::Text("White".to_string())));
    let mut connection_list = Arc::new(Mutex::new(Vec::new()));

    while let Ok((stream, addr)) = server.accept().await { //avalia solicitaçoes de conexão
        let mut websocket = accept_async(stream).await.unwrap(); //aceita conexão
        let (mut outgoing, mut incoming) = websocket.split();
        let copy_connection_list = connection_list.clone();
        {
            let mut connection_list = connection_list.lock().unwrap();
            connection_list.push(outgoing);
        }
        tokio::spawn(handle_connection(incoming, copy_connection_list, current_state.clone()));
    }
}

async fn handle_connection(mut incoming: SplitStream<WebSocketStream<TcpStream>>, connection_list: Arc<Mutex<Vec<SplitSink<WebSocketStream<TcpStream>, Message>>>>, current_state: Arc<Mutex<Message>>){
    loop {
        let msg = incoming.next().await.unwrap().unwrap();
        let copy_current_state = current_state.clone();
        {
            let mut state = current_state.lock().unwrap();
            *state = msg.clone();
        }
        _process_message( connection_list.clone(), msg, copy_current_state).await;
    }
    //
}

async fn _process_message(websocket: Arc<Mutex<Vec<SplitSink<WebSocketStream<TcpStream>, Message>>>>, message:Message, current_state: Arc<Mutex<Message>>){ //responde o cliente
    let msg;
    {
        let state = current_state.lock().unwrap();
        msg = (*state).clone();
    }
    let mut websocket = websocket.lock().unwrap();
    for mut el in &mut *websocket {
        let _ = (*el).send(msg.clone()).await;
        let _ = (*el).flush().await;
    }
}