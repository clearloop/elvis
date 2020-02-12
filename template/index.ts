import { Colors, Elvis, Text, Router, Image } from "../web/ts";

const MyText = Text(`Pink is the Pig!`, {
  bold: true,
  italic: true,
  size: 8,
  color: Colors.PinkAccent(),
});

const MyImage = Image("http://08imgmini.eastday.com/mobile/20190221/20190221225048_ce81ae14103486b54f05f608ca64a7f6_1.jpeg", MyText);

const Show = Text(`The Show Must Go On!`, {
  bold: true,
  italic: true,
  size: 8,
  color: Colors.PinkAccent(),
});

// entry
new Elvis({
  home: MyImage,
  router: new Router({
    "show": Show,
  })
}).calling();
