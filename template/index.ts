import { Colors, Elvis, Text, Widget, Router } from "../web/ts";
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
  home: MyText("Pigger"),
  router: new Router({
    "show": Show,
  })
}).calling();
