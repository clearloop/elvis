import { Elvis, TextStyle, Text, Colors, Unit, UnitAbbr } from "../web/pkg";


// tests
Elvis.render(
  new Text(
    "hello, world!",
    new TextStyle(
      true,
      Colors.Pink,
      true,
      new Unit(10, UnitAbbr.Rem),
      new Unit(1, UnitAbbr.None),
      new Unit(1, UnitAbbr.Rem),
      new Unit(1, UnitAbbr.Rem),
    )
  ).html
);

console.log(new TextStyle(
  true,
  Colors.Pink,
  true,
  new Unit(10, UnitAbbr.Rem),
  new Unit(1, UnitAbbr.None),
  new Unit(1, UnitAbbr.Rem),
  new Unit(1, UnitAbbr.Rem),
).ser())

console.log(new Text(
  "hello, world!",
  new TextStyle(
    true,
    Colors.Pink,
    true,
    new Unit(10, UnitAbbr.Rem),
    new Unit(1, UnitAbbr.None),
    new Unit(1, UnitAbbr.Rem),
    new Unit(1, UnitAbbr.Rem),
  )
).html)
