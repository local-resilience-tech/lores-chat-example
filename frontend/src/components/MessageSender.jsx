import { useState } from "react";

export function MessageSender({ onSend }) {
  const [input, setInput] = useState("");

  function handleSubmit(e) {
    e.preventDefault();
    if (!input.trim()) return;
    onSend(input);
    setInput("");
  }

  return (
    <form onSubmit={handleSubmit} className="message-form">
      <input type="text" value={input} onChange={(e) => setInput(e.target.value)} placeholder="Type a message..." />
      <button type="submit">Send</button>
    </form>
  );
}
