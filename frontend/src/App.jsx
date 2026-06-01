import { useState } from "react";
import "./App.css";

export function App() {
  const [messages, setMessages] = useState([]);
  const [input, setInput] = useState("");

  function handleSubmit(e) {
    e.preventDefault();
    if (!input.trim()) return;
    setMessages((prev) => [...prev, input]);
    setInput("");
  }

  return (
    <div className="app on-light-blue">
      <form
        onSubmit={handleSubmit}
        className="message-form"
      >
        <input
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          placeholder="Type a message..."
        />
        <button type="submit">Send</button>
      </form>

      <ul className="message-list">
        {messages.map((msg, i) => (
          <li key={i}>{msg}</li>
        ))}
      </ul>
    </div>
  );
}