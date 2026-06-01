import { useState } from "react";
import useWebSocket from "react-use-websocket";
import "./App.css";
import { ServerIndicator } from "./components/ServerIndicator";
import { MessageSender } from "./components/MessageSender";
import { MessageList } from "./components/MessageList";

const REGION_ID = "003f1de60ac340ba64b73d3e97bd25f694c73ab178b52f246f8a05bcafcc1676";
const WS_URL = `${window.location.protocol === "https:" ? "wss:" : "ws:"}//${window.location.host}/ws/${REGION_ID}`;

export function App() {
  const [messages, setMessages] = useState([]);
  const [hasError, setHasError] = useState(false);
  const { sendMessage, readyState } = useWebSocket(WS_URL, {
    onError: () => setHasError(true),
    onOpen: () => {
      setHasError(false);
    },
    onMessage: (event) => {
      try {
        const data = JSON.parse(event.data);
        if (data.type === "error") {
          setHasError(true);
          return;
        }
      } catch {
        // non-JSON messages fall through
      }
      console.log("[subscribe] received message from server:", event.data);
      setMessages((prev) => [...prev, { text: event.data, from: "server" }]);
    },
  });

  function handleSend(text) {
    setHasError(false);
    console.log("[publish] sending message to backend:", text);
    sendMessage(text);
  }

  return (
    <div className="app on-light-blue">
      <div className="header">
        <div className="title">
          <h1>Lores Chat Example</h1>
          <ServerIndicator readyState={readyState} hasError={hasError} />
        </div>
        <MessageSender onSend={handleSend} />
      </div>

      <MessageList messages={messages} />
    </div>
  );
}
