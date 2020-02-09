import { Colors, Elvis, Text, Router } from "../web/ts";
const MyText = (name: string) => Text(`Pink is the ${name}!`, {
  bold: true,
  italic: true,
  size: 8,
  color: Colors.PinkAccent(),
});

const Show = Text(`The Show Must Go On!`, {
  bold: true,
  italic: true,
  size: 8,
  color: Colors.PinkAccent(),
});

// entry
new Elvis({
  home: MyText("Pig"),
  router: new Router({
    "show": Show,
  })
}).calling();
