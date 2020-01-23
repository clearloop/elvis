# MultiColumn

The very easy way to layout our pages, maybe fantastic in the old web, it means, the true web, I like it. 

Follows MDN doc [Multiple-column layout][1].

## Components 📦

Congratulations! one compoent got shot, again.

### MultiColumn
```js
/* MultiColumn */
import { Colors, Page, Elvis } from "calling-elvis";
const { Grid, Text, List } = Elvis;

// Generate an `Grid`
let myMultiColumn = MultiColumn(
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
    color: Colors.red,
    count: 3,
    gap: 20,
    style: MultiColumnStyle.Dotted,
});

Page(mySizedBox).render();
```

Homework, code a New York Times.

## Enums

Mainly the style of `MultiColumn`.

### MultiColumnStyle
```rust
pub enum MultiColumnStyle {
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

If I were you, I will choose `MultiColumnStyle.Groove`, don't ask me why.

[1]: https://developer.mozilla.org/en-US/docs/Learn/CSS/CSS_layout/Multiple-column_Layout
