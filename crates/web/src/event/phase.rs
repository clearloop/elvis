/// Specifies whether the event listener is run during the capture or bubble phase.
///
/// The official specification has [a good explanation](https://www.w3.org/TR/DOM-Level-3-Events/#event-flow)
/// of capturing vs bubbling.
///
/// # Default
///
/// ```rust
/// # use gloo_events::EventListenerPhase;
/// #
/// EventListenerPhase::Bubble
/// # ;
/// ```
#[derive(Debug, Clone, Copy)]
pub enum EventListenerPhase {
    #[allow(missing_docs)]
    Bubble,

    #[allow(missing_docs)]
    Capture,
}

impl EventListenerPhase {
    /// Check if is capture
    #[inline]
    pub fn is_capture(&self) -> bool {
        match self {
            EventListenerPhase::Bubble => false,
            EventListenerPhase::Capture => true,
        }
    }
}

impl Default for EventListenerPhase {
    #[inline]
    fn default() -> Self {
        EventListenerPhase::Bubble
    }
}
