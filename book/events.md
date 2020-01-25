# Events

Follows MDN docs [Events][1]

`Events` is kind like a third-part plugin in `Elvis`, it implelements both in `rust` and `typescript`.

## Javascript
```js
/* Events */
import { Component, Elvis, Events } from "calling-elvis";
const { Text } = Elvis;

class myClickAbleText extends Component {
  state = {
    msg: "",
  }

  consturstor(msg: string) {
    this.msg = msg;
  }

  click() {
    window.alert(`{}, roger that!`);
  }

  render() {
    return Text("Click me plz.");
  }
}
```

Elvis `Events` are outside of native components, we writing them straight just like writing `lifecycles`. `Elvis` strongly recommands our folks writing persnal components even publishing them to `github`.

## Rust

```rust
pub enum Events {
  Cancel,
  Error,
  Scroll,
  Select,
  Show,
  Wheel,
  // Clipborad events
  Copy,
  Cut,
  Paste,
  // Composition events
  CompositionEnd,
  CompositionStart,
  CompositionUpdate,
  // Focus events
  Blur, 
  Focus,
  Focusin,
  Focusout,
  // Fullscreen events
  KeyDown,
  KeyPress,
  KeyUp,
  // Mouse events
  AuxClick,
  Click,
  ContextMenu,
  DbClick,
  MouseDown,
  MouseEnter,
  MouseLeave,
  MouseMove,
  MouseOut,
  MouseOver,
  MouseUp,
  WebkitMouseForceChanged,
  WebkitMouseForceDown,
  WebkitMouseForceWillBegin,
  WebkitMouseForceUp,
  // Touch events,
  TouchCancel,
  TouchEnd,
  TouchMove,
  TouchStart
}
```

Events enum list all events in MDN events, the implementation in rust depends on `gloo`.


[1]: https://developer.mozilla.org/en-US/docs/Web/API/Element#Events
