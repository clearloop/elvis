# Tutorial

Well this is an official tutorial for `Elvis`.

## Installation

### Use `create-elvis-app`

```sh
yarn create elvis-app
```

Use `create-elvis-app` to generate elvis structure directly.

### Use `elvis-cli`

```sh
# create directly
mkdir my-awesome-app && cd my-awesome-app

# add elvis-cli
yarn global add elvis-cli

# add elvis library
yarn add calling-elvis

# elvis-cli will generate pages defaultly
elvis dev
```

### Starting manually

```sh
# create directly
mkdir my-awesome-app && cd my-awesome-app

# add elvis library
yarn add calling-elvis
```

```js
/* router */
import { Elvis } from "calling-elvis";
import Home from "./home";

new Elvis({
  home: Index(),
}).calling();
```

```js
/* home */
import { Center, Text } from "calling-elvis";
const Home = () => Center(
  Text("Calling Elvis!")
);

export default Home;
```

### Starting Rustly

Checkout elvis' [rust doc][doc]


[doc]: https://docs.rs/elvis/0.2.1/elvis/
