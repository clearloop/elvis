import { Colors, Elvis, Grid, Text } from "../web/ts";

const Pig = Text(`Pink is the Pig!`, {
  bold: true,
  italic: true,
  size: 1,
  color: Colors.PinkAccent(),
});

const Show = Text(`The Show Must Go On!`, {
  bold: true,
  italic: true,
  size: 1,
  color: Colors.BlueAccent(),
});

const MyCenter = Grid([Pig, Show], {

});

// entry
new Elvis({
  home: MyCenter,
}).calling();
