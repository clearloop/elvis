import { Elvis, TextStyle, Text, Colors, Unit, UnitAbbr } from "../web/pkg";


// tests
Elvis.render(
  new Text(
    "Pink is the Pig",
    new TextStyle(
      true,
      Colors.PinkAccent,
      true,
      new Unit(10, UnitAbbr.Rem),
      new Unit(1, UnitAbbr.None),
      new Unit(10, UnitAbbr.Rem),
      new Unit(30, UnitAbbr.Percent),
    )
  ).html
);
