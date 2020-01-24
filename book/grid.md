# Grid

Follows MDN doc [Grids][1].

Here is the `Grid` section, Just let Elvis show you how `Grid` Grid.

## Components 📦

`Grid` just have one component now, as you can see, `Grid`.

### Grid
```js
/* Grid */
import { Page, Elvis } from "calling-elvis";
const { Grid, Text, List } = Elvis;

// Generate an `Grid`
let myGrid = Grid(
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
    autoRows: GridAutoRows.Auto(100),
    // col: 3,
    // gap: 2,
    // row: 3,
    // template: GridTemplate.Repeat(3, 1),
});

Page(mySizedBox).render();
```

Aah, duplicate name occurs, take it ease, only one `Grid` Component in Elvis too.

```js
/* Grid */
import { Page, Elvis, Alignments } from "calling-elvis";
const { Grid, Text, List } = Elvis;

// Generate an `Grid`
let myGrid = Grid(
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
  ),
);

Page(mySizedBox).render();
```

`Grid` is quite complex in some way, usually, we just `Grid` our contains.

## Enums 🍩

Grid `Grid` is hard to pronounce, most of time we don't need to do this.

### GridAutoRows
```rust
pub enum GridAutoRows {
  Auto(Unit, Option<Unit>),
  Fixed(Unit),
}
```

`AutoRows` affect the width of Grid children, and the `Auto` choice use the `minmax` function in css, if doesn't pass the second argument, it will be `auto` in meaning.

### GridTemplate
```rust
pub enum GridTemplate {
  Plain(Vec<Unit>),
  Repeat(i32, Unit),
}
```
In the `Plain` choice, `Vec`'s length will be the column count of grid, and every `Unit` is the width of each column, `Repeat` just make this easier, every child are in the same width.

[1]: https://developer.mozilla.org/en-US/docs/Learn/CSS/CSS_layout/Grids
