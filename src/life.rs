/// Lifecycle
///
/// 1. `create()` calling when constructs
/// 2. `update()` calling after `set_state()`
/// 3. `render()` calling after `create()` and `update()`
/// 4. `dispose()` calling after deleting tree
/// 5. `set_state()` calling by users
pub trait LifeCycle<T> {
    fn create(); //
    fn update(); // calling after set_state
    fn render() -> T; // calling when render() occurs, every after crate
    fn dispose(); //
    fn set_state();
}
