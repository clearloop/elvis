# Gesture

`Event` and `Gesture` are out of wasm, this is not cool, but still awesome, `Elvis` implement these two trouble maker in `Typescript`, because `wasm-bindgen` doesn't support passing javascript `Object` to rust for now, BUT, it will be totally cool one day.

## GestureDetector
```js
class GestureDetector {
  onTap: function(e: Event): any;
}
```
