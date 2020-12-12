# MultiColumn

Follows MDN doc [Multiple-column layout][1].

The very easy way to layout our pages, maybe fantastic in the old web, it means, the true web, I like it. 

### MultiColumn
```js
/* MultiColumn */
import { Colors, Elvis, MultiColumn, MultiColumnLineStyle, Text } from "calling-elvis";

// Generate an `MultiColumn`
const myMultiColumn = MultiColumn(
  List(
    Text("Mercury"),
    Text("Venus"),
    Text("Earth"),
    Text("Mars"),
    Text("Jupiter"),
    Text("Saturn"),
    Text("Uranus"),
    Text("Neptune"),
    Text("Pluto"),
  ), {
    color: Colors.Black(),
    count: 3,
    gap: 20,
    style: MultiColumnLineStyle.Groove,
});

Elvis.call(mySizedBox);
```

Homework: code a New York Times.

**Declaration**

```js
function MultiColumn(
  widget: Widget, {
    color: Colors,
    count: number,                  // no unit
    gap: number,                    // Rem
    style: MultiColumnLineStyle,
}): Widget;
```

## Enums 🍩

The style of `MultiColumnLine`.

### MultiColumnLineStyle
```rust
pub enum MultiColumnLineStyle {
  None,
  Hidden,
  Dotted,
  Dashed,
  Solid,
  Double,
  Groove,
  Ridge,
  Inset,
  OutSet
}
```

If I were you, I will choose `MultiColumnStyle.Groove`, don't ask why.

[1]: https://developer.mozilla.org/en-US/docs/Learn/CSS/CSS_layout/Multiple-column_Layout
