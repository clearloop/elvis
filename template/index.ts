import { Colors, Elvis, Text, Router, Image } from "../web/ts";

const MyText = Text(`Pink is the Pig!`, {
  bold: true,
  italic: true,
  size: 8,
  color: Colors.PinkAccent(),
});

const MyImage = Image("http://img1.gtimg.com/cul/pics/hv1/58/22/2141/139224193.jpg", MyText);

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
