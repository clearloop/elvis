# Text

```js
/* Text */
import { Elvis, Page, TextStyle, Colors } from "calling-elvis";
const { Center, Text } = Elvis;

Page(
  Center(
    Text(
      "Calling Elvis",
      new TextStyle({
        bold: true,
        color: Colors.Red,
        italic: true,
        size: 42.0,
        weight: 700,
        height: 20,
        stretch: 100,
      }),
    ),
  ),
).render();
```

`Text` might be the most popular spider from Mars, Does it know the Great `Ziggy Stardust`?
