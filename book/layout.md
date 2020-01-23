# Layout

Elvis layout follows the MDN doc [CSS Layout][1].

## Components 📦

Elvis Layout mainly contains [`Flex`](/flex.md) and [`Grid`](/grid.md), btw, Elvis offerrs two basic components `Container` and `SizedBox` for simple usages.

### Container
```js
/* Container */
import { Page, Elvis, Alignments } from "calling-elvis";
const { SizedBox, Text } = Elvis;

// Generate a `Container`
let myContainer = Container(
  Text("My Container"), {
    alignments: Alignments.Center,
    height: 42,
    margin: 10,
    padding: 10,
    width: 42,
});

Page(mySizedBox).render();
```

### SizedBox

```js
/* SizedBox */
import { Page, Elvis } from "calling-elvis";
const { SizedBox, Text } = Elvis;

// Generate a `SizedBox`
let mySizedBox = SizedBox(
  Text("My SizedBox"), {
    height: 2,
    width: 2,
});

Page(mySizedBox).render();
```

## Enums 🍩

Elvis Layout Aligns follows the MDN doc [CSS Box Alignment][2], but simplify it into a enum `Aligment` here, `Alignment` is used by all `Flex` components and `Container` in Elvis.

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

For using `Alignment` with `calling-elvis`, we write our code just like `Container` template above.

```js
/* Container */
import { Page, Elvis, Alignments } from "calling-elvis";
const { SizedBox, Text } = Elvis;

// Generate a `Container`
let myContainer = Container(
  Text("My Container"), {
    // change the line below
    alignments: Alignments.Center,
    height: 42,
    margin: 10,
    padding: 10,
    width: 42,
});

Page(mySizedBox).render();
```


[1]: https://developer.mozilla.org/en-US/docs/Learn/CSS/CSS_layout
[2]: https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Box_Alignment
