use super::phase::EventListenerPhase;
use web_sys::AddEventListenerOptions;

// This defaults passive to true to avoid performance issues in browsers:
// https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener#Improving_scrolling_performance_with_passive_listeners
thread_local! {
    pub static NEW_OPTIONS: AddEventListenerOptions = EventListenerOptions::default().to_js(false);
    pub static ONCE_OPTIONS: AddEventListenerOptions = EventListenerOptions::default().to_js(true);
}

/// Specifies options for [`EventListener::new_with_options`](struct.EventListener.html#method.new_with_options) and
/// [`EventListener::once_with_options`](struct.EventListener.html#method.once_with_options).
///
/// # Default
///
/// ```rust
/// # use gloo_events::{EventListenerOptions, EventListenerPhase};
/// #
/// EventListenerOptions {
///     phase: EventListenerPhase::Bubble,
///     passive: true,
/// }
/// # ;
/// ```
///
/// # Examples
///
/// Sets `phase` to `EventListenerPhase::Capture`, using the default for the rest:
///
/// ```rust
/// # use gloo_events::EventListenerOptions;
/// #
/// let options = EventListenerOptions::run_in_capture_phase();
/// ```
///
/// Sets `passive` to `false`, using the default for the rest:
///
/// ```rust
/// # use gloo_events::EventListenerOptions;
/// #
/// let options = EventListenerOptions::enable_prevent_default();
/// ```
///
/// Specifies all options:
///
/// ```rust
/// # use gloo_events::{EventListenerOptions, EventListenerPhase};
/// #
/// let options = EventListenerOptions {
///     phase: EventListenerPhase::Capture,
///     passive: false,
/// };
/// ```
#[derive(Debug, Clone, Copy)]
pub struct EventListenerOptions {
    /// The phase that the event listener should be run in.
    pub phase: EventListenerPhase,

    /// If this is `true` then performance is improved, but it is not possible to use
    /// [`event.prevent_default()`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.prevent_default).
    ///
    /// If this is `false` then performance might be reduced, but now it is possible to use
    /// [`event.prevent_default()`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.prevent_default).
    ///
    /// You can read more about the performance costs
    /// [here](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener#Improving_scrolling_performance_with_passive_listeners).
    pub passive: bool,
}

impl EventListenerOptions {
    /// Returns an `EventListenerOptions` with `phase` set to `EventListenerPhase::Capture`.
    ///
    /// This is the same as:
    ///
    /// ```rust
    /// # use gloo_events::{EventListenerOptions, EventListenerPhase};
    /// #
    /// EventListenerOptions {
    ///     phase: EventListenerPhase::Capture,
    ///     ..Default::default()
    /// }
    /// # ;
    /// ```
    #[inline]
    pub fn run_in_capture_phase() -> Self {
        Self {
            phase: EventListenerPhase::Capture,
            ..Self::default()
        }
    }

    /// Returns an `EventListenerOptions` with `passive` set to `false`.
    ///
    /// This is the same as:
    ///
    /// ```rust
    /// # use gloo_events::EventListenerOptions;
    /// #
    /// EventListenerOptions {
    ///     passive: false,
    ///     ..Default::default()
    /// }
    /// # ;
    /// ```
    #[inline]
    pub fn enable_prevent_default() -> Self {
        Self {
            passive: false,
            ..Self::default()
        }
    }

    /// To javascript listener option
    #[inline]
    pub fn to_js(&self, once: bool) -> AddEventListenerOptions {
        let mut options = AddEventListenerOptions::new();

        options.capture(self.phase.is_capture());
        options.once(once);
        options.passive(self.passive);

        options
    }
}

impl Default for EventListenerOptions {
    #[inline]
    fn default() -> Self {
        Self {
            phase: Default::default(),
            passive: true,
        }
    }
}
