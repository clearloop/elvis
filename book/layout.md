# Layout

Elvis layout follows the MDN doc [CSS Layout][1].

## Components 📦

Elvis Layout mainly contains [`Flex`](/flex.md) and [`Grid`](/grid.md), btw, Elvis offerrs two basic components `Container` and `SizedBox` for simple usages.

### Container
```js
/* Container */
import { Page, Elvis, Alignments, Color } from "calling-elvis";
const { SizedBox, Text } = Elvis;

// Generate a `Container`
let myContainer = Container(
  Text("Where is my AJ-1?"), {
    alignments: Alignments.Center,
    height: 42,
    margin: 10,
    padding: 10,
    width: 42,
    color: Colors.Black,
    backgroundColor: Colors.Red,
});

Page(mySizedBox).render();
```

The `Alignments` enum is from [Flex](/flex.md), to be honest, `Container` component is a part of `Flex` family, but he is too brilliant to stay in `Flex` family, `Layout` calls him.


### SizedBox

```js
/* SizedBox */
import { Page, Elvis } from "calling-elvis";
const { SizedBox, Text } = Elvis;

// Generate a `SizedBox`
let mySizedBox = SizedBox(
  Text("My SizedBox"), {
    height: 42,
    width: 42,
});

Page(mySizedBox).render();
```

`SizedBox` just has `width` and `height` two arguments, we use this component to take some white space usually.

[1]: https://developer.mozilla.org/en-US/docs/Learn/CSS/CSS_layout
[2]: https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Box_Alignment

