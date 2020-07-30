+ Preparing to merge `elvis.js` and `elvis-web` into this one repo.
+ Trying to Rewrite the web client of [rust.cc][rust.cc]...

# Calling Elvis 🦀 🕸  🎸 📡 🚀 🪐 🛰

![Rust](https://github.com/clearloop/leetcode-cli/workflows/Rust/badge.svg)
[![crate](https://img.shields.io/crates/v/elvis.svg)](https://crates.io/crates/elvis)
[![doc](https://img.shields.io/badge/current-docs-brightgreen.svg)](https://docs.rs/elvis/)
[![downloads](https://img.shields.io/crates/d/elvis.svg)](https://crates.io/crates/elvis)
[![Discord Chat](https://img.shields.io/discord/729613877184299019.svg?logo=discord&style=flat-square)](https://discord.gg/dxpefwy)
[![LICENSE](https://img.shields.io/crates/l/elvis.svg)](https://choosealicense.com/licenses/mit/)


Is anybody home?

As we know, `Elvis` is a famous rock star, both a famous rock song named `Calling Elvis` wrote by `Dire Straits` which inspired a unknown rock star writing down these chords.

For now, Elvis, the rock star will rise once again, **beyond** the internet —— truely your wasm web library, 🦀 + 🕸  => 💖

[The Evlis Book][1] mainly talks about the usage of [calling-elvis][2], if you want to use `low-level` api rusting the web, plz check out [elvis' rust doc][3].

## Goals 🎯

Writing web pages in **pure rust or javascript** using wasm bindings, **without** `jsx` or `any other` complex syntax, of course, **not** writing `html` nor `css` either.

## 21st Century Schizoid Magic 🍩

```
# Install elvis package manager
$ cargo install epm

# New your awesome-app
$ epm new my-awesome-app

# Start development server
$ cd my-awesome-app && epm dev
[INFO  warp::server] listening on http://0.0.0.0:3000
```

## Roll up for the Magical Mystery Tour! 🌈

🧙‍♂️ 🤹‍♂️ Here we go! Roll up, roll up for the mystery tour, the magical mystery tour is waiting to take you away! Hoping to take you away! Coming to take you away! Dying to take you away, take you today! 🛸

🎻 Let me take you down, cause I'm going to, **Strawberry Fields** 🧑‍🚀


```rust
//! src/lib.rs
use elvis::{
    prelude::*,
    layouts::Center,
    widgets::{Text, TextStyle},
};

#[elvis(page)]
struct Index;

impl LifeCycle<Center> for Index {
    fn new() -> Center {
        Center {
            child: Text {
                text: "Pink is the Pig!!!".into(),
                style: TextStyle::default(),
            }.into()
        }
    }
}
```

## Examples

+ [hello-world][hello-world-example]
+ [click][click-example]
+ [router][router-example]

## Community

Welcome to join us! Check our community channels [here][community].


## LICENSE

Heartbreak Hotel.

[1]: https://elvisjs.github.io/book
[2]: https://github.com/elvisjs/calling-elvis
[3]: https://docs.rs/elvis
[rust.cc]: https://rustcc.cn
[community]: https://elvisjs.github.io/the-elvis-book/community
[hello-world-example]: https://github.com/elvisjs/elvis/tree/master/examples/hello-world
[click-example]: https://github.com/elvisjs/elvis/tree/master/examples/click
[router-example]: https://github.com/elvisjs/elvis/tree/master/examples/router
