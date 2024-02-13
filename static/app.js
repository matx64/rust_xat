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

const handleMessage = {
  normal: (message) => {
    messagesList.appendChild(message.createNormalElement());
  },
  join: (message) => {
    usersList.appendChild(message.createUserElement());
    messagesList.appendChild(message.createSystemElement());
  },
  left: (message) => {
    document.getElementById("user-" + message.username).remove();
    messagesList.appendChild(message.createSystemElement());
  },
};

connectForm.addEventListener("submit", function (e) {
  e.preventDefault();

  const username = usernameInput.value;
  const room_id = roomInput.value;

  setupWebSocket(room_id, username);
  setupMessageForm(room_id, username);

  roomDisplay.innerHTML = "Room " + roomInput.value;
  connectForm.classList.add("d-none");
  conversationDiv.classList.remove("d-none");
  messageInput.focus();
});

function setupWebSocket(room_id, username) {
  window.ws = new WebSocket(
    "ws://" +
      window.location.host +
      "/ws?room_id=" +
      room_id +
      "&username=" +
      username
  );

  window.ws.addEventListener("message", function (e) {
    const message = ChatMessage.parse(e.data);
    message.sent_at = new Date(message.sent_at * 1000).toLocaleTimeString();

    handleMessage[message.kind](message);
    chatScroll.scrollIntoView({ behavior: "smooth", block: "end" });
  });
}

function setupMessageForm(room_id, username) {
  messageForm.addEventListener("submit", function (e) {
    e.preventDefault();

    const message = new ChatMessage({
      room_id: room_id,
      username: username,
      text: messageInput.value,
      kind: "normal",
      sent_at: Math.floor(Date.now() / 1000),
    });

    window.ws.send(message.to_string());

    messageInput.value = "";
  });
}
