const connectForm = document.getElementById("connect-form");
const messageForm = document.getElementById("message-form");
const quitForm = document.getElementById("quit-form");
const usernameInput = document.getElementById("username-input");
const roomInput = document.getElementById("room-input");
const messageInput = document.getElementById("message-input");
const conversationDiv = document.getElementById("conversation-div");
const messagesList = document.getElementById("messages-list");
const usersList = document.getElementById("users-list");
const roomDisplay = document.getElementById("room-display");
const chatScroll = document.getElementById("chat-scroll");

let roomUsers = new Set();

const handleMessage = {
  normal: (message) => {
    if (message.username == window.username) {
      messagesList.innerHTML += `<div class="m-2"><div class="d-flex flex-column text-white rounded p-3 float-end msg-sent">
                <div class="text-break">${message.text}</div> 
                <div class="fs-6 fw-light text-end">${message.sent_at}</div>
            </div></div>`;
    } else {
      messagesList.innerHTML += `<div class="m-2"><div class="d-flex flex-column text-white rounded p-3 float-start msg-received">
                <div class="fw-bold">${message.username}</div> 
                <div class="text-break">${message.text}</div> 
                <div class="fs-6 fw-light text-end">${message.sent_at}</div>
            </div></div>`;
    }
  },
  join: (message) => {
    roomUsers.add(message.username);
    usersList.innerHTML += `<div class="text-white m-2 fs-6 fw-bold text-break" id="user-${message.username}">${message.username}</div>`;
    messagesList.innerHTML += `<div class="text-success text-center my-2"><strong>${message.text}</strong> ${message.sent_at}</div>`;
  },
  left: (message) => {
    roomUsers.delete(message.username);
    document.getElementById("user-" + message.username).remove();
    messagesList.innerHTML += `<div class="text-danger text-center my-2"><strong>${message.text}</strong> ${message.sent_at}</div>`;
  },
};

connectForm.addEventListener("submit", function (e) {
  e.preventDefault();

  window.username = usernameInput.value;
  window.room_id = roomInput.value;

  setupWebSocket();
  setupMessageForm();

  roomDisplay.innerHTML = "Room " + roomInput.value;
  connectForm.classList.add("d-none");
  conversationDiv.classList.remove("d-none");
  messageInput.focus();
});

function setupWebSocket() {
  window.ws = new WebSocket(
    "ws://" +
      window.location.host +
      "/ws?room_id=" +
      window.room_id +
      "&username=" +
      window.username
  );

  window.ws.addEventListener("message", function (e) {
    const message = ChatMessage.parse(e.data);
    message.sent_at = new Date(message.sent_at * 1000).toLocaleTimeString();

    handleMessage[message.kind](message);
    chatScroll.scrollIntoView({ behavior: "smooth", block: "end" });
  });
}

function setupMessageForm() {
  messageForm.addEventListener("submit", function (e) {
    e.preventDefault();

    const message = new ChatMessage({
      room_id: window.room_id,
      username: window.username,
      text: messageInput.value,
      kind: "normal",
      sent_at: Math.floor(Date.now() / 1000),
    });

    window.ws.send(message.to_string());

    messageInput.value = "";
  });
}
