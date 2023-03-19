let lobbyIdElement = document.getElementById("lobby-id");

lobbyIdElement.textContent = window.location.pathname;

lobbyIdElement.addEventListener("click", e => {
    navigator.clipboard.writeText(window.location.href);
    lobbyIdElement.classList.add("activated");
});
