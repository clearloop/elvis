import { TextElement as X, TextStyle as S } from "../TextElement";

// Basic text class using `div`
class Text extends X {
  constructor(text: string, style: S) {
    super("div", text, style);
  }
}

class Title extends X {
  constructor(text: string, style: S) {
    super("h1", text, style);
  }
}

class SubTitle extends X {
  constructor(text: string, style: S) {
    super("h2", text, style);
  }
}

class Headline extends X {
  constructor(text: string, style: S) {
    super("h3", text, style);
  }
}

export {
  Text,
  Title,
  SubTitle,
  Headline,
};
