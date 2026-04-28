const socket = new WebSocket("ws://127.0.0.1:8080");

socket.onopen = () => {
  console.log("Conectado al servidor");
  socket.send("Hola desde React");
};

socket.onmessage = (event) => {
  console.log("Respuesta del servidor:", event.data);
};

socket.onclose = () => {
  console.log("Conexión cerrada");
};

socket.onerror = (error) => {
  console.error("Error:", error);
};