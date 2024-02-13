class ChatMessage {
  constructor(data) {
    this.room_id = data.room_id;
    this.username = data.username;
    this.text = data.text;
    this.kind = data.kind;
    this.sent_at = data.sent_at;
  }

  /**
   * @param {string} data
   * @returns {ChatMessage}
   */
  static parse(data) {
    let result = {};

    data = data.split(";");
    for (let item of data) {
      item = item.split("=");
      if (item.length == 2) result[item[0]] = item[1];
    }

    return new ChatMessage(result);
  }

  to_string() {
    return `room_id=${this.room_id};username=${this.username};text=${this.text};kind=${this.kind};sent_at=${this.sent_at}`;
  }

  createNormalElement() {
    const element = document.createElement("div");
    element.classList.add("m-2");

    if (this.username == window.username) {
      element.innerHTML = `<div class="d-flex flex-column text-white rounded p-3 float-end msg-sent">
                <div class="text-break">${this.text}</div> 
                <div class="fs-6 fw-light text-end">${this.sent_at}</div>
            </div>`;
    } else {
      element.innerHTML = `<div class="d-flex flex-column text-white rounded p-3 float-start msg-received">
                <div class="fw-bold">${this.username}</div> 
                <div class="text-break">${this.text}</div> 
                <div class="fs-6 fw-light text-end">${this.sent_at}</div>
            </div>`;
    }

    return element;
  }

  createSystemElement() {
    const element = document.createElement("div");
    element.classList.add(
      "text-center",
      "my-2",
      this.kind == "join" ? "text-success" : "text-danger"
    );

    element.innerHTML = `<strong>${this.text}</strong> ${this.sent_at}`;

    return element;
  }

  createUserElement() {
    const element = document.createElement("div");
    element.classList.add("text-white", "m-2", "fs-6", "fw-bold", "text-break");

    element.id = `user-${this.username}`;
    element.innerHTML = this.username;

    return element;
  }
}
