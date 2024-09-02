let ws;

function connectWebSocket() {
    ws = new WebSocket("ws://127.0.0.1:8080/ws/login");

    ws.onopen = function () {
        console.log("Connected to WebSocket");
    };

    ws.onmessage = function (event) {
        console.log("Received message from server: ", event.data);  // Log received message
        try {
            let data = JSON.parse(event.data);
            if (data.status === "code_sent") {
                document.getElementById("code-input").style.display = "block";
            } else if (data.status === "login_successful") {
                window.location.href = "/projects";
            } else if (data.status === "invalid_code") {
                alert("Invalid code, please try again.");
            }
        } catch (e) {
            console.error("Error parsing message:", e);
        }
    };

    ws.onclose = function () {
        console.log("WebSocket connection closed");
    };
}

function sendLogin(username) {
    const data = JSON.stringify({ action: "login", username: username });
    console.log("Sending login data: ", data);  // Log the data being sent
    ws.send(data);
}

function verifyCode(username, code) {
    ws.send(JSON.stringify({ action: "verify_code", username: username, code: code }));
}

window.onload = function () {
    connectWebSocket();
};