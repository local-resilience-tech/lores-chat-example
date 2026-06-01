import { ReadyState } from "react-use-websocket";

const STATUS_CONFIG = {
  [ReadyState.CONNECTING]: { emoji: "🟡", label: "Connecting" },
  [ReadyState.OPEN]: { emoji: "🟢", label: "Connected" },
  [ReadyState.CLOSING]: { emoji: "🟠", label: "Closing" },
  [ReadyState.CLOSED]: { emoji: "🔴", label: "Disconnected" },
  [ReadyState.UNINSTANTIATED]: { emoji: "⚪", label: "Not started" },
};

export function ServerIndicator({ readyState, hasError }) {
  const { emoji, label } = hasError ? { emoji: "❌", label: "Error" } : (STATUS_CONFIG[readyState] ?? { emoji: "⚪", label: "Unknown" });

  return (
    <span className="server-indicator" title={label} aria-label={`Server status: ${label}`}>
      {emoji}
    </span>
  );
}
