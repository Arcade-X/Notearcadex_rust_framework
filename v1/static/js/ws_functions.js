let ws;

function connectWebSocket() {
    ws = new WebSocket("ws://127.0.0.1:8080/ws/login");

    ws.onopen = function () {
        console.log("Connected to WebSocket");
    };

    ws.onmessage = function (event) {
        console.log("Received message from server: ", event.data);
        try {
            let data = JSON.parse(event.data);
            if (data.status === "login_successful") {
                window.location.href = "/projects";
            } else if (data.status === "invalid_credentials") {
                alert("Invalid username or password.");
            } else if (data.status === "user_not_found") {
                alert("User not found.");
            }
        } catch (e) {
            console.error("Error parsing message:", e);
        }
    };

    ws.onclose = function () {
        console.log("WebSocket connection closed. Attempting to reconnect...");
        setTimeout(connectWebSocket, 1000); // Reconnect after 1 second
    };
}

function sendLogin(username, password) {
    const data = JSON.stringify({ action: "login", username: username, password: password });
    console.log("Sending login data: ", data);
    ws.send(data);
}

window.onload = function () {
    connectWebSocket();
};