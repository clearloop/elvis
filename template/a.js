import { Colors, Elvis, Text, Widget } from "../web/ts";

const Home = () => Container(
  Grid({
    children: [
      A1(), A2(), A3(), A4(),
      B1(), B2(), B3(), B4(),
    ],
    textStyle: {
      size: 12,
      color: Colors.White,
      align: Align.Center,
    },
    repeat: Grid.AutoRows(3, 4)
  }), {
  top: 50,
  left: 50,
});
