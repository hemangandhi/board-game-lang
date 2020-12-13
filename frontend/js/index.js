// Yay, no external deps!!!
const socket = new WebSocket("ws://localhost:8080");

socket.addEventListener('message', function(e) {
    let new_item = document.createElement('li');
    new_item.innerText = JSON.stringify(e.data);
    document.getElementById('inbox-list').appendChild(new_item);
});

document.getElementById('send-btn').addEventListener('click', function(e) {
    let x = parseInt(document.getElementById('x-input').value);
    let y = parseInt(document.getElementById('y-input').value);
    socket.send(JSON.stringify({ x: x, y: y}));
});
