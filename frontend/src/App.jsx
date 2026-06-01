import { useState } from "react";
import useWebSocket from "react-use-websocket";
import "./App.css";
import { ServerIndicator } from "./components/ServerIndicator";

const WS_URL = `${window.location.protocol === "https:" ? "wss:" : "ws:"}//${window.location.host}/ws`;

export function App() {
  const [messages, setMessages] = useState([]);
  const [input, setInput] = useState("");
  const [hasError, setHasError] = useState(false);
  const { sendMessage, readyState } = useWebSocket(WS_URL, {
    onError: () => setHasError(true),
    onOpen: () => setHasError(false),
    onMessage: (event) => {
      try {
        const data = JSON.parse(event.data);
        if (data.type === "error") {
          setHasError(true);
        }
      } catch {
        // non-JSON messages are fine
      }
    },
  });

  function handleSubmit(e) {
    e.preventDefault();
    if (!input.trim()) return;
    setHasError(false);
    sendMessage(input);
    setMessages((prev) => [...prev, input]);
    setInput("");
  }

  return (
    <div className="app on-light-blue">
      <div className="header">
        <div className="title">
          <h1>Lores Chat Example</h1>
          <ServerIndicator readyState={readyState} hasError={hasError} />
        </div>
        <form onSubmit={handleSubmit} className="message-form">
          <input type="text" value={input} onChange={(e) => setInput(e.target.value)} placeholder="Type a message..." />
          <button type="submit">Send</button>
        </form>
      </div>

      <ul className="message-list">
        {messages.map((msg, i) => (
          <li key={i}>{msg}</li>
        ))}
      </ul>
    </div>
  );
}
