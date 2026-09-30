export class Thing {
  #name;
  constructor(name) { this.#name = name; }
  name() {
    console.log(`name() called on ${this.#name}`);
    return this.#name;
  }
}

export function root() {
  return new Thing("root");
}

export async function make(name) {
  return new Thing(name);
}
