let ws;
let sessionToken = sessionStorage.getItem("sessionToken"); // Load session token if exists

function connectWebSocket() {
    ws = new WebSocket("ws://127.0.0.1:8080/ws/login");

    ws.onopen = function () {
        console.log("WebSocket Connection Established");
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
                document.body.innerHTML = data.content;
                history.pushState(null, "", "/projects");
                window.reinitializePage(); // Call the global reinitializePage function
            } else if (data.status === "sandbox_page") {
                document.body.innerHTML = data.content;
                history.pushState(null, "", "/sandbox");
                window.reinitializePage(); // Call the global reinitializePage function
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
        setTimeout(connectWebSocket, 5000);
    };
}

function sendLogin(username, password) {
    const data = JSON.stringify({ action: "login", username: username, password: password });
    ws.send(data);
}

function logout() {
    sessionStorage.removeItem("sessionToken");
    window.location.href = "/login";
}

// Function to navigate to another page
function navigateTo(page) {
    if (ws && sessionToken) {
        ws.send(JSON.stringify({ action: `request_${page}`, token: sessionToken }));
    } else {
        window.location.href = '/login';
    }
}

