const ANIMAL_EMOJI = {
  bear: "🐻",
  fox: "🦊",
  wolf: "🐺",
  owl: "🦉",
  deer: "🦌",
  lynx: "🐱",
  hawk: "🦅",
  frog: "🐸",
};

const COLOUR_STYLES = {
  red:    { background: "#c0392b", color: "#fff" },
  blue:   { background: "#2980b9", color: "#fff" },
  green:  { background: "#27ae60", color: "#fff" },
  purple: { background: "#8e44ad", color: "#fff" },
  orange: { background: "#e67e22", color: "#fff" },
  pink:   { background: "#e91e8c", color: "#fff" },
  teal:   { background: "#16a085", color: "#fff" },
  yellow: { background: "#f1c40f", color: "#333" },
};

export function UserIdentity({ identity }) {
  const emoji = ANIMAL_EMOJI[identity.animal] ?? "🐾";
  const style = COLOUR_STYLES[identity.colour] ?? { background: "#888", color: "#fff" };

  return (
    <span
      className="user-identity"
      title={identity.name}
      aria-label={`Your identity: ${identity.name}`}
      style={style}
    >
      {emoji} {identity.name}
    </span>
  );
}
