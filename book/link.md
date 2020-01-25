# Link

```js
/* Link */
import { Elvis, Page } from "calling-elvis";
const { Link } = Elvis;

Page(
  Center(
    Link(
      Text("Across the Great Wall, we can reach every corner in the world."),
      href: "https://google.com",
    ),
  ),
).render();
```

`Link` mainly links `url`s outside our picture, if you wanna spray your paint here there and everywhere, `Router` is waiting for you.
