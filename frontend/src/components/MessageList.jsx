import { UserIdentity } from "./UserIdentity";

export function MessageList({ messages }) {
  return (
    <ul className="message-list">
      {messages.map((msg, i) => (
        <li key={i} className={msg.from === "me" ? "message-me" : "message-server"}>
          <span className="message-meta">
            {msg.identity && <UserIdentity identity={msg.identity} />}
            {msg.author_node && <span className="message-author">{msg.author_node.slice(0, 8)}</span>}
          </span>
          {msg.text}
        </li>
      ))}
    </ul>
  );
}
