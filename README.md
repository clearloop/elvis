+ Preparing to merge `elvis.js` and `elvis-web` into this one repo.
+ Rewriting the web client of [rust.cc][rust.cc]...

# Calling Elvis 🦀 🕸  🎸 📡 🚀 🪐 🛰

![Rust](https://github.com/clearloop/leetcode-cli/workflows/Rust/badge.svg)
[![crate](https://img.shields.io/crates/v/elvis.svg)](https://crates.io/crates/elvis)
[![doc](https://img.shields.io/badge/current-docs-brightgreen.svg)](https://docs.rs/elvis/)
[![downloads](https://img.shields.io/crates/d/elvis.svg)](https://crates.io/crates/elvis)
[![gitter](https://img.shields.io/gitter/room/elvisjs/community)](https://gitter.im/elvisjs/community)
[![LICENSE](https://img.shields.io/crates/l/elvis.svg)](https://choosealicense.com/licenses/mit/)


Is anybody home?

As we know, `Elvis` is a famous rock star, both a famous rock song named `Calling Elvis` wrote by `Dire Straits` which inspired a unknown rock star writing down these chords.

For now, Elvis, the rock star will rise once again, **beyond** the internet —— truely your wasm web library, 🦀 + 🕸  => 💖

[The Evlis Book][1] mainly talks about the usage of [calling-elvis][2], if you want to use `low-level` api rusting the web, plz check out [elvis' rust doc][3].

## Goals 🎯

Writing web pages in **pure rust or javascript** using wasm bindings, **without** `jsx` or `any other` complex syntax, of course, **not** writing `html` nor `css` either.

## Roll up for the Magical Mystery Tour! 🌈

🧙‍♂️ 🤹‍♂️ Here we go! Roll up, roll up for the mystery tour, the magical mystery tour is waiting to take you away! Hoping to take you away! Coming to take you away! Dying to take you away, take you today! 🛸

🎻 Let me take you down, cause I'm going to,



### Rust

```rust
use elvis::widgets::{layouts::Center, Text, TextStyle},
use elvis_web::Widget;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    let mut center = Widget::new(Center {
        child: Text {
            text: "Pink is the Pig!".into(),
            style: TextStyle::default(),
        }
        .into(),
    });

    center.calling().unwrap();
}

```



### Javascript

```js
import { Colors, Elvis, Text } from "calling-elvis";

const Home = Text("Pink is the Pig!", {
  bold: true,
  italic: true,
  size: 10,
  color: Colors.PinkAccent(),
});

new Elvis({
  home: Home,
}).calling();
```
 **Strawberry Fields** 🧑‍🚀

## 21st Century Schizoid Magic 🍩

### Rust

```text
$ cargo install epm
$ epm
epm 0.0.2

USAGE:
    epm <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    dev     Start develop server
    help    Prints this message or the help of the given subcommand(s)
    init    Create a new elvis package in an existing directory
    new     Create a new elvis package

$ epm new my-awesome-project
$ cd my-awesome-project && epm dev

```



### Javascript

```text
$ yarn create elvis-app
✔ What is your project named? … my-awesome-app
[ info ] Generating elvis files ...
[ wait ] Installing elvis dependencies ...
[ done ] Let's Roll up for the Magical Mystery Tour!

   ┌─────────────────────────────────────────────────────────────┐
   │                                                             │
   │   Success! Created my-awesome-app at:                       │
   │                                                             │
   │     /path/to/my-awesome-app                                 │
   │                                                             │
   │   Inside that directory, you can run several commands:      │
   │                                                             │
   │     - yarn dev:    Starts the development server.           │
   │     - yarn docs:   Open The Elvis Book.                     │
   │     - yarn build:  Builds my-awesome-app for production.    │
   │     - yarn start:  Runs my-awesome-app production mode.     │
   │                                                             │
   │   We suggest that you begin by typing:                      │
   │                                                             │
   │     - cd my-awesome-app                                     │
   │     - yarn dev                                              │
   │                                                             │
   │                                                             │
   └─────────────────────────────────────────────────────────────┘
```

## LICENSE

Heartbreak Hotel.

[1]: https://elvisjs.github.io/the-elvis-book
[2]: https://github.com/elvisjs/calling-elvis
[3]: https://docs.rs/elvis
[rust.cc]: https://rustcc.cn
