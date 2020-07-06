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

🎻 Let me take you down, cause I'm going to, **Strawberry Fields** 🧑‍🚀


```rust
//! src/lib.rs
use elvis::{
    prelude::*,
    widgets::{layouts::Center, Text, TextStyle},
};

#[elvis]
struct Index;

impl LifeCycle<Center> for Index {
    fn create(&self) -> Center {
        Center {
            child: Text {
                text: "Pink is the Pig!!!".into(),
                style: TextStyle::default(),
            }
            .into(),
        }
    }
}
```

## 21st Century Schizoid Magic 🍩

#### Install epm

```
$ cargo install epm
$ epm
epm 0.1.4

USAGE:
    epm <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    build    
    dev      Start development server
    help     Prints this message or the help of the given subcommand(s)
    init     Create a new elvis package in an existing directory
    new      Create a new elvis package
```

#### New elvis app

```
$ epm new my-awesome-app
```

#### Start development server

```
$ cd my-awesome-app && epm dev
[INFO  warp::server] listening on http://0.0.0.0:3000
```

## Community

Welcome to join us! Check our community channels [here][community].


## LICENSE

Heartbreak Hotel.

[1]: https://elvisjs.github.io/the-elvis-book
[2]: https://github.com/elvisjs/calling-elvis
[3]: https://docs.rs/elvis
[rust.cc]: https://rustcc.cn
[community]: https://elvisjs.github.io/the-elvis-book/community
