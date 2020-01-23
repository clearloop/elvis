# Flex

Elvis Flex follows the MDN doc [CSS Flexible Box Layout][1]


## Components 📦

The very basic core of `Flex Layout` components is called `Flex`, unfortunatly, it contains almost all css flex properties, not easy to use, that's why elvis composes some "newbie" components as well.

### Align

```js
/* Align */
import { Page, Elvis, Alignments } from "calling-elvis";
const { Align, Text } = Elvis;

// Generate an `Align`
let myAlign = Align(
  Text("Align Elvis"), {
    alignments: Alignments.Center,
});

Page(mySizedBox).render();
```

`Align` inherits the core usage of `Alignment`, it's quite simple, just one property.

### Center
```js
/* Center */
import { Page, Elvis, Alignments } from "calling-elvis";
const { Center, Text } = Elvis;

// Generate an `Center`
let myCenter = Center(
  Text("My website only have a line of chars 🦀 "),
);

Page(mySizedBox).render();
```
`Center` is a very nice component, if your website only have a line of chars, use it!


### Col
```js
/* Col */
import { Page, Elvis, Alignments } from "calling-elvis";
const { Col, Text } = Elvis;

// Generate an `Col`
let myAlign = Col([
  Text("All is above you all is sky"),
  Text("All is behind you all is sea"),
]);

Page(mySizedBox).render();
```

`Col` towards column, the typical flow in html, with flexible enhance.


### Flex
```js
let flex = Flex(
  List([
    Text("hi, I'm the Lunatic Component's child No.1"),
	Text("hi, I'm the Lunatic Component's child No.2"),
	Text("hi, I'm the Lunatic Component's child No.3"),
  )], {
    basis: FlexBasis.Auto,
    direction: FlexDirection.ColumnReverse,
	grow: 1,
	order: 1,
	shrink: 2,
    wrap: FlexWrap.Wrap,
});
```
This is `the Lunatic Component` to `Ground Control`, I'm stepping throw the `Window`.


### Row
```js
/* Row */
import { Page, Elvis, Alignments } from "calling-elvis";
const { Row, Text } = Elvis;

// Generate a `Row`
let myAlign = Row([
  Text("I'm Wrong"),
  Text("I'm Right"),
]);

Page(mySizedBox).render();
```

Both `Col` and `Row` are using `flex-start`, if you want to reverse the children of them, better to work on the list order.

## Enums 🍩

Elvis Layout Aligns follows the MDN doc [CSS Box Alignment][2], but simplify it into a enum `Aligment` here, `Alignment` is used by all `Flex` components and `Container` in Elvis.

### Alignment

Here is the `Alignment` defination in rust source code.

```rust
/// Magical Alignment
pub enum Alignment {
  BottomCenter,
  BottomLeft,
  BottomRight,
  Center,
  CenterLeft,
  CenterRight,
  TopCenter,
  TopLeft,
  TopRight,
}
```

### FlexBasis
```rust
pub enum FlexBasis {
  Fill,
  MaxContent,
  MinContent,
  FitContent,
  // em default
  Number(f64),
}
```
Well, lunatic `FlexBasis` in Rust source code.

### FlexDirection
```rust
pub enum FlexDirection {
  Column,
  ColumnReverse,
  Row,
  RowReverse,
}
```
Lunatic `FlexDirection`, you know it.


[1]: https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Flexible_Box_Layout
