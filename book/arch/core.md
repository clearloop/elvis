# core

The [core][core] library abstracts the UI without invoking into any platform.

We don't need to know how it works if you just want to develop Apps using
ElvisJS, because the [elvis][elvis] library provides all we need in the
core library.

## Node

The common UI of ElvisJS are composed with nodes which are defined in this core
library, just the frame and transform stuffs.


## UI

### Attribute && Class

Attribute and Class are defined in this library as well, both of them are served for
the `Node`, attribute controls the features of Node, and the class affect on the style
of Node.

### Values

All static values are defined in this library, for example, `Color`, `Unit`.

### Layout

Finally, we are reaching the layout part. The difference of ElvisJS and other UI library
in rust/wasm is ElvisJS provides UI itself, not just give you guys a parser parsing rust
to Javascript or html.

The layout part is only defined in the core library, for example, the [web][web] library
is just a re-export of the layout part of this core library, migrating the ElvisJS UI
into browser.

## Lifecycle

The lifecycle of ElvisJS defines like 

| LifeCycle | Description                                         |
|-----------|-----------------------------------------------------|
| Create    | Triggers while creating the component in node tree  |
| Render    | Triggers while rendering the component in node tree |
| Update    | Triggers when the state machine changes             |
| Drop      | Triggers while droping the component                |


## State machine

State machine of Elvisjs equipped for user-defined components, the fields of the struct
are the storage of the state machine, which we can modify in gestures.

[web]: https://github.com/elvisjs/elvis/tree/master/crates/web
[core]: https://github.com/elvisjs/elvis/tree/master/crates/core
[elvis]: https://github.com/elvisjs/elvis
