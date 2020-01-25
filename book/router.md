# Router

```js
/* Entry */
import { App } from "calling-elvis";
import { Africa, America, Asia, Europe, Oceania } from "./map";

class myApp extends App {
  render() {
    return App(
      title: "Across the earth",
      initialRoute: "/",
      routes: {
        "africa": Africa,
        "america": America,
        "asia": Asia,
        "europe": Europe,
        "Oceania": Oceania,
      },
    );
  }
}
```

A ship have a `Navigator`, we call them App.

```js
/* Router */
import { Elvis, Router, GestureDetector, Component } from "calling-elvis";
const { Center, Container, Text } = Elvis;

export default class myComponent extends Component {
  render() {
    return Page(
      Center(
        GestureDetector(
          // onTap: () => Router.push("/start?target=Mars"),
          onTap: () => Router.push(
            "/stars",
            arguments: {
              target: "Mars",
            },
          ),
        ),
        Text("Lead me to the stars"),
      ),
    );
  }
}
```

We generate our routes in the entry of our Apps usually, `Elvis` has inner url parser in the navigator process, both url parameters and object style arguments are supported, so if we want to fly to the Mars, just do it.
