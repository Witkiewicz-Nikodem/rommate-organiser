function checkSessionCookie() {
    // Sprawdzamy, czy ciasteczko sesji istnieje
    const sessionCookie = document.cookie.split(';').find(cookie => cookie.trim().startsWith('id='));

    if (!sessionCookie) {
        alert('Your session has expired. Redirecting to home...');
        window.location.pathname = "/logged_out/home"
    }
}

// Sprawdzenie co 5 minut (300000 ms)
setInterval(checkSessionCookie, 1_000);