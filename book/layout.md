# Layout

Follows MDN doc [CSS Layout][1].

## Widgets 📦

Elvis Layout mainly contains `Flex` and `Grid`, btw, Elvis offerrs two basic widgets `Container` and `SizedBox` for simple usages.

### Container
```js
/* Container */
import { Container, Colors, Elvis, Alignments } from "calling-elvis";

// Generate a `Container`
const myContainer = Container(
  Text("Where is my AJ-1?"), {
    align: Alignments.Center,
    color: Colors.Black(),
    height: 42,
    margin: 10,
    padding: 10,
    width: 42,
});

Elvis.call(myContainer);
```

The `Alignments` enum is from `Flex`, to be honest, `Container` component is a part of `Flex` family, but he is too brilliant to stay in `Flex` family, `Layout` calls him.

> Declaraction:
> 
> ```js
> function Container(child: Widget, {
>   align: Alignments,
>   color: Colors,
>   height: number,        // rem
>   margin: number,        // rem
>   padding: number,       // rem
>   width: number,         // rem
> }): Widget;
> ```

### List
```js
/* List */
import { Elvis, List, Text } from "calling-elvis";

// Generate a `List`
let myList = List([
  Text("poor-orphan-1"),
  Text("poor-orphan-2"),
  Text("poor-orphan-3"),
]);

Elvis.call(myList);
```

(Sorry about that), `List` is a set of poor orphan children, they don't have any style, just blowing in the wind.

> Declaraction:
> 
> ```js
> function List(widgets: Widget[]): Widget;
> ```

### SizedBox

```js
/* SizedBox */
import { Elvis, SizedBox, Text } from "calling-elvis";

// Generate a `SizedBox`
const mySizedBox = SizedBox(
  Text("My SizedBox"), {
    height: 42,
    width: 42,
});

Elvis.call(mySizedBox);
```

`SizedBox` just has `width` and `height` two arguments, we use this component to take some white space usually.

> Declaraction:
>
> ```js
> function SizedBox(widget: Widget, {
>   height: number, // rem
>   width: number,  // rem
> }): Widget;
> ```

[1]: https://developer.mozilla.org/en-US/docs/Learn/CSS/CSS_layout
[2]: https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Box_Alignment

