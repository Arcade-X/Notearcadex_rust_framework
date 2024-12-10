let ws;
let sessionToken = sessionStorage.getItem("sessionToken"); // Load session token if exists
let isWebSocketReady = false;

export function connectWebSocket() {
    ws = new WebSocket("ws://127.0.0.1:8080/ws/login");

    ws.onopen = function () {
        console.log("WebSocket Connection Established");
        isWebSocketReady = true;

        if (sessionToken) {
            console.log("Token exists, verifying:", sessionToken);
            ws.send(JSON.stringify({ action: "verify_token", token: sessionToken }));
        }
    };

    ws.onmessage = function (event) {
        try {
            let data = JSON.parse(event.data);
            console.log("Received WebSocket message:", data);

            if (data.status === "login_successful") {
                sessionToken = data.token;
                console.log("Login successful, token received:", sessionToken);
                sessionStorage.setItem("sessionToken", sessionToken);
                ws.send(JSON.stringify({ action: "request_projects", token: sessionToken }));
            } else if (data.status === "token_valid") {
                console.log("Token validated:", sessionToken);
                ws.send(JSON.stringify({ action: "request_projects", token: sessionToken }));
            } else if (data.status === "projects_page") {
                console.log("Projects page loaded");
                document.body.innerHTML = data.content;
                history.pushState(null, "", "/projects");
                if (typeof window.reinitializePage === 'function') {
                    window.reinitializePage(); // Call reinitializePage function if it exists
                }
            } else if (data.status === "sandbox_page") {
                console.log("Sandbox page loaded");
                document.body.innerHTML = data.content;
                history.pushState(null, "", "/sandbox");

                // Call the function to initialize Three.js now that the page content is loaded
                if (typeof window.reinitializePage === 'function') {
                    window.reinitializePage();
                }
            } else if (data.status === "invalid_credentials") {
                alert("Invalid username or password.");
            } else if (data.status === "user_not_found") {
                alert("User not found.");
            } else if (data.status === "unauthorized") {
                console.log("Unauthorized access, redirecting to login");
                window.location.href = "/login";
            }
        } catch (e) {
            console.error("Error parsing message:", e);
        }
    };

    ws.onclose = function () {
        console.log("WebSocket connection closed. Reconnecting...");
        isWebSocketReady = false;
        setTimeout(connectWebSocket, 5000);
    };
}

// Send login request
export function sendLogin(username, password) {
    if (!isWebSocketReady) {
        console.error("WebSocket is not open. Cannot send login request.");
        return;
    }

    console.log("Sending login request for user:", username);
    const data = JSON.stringify({ action: "login", username: username, password: password });
    ws.send(data);
}

// Handle page navigation via WebSocket
export function navigateTo(page) {
    if (ws && sessionToken) {
        ws.send(JSON.stringify({ action: `request_${page}`, token: sessionToken }));
    } else {
        console.error("WebSocket not ready or session token missing.");
    }
}

// Handle logout
export function logout() {
    sessionStorage.removeItem("sessionToken");
    window.location.href = "/login";
}

// Make these functions globally available
window.navigateTo = navigateTo;
window.logout = logout;