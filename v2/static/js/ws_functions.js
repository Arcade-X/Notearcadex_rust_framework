let ws;
let sessionToken = sessionStorage.getItem("sessionToken"); // Load session token if exists

function connectWebSocket() {
    ws = new WebSocket("ws://127.0.0.1:8080/ws/login");

    ws.onopen = function () {
        console.log("Connected to WebSocket");
        if (sessionToken) {
            // Verify token if already logged in
            ws.send(JSON.stringify({ action: "verify_token", token: sessionToken }));
        }
    };

    ws.onmessage = function (event) {
        try {
            let data = JSON.parse(event.data);
            if (data.status === "login_successful") {
                sessionToken = data.token; // Store the session token
                sessionStorage.setItem("sessionToken", sessionToken); // Persist the session
                ws.send(JSON.stringify({ action: "request_projects", token: sessionToken }));
            } else if (data.status === "token_valid") {
                ws.send(JSON.stringify({ action: "request_projects", token: sessionToken }));
            } else if (data.status === "projects_page") {
                document.body.innerHTML = data.content; // This replaces the entire body content
                history.pushState(null, "", "/projects"); // Update the URL to /projects
            } else if (data.status === "invalid_credentials") {
                alert("Invalid username or password.");
            } else if (data.status === "user_not_found") {
                alert("User not found.");
            } else if (data.status === "unauthorized") {
                window.location.href = "/login";
            }
        } catch (e) {
            console.error("Error parsing message:", e);
        }
    };

    ws.onclose = function () {
        setTimeout(connectWebSocket, 5000); // Reconnect after 5 seconds
    };
}

function sendLogin(username, password) {
    sessionToken = null; // Clear session token before sending login request
    sessionStorage.removeItem("sessionToken"); // Clear session token from storage
    const data = JSON.stringify({ action: "login", username: username, password: password });
    ws.send(data);
}

function logout() {
    sessionToken = null;
    sessionStorage.removeItem("sessionToken");
    ws.close(); // Close WebSocket connection on logout
    window.location.href = "/login";
}

window.onload = function () {
    connectWebSocket();
};