import { Colors, Elvis, Text, Widget, Router } from "../web/ts";
const MyText = (name: string) => Text(`Pink is the ${name}!`, {
  bold: true,
  italic: true,
  size: 8,
  color: Colors.PinkAccent(),
});

// self widget
// class Home extends Widget {
//   constructor() {
//     super();
//   }
// 
//   public create() {
//     console.log("create");
//   }
// }

// entry
new Elvis({
  home: MyText("Pigger")
}).calling();
