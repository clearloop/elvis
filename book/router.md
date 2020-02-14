# Router

```js
/* Entry */
import { Elvis, Router } from "calling-elvis";
import { Africa, America, Asia, Europe, Oceania } from "./map";

new Elvis({
  home: Oceania,
  routes: {
    "africa": Africa,
    "america": America,
    "asia": Asia,
    "europe": Europe,
    "oceania": Oceania,
  },
})
```

A ship got a `Navigator`, we call it an App.


## TODO*

We generate our routes in the entry of our Apps usually, `Elvis` has inner url parser in the navigator process, both url parameters and object style arguments are supported, so if we want to fly to the Mars, just do it.
