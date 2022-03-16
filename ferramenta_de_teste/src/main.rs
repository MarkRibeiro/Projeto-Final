extern crate core;

use tungstenite::{connect, Message};
use url::Url;
use rand::Rng;

fn main() {
    let (mut socket, response) = connect(Url::parse("ws://[::1]:3012").unwrap()).expect("Can't connect");
    let mut aux = 0;
    while(aux<50) {
        let mut rng = rand::thread_rng();
        let movimento = rng.gen_range(0..=3);
        aux += 1;
        let mut msg="oi";
        if movimento == 0 {
            msg = "atualiza;cima";
        }
        if movimento == 1 {
            msg = "atualiza;baixo";
        }
        if movimento == 2 {
            msg = "atualiza;esquerda";
        }
        if movimento == 3 {
            msg = "atualiza;direita";
        }
        //receber resposta do servidor

        socket.write_message(Message::Text(msg.into())).unwrap();
    }
}
