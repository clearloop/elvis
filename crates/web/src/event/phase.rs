/// Specifies whether the event listener is run during the capture or bubble phase.
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
