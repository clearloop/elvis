# Text

```js
/* Text */
import { Center, Colors, Elvis, Text } from "calling-elvis";

Elvis.call(
  Center(
    Text("Calling Elvis", {
      bold: true,
      color: Colors.PinkAccent(),
      italic: true,
      size: 42.0,
      weight: 700,
      height: 20,
      stretch: 100,
    }),
  ),
);
```

`Text` might be the most popular spider from Mars, Does it know the Great `Ziggy Stardust`?

> Declaration:
>
> ```js
> function Text(text: string, {
>   bold: boolean,
>   color: Colors,
>   italic: boolean,
>   size: number,    // rem
>   weight: number,  // rem
>   height: number,  // rem
>   stretch: number, // percent
> }): Widget;
> ```
