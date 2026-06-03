import { useState, useEffect, useMemo } from "react";
import useWebSocket from "react-use-websocket";
import "./App.css";
import { ServerIndicator } from "./components/ServerIndicator";
import { UserIdentity } from "./components/UserIdentity";
import { MessageSender } from "./components/MessageSender";
import { MessageList } from "./components/MessageList";
import { RegionSelector } from "./components/RegionSelector";
import { generateIdentity } from "./identity";

const WS_URL = (regionId) => `${window.location.protocol === "https:" ? "wss:" : "ws:"}//${window.location.host}/ws/${regionId}`;

export function App() {
  const identity = useMemo(() => generateIdentity(), []);
  const [messages, setMessages] = useState([]);
  const [hasError, setHasError] = useState(false);
  const [regions, setRegions] = useState(null);
  const [currentRegion, setCurrentRegion] = useState(null);

  useEffect(() => {
    fetch("/api/regions")
      .then((res) => res.json())
      .then((data) => {
        setRegions(data);
        if (data.length === 1) {
          setCurrentRegion(data[0]);
        }
      })
      .catch((err) => console.error("failed to fetch regions:", err));
  }, []);

  const { sendMessage, readyState } = useWebSocket(currentRegion ? WS_URL(currentRegion) : null, {
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
        if (data.author_node !== undefined && data.text !== undefined) {
          console.log("[subscribe] received message from server:", event.data);
          let text = data.text;
          let senderIdentity = null;
          try {
            const payload = JSON.parse(data.text);
            if (payload.text !== undefined && payload.identity !== undefined) {
              text = payload.text;
              senderIdentity = payload.identity;
            }
          } catch {
            // plain text message, no identity
          }
          setMessages((prev) => [...prev, { text, identity: senderIdentity, author_node: data.author_node, from: "server" }]);
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
    const payload = JSON.stringify({ identity, text });
    console.log("[publish] sending message to backend:", payload);
    sendMessage(payload);
  }

  return (
    <div className="app on-light-blue">
      <div className="header">
        <div className="title">
          <h1>Lores Chat Example</h1>
          <div className="title-right">
            <UserIdentity identity={identity} />
            <ServerIndicator readyState={readyState} hasError={hasError} />
          </div>
        </div>
        {regions === null ? <p>loading regions...</p> : currentRegion ? <MessageSender onSend={handleSend} /> : <RegionSelector regions={regions} onSelect={setCurrentRegion} />}
      </div>

      <MessageList messages={messages} />
    </div>
  );
}
