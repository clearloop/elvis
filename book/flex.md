# Flex

Follows MDN doc [CSS Flexible Box Layout][1]


## Widgets 📦

The very basic core of `Flex` Layout widgets is called `Flex`, unfortunatly, it contains almost all css flex properties, no easy to use, that's why elvis composes some newbie components as well.

### Align

```js
/* Align */
import { Align, Alignments, Elvis, Text } from "calling-elvis";

// Generate an `Align`
let myAlign = Align(
  Text("Align Elvis"), {
    align: Alignments.Center,
});

Elvis.call(myAlign);
```

`Align` inherits the core usage of `Alignment`, it's quite simple, just one property.

> Declaration: 
> 
> ```js
> function Align(
>   child: Widget, {
>   align: Alignments, 
> }): Widget;
> ```

### Center
```js
/* Center */
import { Center, Elvis, Text } from "calling-elvis";

// Generate an `Center`
let myCenter = Center(
  Text("My website only have a line of chars 🦀 "),
);

Elvis.call(myCenter);
```

`Center` is a very nice widget, if your website only have a line of chars, use it!

> Declaration: 
> 
> ```js
> function Center(child: Widget): Widget;
> ```

### Col
```js
/* Col */
import { Col, Elvis, Text } from "calling-elvis";

// Generate a `Col`
const myCol = Col([
  Text("All is above you all is sky"),
  Text("All is behind you all is sea"),
]);

Elvis.call(myCol);
```

`Col` towards column, the typical flow in html, with flexible enhance.

> Declaration: 
> 
> ```js
> function Col(widgets: Widget[]): Widget;
> ```

### Flex
```js
/* Flex */
import { Elvis, Flex, List, Text } from "calling-elvis";

const myFlex = Flex(
  List([
    Text("hi, I'm the Lunatic Widget's child No.1"),
    Text("hi, I'm the Lunatic widget's child No.2"),
    Text("hi, I'm the Lunatic Widget's child No.3"),
  )], {
    align: Alignments.Center,
    basis: FlexBasis.Fill(),
    direction: FlexDirection.ColumnReverse,
    grow: 1,
    order: 1,
    shrink: 1,
    wrap: true,
});

Elvis.call(myFlex);
```

This is the Lunatic Widget to Ground Control, I'm stepping throw the `Window`.


> Declaration: 
> 
> ```js
> function Flex(widget: Widget, {
>   align: Alignments,
>   basis: FlexBasis,
>   direction: FlexDirection,
>   grow: number,             // no unit
>   order: number,            // no unit
>   shrink: number,           // no unit
>   wrap: boolean,
> }): Widget;
> ```

### Row
```js
/* Row */
import { Elvis, Row, Text, } from "calling-elvis";

// Generate a `Row`
let myRow = Row([
  Text("I'm Wrong"),
  Text("I'm Right"),
]);

Elvis.call(myRow);
```

Both `Col` and `Row` are using `flex-start`, if you want to reverse the children of them, better to work on the list order.

> Declaration: 
> 
> ```js
> function Row(widgets: Widget[]): Widget;
> ```

## Enums 🍩

Elvis `Layout` Aligns follows the MDN doc [CSS Box Alignment][2], but simplify it into a enum `Aligment` here, `Alignment` is used by all `Flex` components and `Container` in Elvis.

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
  Number(Unit), // Rem
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
