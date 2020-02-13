# Image

```js
/* Image */
import { Center, Elvis, Text, Image } from "calling-elvis";

// Generate a `Image`
const myImage = Image(
  "https://images-assets.nasa.gov/image/S65-34635/S65-34635~orig.jpg", 
  Text("hallo, spaceboy!"),
);

Elvis.call(Center(myImage));
```

If you don't want `Image` playing in background anonymously, just remove the child field.
