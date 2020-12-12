# Grid

Follows MDN doc [Grids][1].

Here is the `Grid` section, Just let Elvis show you how `Grid` Grid.

### Grid
```js
/* Grid */
import { Grid, GridAuto, GridFlow, GridTemplate, Elvis, Text } from "calling-elvis";

// Generate an `Grid`
const myGrid = Grid(
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
    col: Grid.Auto(),
    col_gap: 1,
    flow: GridFlow.Column(),
    row: Grid.Auto(),
    row_gap: 1,
    template_col: GridTemplate.Inherit(),
    template_row: GridTemplate.Inherit(),
});

Elvis.call(mySizedBox);
```

Take it ease, only one `Grid` widget in Elvis.

```js
/* Grid */
import { Grid, Elvis, List, Text } from "calling-elvis";

// Generate an `Grid`
const myGrid = Grid(
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

Elvis.call(myGrid);
```

`Grid` is quite complex in some way, usually, we just `Grid` our contains.

> **Declaration**
> 
> ```js
> function Grid(widget: Widget, {
>   col: GridAuto,
>   col_gap: number,              // Rem
>   flow: GridFlow,
>   row: GridAuto,
>   row_gap: number,              // Rem
>   template_col: GridTemplate,
>   template_row: GridTemplate,
> }): Widget;
> ```

## Enums 🍩

Grid `Grid` is hard to pronounce, most of time we don't need to do this.

### GridAuto
```rust
pub enum GridAuto {
    Auto,
    Fixed(Unit),
    Inherit,
    Initial,
    MaxContent,
    MinContent,
    MinMax(Unit, Unit),
    Plain(Vec<Unit>),
    Unset,
}
```

`GridAuto` affect the width of Grid children, and the `Auto` choice use the `minmax` function in css, if doesn't pass the second argument, it will be `auto` in meaning.


### GridFlow
```rust
pub enum GridFlow {
    Column,
    Row,
    Dense,
    ColumnDense,
    RowDense,
    Inherit,
    Initial,
    Unset,
}
```

`GridFlow::Column` by default.

### GridTemplate
```rust
pub enum GridTemplate {
    FitContent(Unit),
    Inherit,
    Initial,
    MinMax(Unit, Unit),
    None,
    Plain(Vec<Unit>),
    Repeat(i32, Unit),
    SubGrid,
    Unset,
}
```
In the `Plain` choice, `Vec`'s length will be the column count of grid, and every `Unit` is the width of each column, `Repeat` just make this easier, every child are in the same width.

[1]: https://developer.mozilla.org/en-US/docs/Learn/CSS/CSS_layout/Grids
