# Calling Elvis

![Rust](https://github.com/clearloop/leetcode-cli/workflows/Rust/badge.svg)
[![crate](https://img.shields.io/crates/v/elvis.svg)](https://crates.io/crates/elvis)
[![doc](https://img.shields.io/badge/current-docs-brightgreen.svg)](https://docs.rs/elvis/)
[![downloads](https://img.shields.io/crates/d/elvis.svg)](https://crates.io/crates/elvis)
[![Discord Chat](https://img.shields.io/discord/729613877184299019.svg?logo=discord&style=flat-square)](https://discord.gg/dxpefwy)
[![LICENSE](https://img.shields.io/crates/l/elvis.svg)](https://choosealicense.com/licenses/mit/)

Is anybody home? [The Evlis Book][1] mainly talks about the usage of [elvis][2], here is our [roadmap][roadmap], come and [join][community] us !~


## Help Wanted
 
+ [ ] A static http/websocket development server [#16][#16]
+ [ ] Documentation of ElvisJS [#65][#65]
+ [ ] A Markdown parser using ElvisJS [#66][#66]
+ [ ] Optimize the State machine [#67][#67]

## Getting Started

```
# Install elvis package manager
$ cargo install epm

# New your awesome-app
$ epm new my-awesome-app

# Start development server
$ cd my-awesome-app && epm dev
[INFO  warp::server] listening on http://0.0.0.0:3000
```


## Hello, World!

```rust
//! src/lib.rs
use elvis::{
    prelude::*,
    widgets::{layouts::Center, Text},
};

#[page]
struct Index;

impl LifeCycle for Index {
    fn create(&self) -> Node {
        Center::with(Text::with("Hello, World!")).into()
    }
}
```

## Examples

+ [hello-world][hello-world-example]
+ [click][click-example]
+ [router][router-example]
+ [todo-mvc][todo-mvc]
+ [website][website]



## LICENSE

Heartbreak Hotel.

[#16]: https://github.com/elvisjs/elvis/issues/16
[#65]: https://github.com/elvisjs/elvis/issues/65
[#66]: https://github.com/elvisjs/elvis/issues/66
[#67]: https://github.com/elvisjs/elvis/issues/67
[1]: https://elvisjs.github.io/book
[2]: https://docs.rs/elvis
[community]: https://elvisjs.github.io/book/community
[hello-world-example]: https://github.com/elvisjs/elvis/tree/master/examples/hello-world
[click-example]: https://github.com/elvisjs/elvis/tree/master/examples/click
[router-example]: https://github.com/elvisjs/elvis/tree/master/examples/router
[todo-mvc]: https://github.com/elvisjs/elvis/tree/master/examples/todo-mvc
[website]: https://github.com/elvisjs/elvis/tree/master/examples/website
[roadmap]: https://github.com/elvisjs/elvis/milestones
