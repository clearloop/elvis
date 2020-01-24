# Image

```js
/* Imageoo */
import { Page, Elvis } from "calling-elvis";
const { Text, Image } = Elvis;

// Generate a `Image`
let myImage = Image({
  child: Center(
    Text("Hallo, Spaceboy!")
  ),
  url: "https://images-assets.nasa.gov/image/S65-34635/S65-34635~orig.jpg",
  height: 42,
  width: 42,
});

Page(myImage).render();
```

If you don't want `Image` to play in background anonymously, just remove the child field.
