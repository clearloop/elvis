import { Colors, Elvis, Text, Widget } from "../web/ts";

class MyWidget extends Widget {
  public render() {
    return Text("pink is the pig!", {
      bold: true,
      italic: true,
      size: 10,
      color: Colors.PinkAccent,
    });
  }
}

new Elvis({
  home: new MyWidget(),
}).calling();
