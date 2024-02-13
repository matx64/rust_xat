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
}
