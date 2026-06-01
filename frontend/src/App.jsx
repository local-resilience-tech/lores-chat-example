import { useState } from "react";
import useWebSocket from "react-use-websocket";
import "./App.css";

const WS_URL = `${window.location.protocol === "https:" ? "wss:" : "ws:"}//${window.location.host}/ws`;

export function App() {
  const [messages, setMessages] = useState([]);
  const [input, setInput] = useState("");
  const { sendMessage } = useWebSocket(WS_URL);

  function handleSubmit(e) {
    e.preventDefault();
    if (!input.trim()) return;
    sendMessage(input);
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