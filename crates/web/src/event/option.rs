use super::phase::EventListenerPhase;
use web_sys::AddEventListenerOptions;

// This defaults passive to true to avoid performance issues in browsers:
// https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener#Improving_scrolling_performance_with_passive_listeners
thread_local! {
    pub static NEW_OPTIONS: AddEventListenerOptions = EventListenerOptions::default().to_js(false);
    pub static ONCE_OPTIONS: AddEventListenerOptions = EventListenerOptions::default().to_js(true);
}

/// Specifies options for `EventListener`
#[derive(Debug, Clone, Copy)]
pub struct EventListenerOptions {
    /// The phase that the event listener should be run in.
    pub phase: EventListenerPhase,

    /// If this is `true` then performance is improved
    pub passive: bool,
}

impl EventListenerOptions {
    /// Returns an `EventListenerOptions` with `phase` set to `EventListenerPhase::Capture`.
    #[inline]
    pub fn run_in_capture_phase() -> Self {
        Self {
            phase: EventListenerPhase::Capture,
            ..Self::default()
        }
    }

    /// Returns an `EventListenerOptions` with `passive` set to `false`.
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
