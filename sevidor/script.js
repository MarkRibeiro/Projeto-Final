let aWebSocket = new WebSocket("ws://[::1]:3012");
let canvas = document.getElementById("Canvas");
let contcanvas = canvas.getContext("2d");
let pontuacao = document.getElementById("Pontuacao");

let posi = {x:0, y:0};
let largura = 500;
let altura = 500;
let passo = 50;
let tamJogador = 50;
let espacosOcupados = [];
let cor = "green"

aWebSocket.onopen = function(event) {
    console.log("WebSocket is open now.");
    aWebSocket.send("fst;" + cor + ";" + posi.x + ";" + posi.y);
    contcanvas.beginPath();
    contcanvas.fillStyle = cor;
    contcanvas.rect(posi.x, posi.y, tamJogador, tamJogador);
    contcanvas.fill();
};
/*
let aux = 0
document.getElementById("button1").onclick = function(){
    let cor = ['blue', 'black', 'yellow', 'green', 'red', 'orange', 'gray', 'purple', 'magenta', 'pink', 'white']
    aWebSocket.send(cor[aux%11]);
    aux += 1;
}*/
aWebSocket.onmessage = function(event) {
    contcanvas.fillStyle = "white"
    contcanvas.beginPath();
    contcanvas.rect(0, 0, largura, altura);
    contcanvas.fill();

    let aux = JSON.parse(event.data);
    let mapa = aux.mapa
    posi.x = aux.x
    posi.y = aux.y

    let x = 0
    for(let coluna of mapa){
        let y = 0
        for(let corQuadrado of coluna){
            contcanvas.fillStyle = corQuadrado;
            contcanvas.beginPath();
            contcanvas.rect(x*50, y*50, 50, 50);
            contcanvas.fill();
            y+=1
        }
        x+=1
    }
    contcanvas.fillStyle = cor;
    contcanvas.beginPath();
    contcanvas.rect(posi.x, posi.y, 50, 50);
    contcanvas.fill();

    pontuacao.innerHTML = "Pontuação: " +  aux.pontuacao;

    //console.log(event.data);
    //document.body.style.backgroundColor = event.data;
};

function isUnique(novaPosicao, espacosOcupados) {
    for (let espaco of espacosOcupados){
        if(novaPosicao.x == espaco.x && novaPosicao.y == espaco.y){
            return false
        }
    }
    return true
}

function leTeclado(evento) {
    if(event.key == "ArrowUp" && posi.y-passo >= 0){
        aWebSocket.send("atu;cima");
    }

    if(event.key == "ArrowDown" && posi.y+passo+tamJogador <= altura){
        aWebSocket.send("atu;baixo");
    }

    if(event.key == "ArrowLeft" && posi.x-passo >= 0){
        aWebSocket.send("atu;esquerda");
    }

    if(event.key == "ArrowRight" && posi.x+passo+tamJogador <= largura){
        aWebSocket.send("atu;direita");
    }

    if(event.key == " " && isUnique({x:posi.x, y:posi.y}, espacosOcupados)){
        //espacosOcupados.push({x:posi.x, y:posi.y})
        aWebSocket.send("pinta;" + posi.x + ";" + posi.y);
    }
    /*
    for (let espaco of espacosOcupados){
        contcanvas.fillStyle = cor;
        contcanvas.beginPath();
        contcanvas.rect(espaco.x, espaco.y, 50, 50);
        contcanvas.fill();
    }*/
    aWebSocket.send("atu;" + cor + ";" + posi.x + ";" + posi.y);
}
document.addEventListener("keydown", leTeclado);
