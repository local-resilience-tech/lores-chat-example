const COLOURS = ["red", "blue", "green", "purple", "orange", "pink", "teal", "yellow"];
const ANIMALS = ["bear", "fox", "wolf", "owl", "deer", "lynx", "hawk", "frog"];

export function generateIdentity() {
  const colour = COLOURS[Math.floor(Math.random() * COLOURS.length)];
  const animal = ANIMALS[Math.floor(Math.random() * ANIMALS.length)];
  return { colour, animal, name: `${colour} ${animal}` };
}
