import { Colors, Elvis, List, Text } from "../web/ts";

const Pig = Text(`Pink is the Pig!`, {
  bold: true,
  italic: true,
  size: 5,
  color: Colors.PinkAccent(),
});

const Show = Text(`The Show Must Go On!`, {
  bold: true,
  italic: true,
  size: 5,
  color: Colors.BlueAccent(),
});

const MyCenter = List([Pig, Show]);

// entry
new Elvis({
  home: MyCenter,
}).calling();
