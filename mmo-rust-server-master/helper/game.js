var gameContext = {
    canvas : document.createElement("canvas"),
    start : function() {
       this.canvas.width  = 1000;
       this.canvas.height = 500;
       this.context = this.canvas.getContext("2d");
       document.body.insertBefore(this.canvas, document.body.childNodes[0]);
    },
    clear : function() {
       this.context.clearRect(0, 0, this.canvas.width, this.canvas.height);
    }
 }

 var connection = new WebSocket('ws://[fd7a:115c:a1e0:ab12:4843:cd96:6266:994b]:12332');
 connection.onmessage = (e) => {
    let state = JSON.parse(e.data);

    gameContext.clear();
    for (var i = 0; i < state.length; i++) {
       let e = state[i];
       console.log(e.position);
       let ctx = gameContext.context;
       ctx.fillStyle = "red";
       ctx.fillRect(e.position.x,e.position.y,100,100);
    }
 }


 document.addEventListener('keydown', function(event) {
     if (event.keyCode == 37)
        connection.send("left");
     else if (event.keyCode == 39)
        connection.send("right");
     else if (event.keyCode == 38)
        connection.send("up");
     else if (event.keyCode == 40)
        connection.send("down");
 });

 document.body.onload = function(){ 
    gameContext.start();
 }