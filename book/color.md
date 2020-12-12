# Color

Follows MDN doc [CSS Color Value][1]

```rust
pub enum Colors {
  RGBA(i32, i32, i32, f64),
  Hex(String),
  HSL(Unit, i32, i32),
  Plain(String),
  Amber,
  AmberAccent,
  Black,
  Blue,
  BlueAccent,
  BlueGrey,
  Brown,
  Cyan,
  CyanAccent,
  DeepOrange,
  DeepOrangeAccent,
  DeepPurple,
  DeepPurpleAccent,
  Green,
  GreenAccent,
  Grey,
  Indigo,
  IndigoAccent,
  LightBlue,
  LightBlueAccent,
  LightGreen,
  LightGreenAccent,
  Lime,
  LimeAccent,
  Orange,
  OrangeAccent,
  Pink,
  PinkAccent,
  Purple,
  PurpleAccent,
  Red,
  RedAccent,
  Teal,
  TealAccent,
  Transparent,
  White,
  Yellow,
  YellowAccent,
}
```

So many colors...well, the `Colors` above is Elvis' color system.

[1]: https://developer.mozilla.org/en-US/docs/Web/CSS/color_value
