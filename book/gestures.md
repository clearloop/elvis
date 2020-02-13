# Gestures

> **Note**: In Progress.

`Event` and `Gesture` are out of wasm, this is not cool, but still awesome, `Elvis` implements these two trouble maker in `Typescript`, because `wasm-bindgen` doesn't support passing javascript `Object` to rust for now, BUT, it will be totally cool one day.

## GestureDetector
```rust
pub trait GestureDetector {
  fn on_double_tap(e: Event);
  fn on_force_press_end(e: Event);
  fn on_force_press_peak(e: Event);
  fn on_force_press_start(e: Event);
  fn on_force_press_update(e: Event);
  fn on_horizontal_drag_cancel(e: Event);
  fn on_horizontal_drag_down(e: Event);
  fn on_horizontal_drag_end(e: Event);
  fn on_horizontal_drag_start(e: Event);
  fn on_horizontal_drag_update(e: Event);
  fn on_long_press(e: Event);
  fn on_long_press_end(e: Event);
  fn on_long_press_move_update(e: Event);
  fn on_long_press_start(e: Event);
  fn on_long_press_up(e: Event);
  fn on_pan_cancel(e: Event);
  fn on_pan_down(e: Event);
  fn on_pan_end(e: Event);
  fn on_pan_start(e: Event);
  fn on_pan_update(e: Event);
  fn on_scale_end(e: Event);
  fn on_scale_start(e: Event);
  fn on_scale_update(e: Event);
  fn on_secondary_tap_cancel(e: Event);
  fn on_secondary_tap_down(e: Event);
  fn on_secondary_tap_up(e: Event);
  fn on_vertical_drag_cancel(e: Event);
  fn on_vertical_drag_down(e: Event);
  fn on_vertical_drag_end(e: Event);
  fn on_vertical_drag_start(e: Event);
  fn on_vertical_drag_update(e: Event);
}
```

Are we still cool now?

`Gesture` in `calling-elvis` implements with typescript, but `elvis` still keeps these apis, so we still can rust the web happily with `Elvis` without confuse.
