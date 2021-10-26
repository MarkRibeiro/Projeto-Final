var aWebSocket = new WebSocket("ws://[fd7a:115c:a1e0:ab12:4843:cd96:6266:994b]:3012");
aWebSocket.onopen = function(event) {
    console.log("WebSocket is open now.");
};
let aux = 0
document.getElementById("button1").onclick = function(){
    let cor = ['blue', 'black', 'yellow', 'green', 'red', 'orange', 'gray', 'purple', 'magenta', 'pink', 'white']
    aWebSocket.send(cor[aux%11]);
    aux += 1;
}
aWebSocket.onmessage = function(event) {
    document.body.style.backgroundColor = event.data;
};