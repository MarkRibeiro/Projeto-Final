extern crate tungstenite;
extern crate tokio;
//use tokio::net::{TcpListener, TcpStream};
//use futures_util::{future, SinkExt};
use std::net::{TcpListener, TcpStream};
//use std::thread;
use std::sync::{Arc, Mutex};
//use tokio_tungstenite::{accept_async, WebSocketStream};
//use tokio_tungstenite::tungstenite::Message;
//use tokio_tungstenite::tungstenite::accept;
use tungstenite::{accept, Message, WebSocket};

struct Ponto {
    x:u32,
    y:u32
}

struct Jogador{
    //nome: String,
    cor: String,
    posi: Ponto,
    pontuacao: u32
}

struct Estado {
    jogadores: Vec<Jogador>,
    mapa: Vec<Vec<String>>
}

#[tokio::main]
async fn main() { //estabelece o servidor e recebe mensagens do cliente
    let server = TcpListener::bind("[::1]:3012").unwrap(); //ouvindo a porta 3012
    let mut estado = Estado{ jogadores: vec![], mapa: vec![] };
    estado.mapa = cria_matriz();
    let current_state = Arc::new(Mutex::new(estado));
    //let mut connection_list = Arc::new(Mutex::new(Vec::new()));

    while let Ok((stream, _addr)) = server.accept() { //avalia solicitaçoes de conexão
        let websocket = accept(stream).unwrap(); //aceita conexão
        let websocket =  Arc::new(Mutex::new(websocket));
        /*let (mut outgoing, mut incoming) = websocket.split();
        let copy_connection_list = connection_list.clone();
        {
            let mut connection_list = connection_list.lock().unwrap();
            connection_list.push(outgoing);
        }*/
        tokio::spawn(handle_connection(websocket, current_state.clone()));
    }
}

fn cria_matriz() -> Vec<Vec<String>> {
    let mut matriz = vec![];
    let mut vetor = vec![];
    for _ in 0..10 {
        vetor.push("white".to_string());
    }
    for _ in 0..10 {
        matriz.push(vetor.clone());
    }
    return matriz;
}

async fn handle_connection(websocket: Arc<Mutex<WebSocket<TcpStream>>>, current_state: Arc<Mutex<Estado>>){
    loop {
        let msg = websocket.lock().unwrap().read_message().unwrap();
        let copy_current_state = current_state.clone();
        /*{
            let mut state = current_state.lock().unwrap();
            *state = msg.clone();
        }*/
        _process_message(websocket.clone(), msg, copy_current_state).await;
    }
}

async fn _process_message(websocket: Arc<Mutex<WebSocket<TcpStream>>>, message:Message,
                          current_state: Arc<Mutex<Estado>>){ //responde o cliente
    let msg = message.to_string();
    let info:Vec<&str> = msg.split(";").collect();
    println!("{:?}", info);
    //let msg;
    /*{
        let state = current_state.lock().unwrap();
        msg = (*state).clone();
    }*/
    let mut websocket = websocket.lock().unwrap();
    //let _ = (*websocket).write_message(msg.clone());
    let mut state = current_state.lock().unwrap();
    if info[0]=="fst" {
        let jogador = Jogador{
            //nome: "mark".to_string(),
            cor: info[1].to_string(),
            posi: Ponto { x: 0, y: 0 },
            pontuacao: 0
        };
        state.jogadores.push(jogador);
    }
    if info[0]=="atu" {
        if info[1] == "cima" {
            state.jogadores[0].posi.y -= 50;
        }
        if info[1] == "baixo" {
            state.jogadores[0].posi.y += 50;
        }
        if info[1] == "esquerda" {
            state.jogadores[0].posi.x -= 50;
        }
        if info[1] == "direita" {
            state.jogadores[0].posi.x += 50;
        }
    }
    else if info[0]=="pinta" {
        let novo_x = info[1].parse::<usize>();
        let novo_y = info[2].parse::<usize>();
        if let Ok(x) = novo_x{
            if let Ok(y) = novo_y {
                //println!("x:{}, y:{}", X/50, Y/50);
                state.mapa[x/50][y/50] = "green".to_string();
            }
        }
    }
    for mut jogador in &mut state.jogadores{
        jogador.pontuacao = 0;
    }

    for coluna in state.mapa.clone(){
        for cor in coluna{
            for mut jogador in &mut state.jogadores{
                if jogador.cor == *cor {
                    jogador.pontuacao += 1;
                }
            }
        }
    }

    let ret = format!("{{\"x\":{},\"y\":{}, \"mapa\":{:?}, \"pontuacao\":{}}}",
                      state.jogadores[0].posi.x, state.jogadores[0].posi.y, state.mapa,
                      state.jogadores[0].pontuacao);
    let _ = (*websocket).write_message(Message::Text(ret));

}