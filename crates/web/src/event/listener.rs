use super::{
    option::{EventListenerOptions, NEW_OPTIONS, ONCE_OPTIONS},
    phase::EventListenerPhase,
};
use std::borrow::Cow;
use wasm_bindgen::{closure::Closure, JsCast, UnwrapThrowExt};
use web_sys::{AddEventListenerOptions, Event, EventTarget};

/// RAII type which is used to manage DOM event listeners.
///
/// When the `EventListener` is dropped, it will automatically deregister the event listener and
/// clean up the closure's memory.
#[derive(Debug)]
#[must_use = "event listener will never be called after being dropped"]
pub struct EventListener {
    target: EventTarget,
    event_type: Cow<'static, str>,
    callback: Option<Closure<dyn FnMut(&Event)>>,
    phase: EventListenerPhase,
}

impl EventListener {
    /// New Raw Event
    #[inline]
    pub fn raw_new(
        target: &EventTarget,
        event_type: Cow<'static, str>,
        callback: Closure<dyn FnMut(&Event)>,
        options: &AddEventListenerOptions,
        phase: EventListenerPhase,
    ) -> Self {
        target
            .add_event_listener_with_callback_and_add_event_listener_options(
                &event_type,
                callback.as_ref().unchecked_ref(),
                options,
            )
            .unwrap_throw();

        Self {
            target: target.clone(),
            event_type,
            callback: Some(callback),
            phase,
        }
    }

    /// Registers an event listener on an [`EventTarget`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.EventTarget.html).
    ///
    /// For specifying options, there is a corresponding [`EventListener::new_with_options`](#method.new_with_options) method.
    ///
    /// If you only need the event to fire once, you can use [`EventListener::once`](#method.once) instead,
    /// which accepts an `FnOnce` closure.
    #[inline]
    pub fn new<S, F>(target: &EventTarget, event_type: S, callback: F) -> Self
    where
        S: Into<Cow<'static, str>>,
        F: FnMut(&Event) + 'static,
    {
        let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(&Event)>);

        NEW_OPTIONS.with(move |options| {
            Self::raw_new(
                target,
                event_type.into(),
                callback,
                options,
                EventListenerPhase::Bubble,
            )
        })
    }

    /// This is exactly the same as [`EventListener::new`](#method.new), except the event will only fire once,
    /// and it accepts `FnOnce` instead of `FnMut`.
    ///
    /// For specifying options, there is a corresponding [`EventListener::once_with_options`](#method.once_with_options) method.
    #[inline]
    pub fn once<S, F>(target: &EventTarget, event_type: S, callback: F) -> Self
    where
        S: Into<Cow<'static, str>>,
        F: FnOnce(&Event) + 'static,
    {
        let callback = Closure::once(callback);

        ONCE_OPTIONS.with(move |options| {
            Self::raw_new(
                target,
                event_type.into(),
                callback,
                options,
                EventListenerPhase::Bubble,
            )
        })
    }

    /// Registers an event listener on an [`EventTarget`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.EventTarget.html).
    ///
    /// It is recommended to use [`EventListener::new`](#method.new) instead, because it has better performance, and it is more convenient.
    ///
    /// If you only need the event to fire once, you can use [`EventListener::once_with_options`](#method.once_with_options) instead,
    /// which accepts an `FnOnce` closure.
    #[inline]
    pub fn new_with_options<S, F>(
        target: &EventTarget,
        event_type: S,
        options: EventListenerOptions,
        callback: F,
    ) -> Self
    where
        S: Into<Cow<'static, str>>,
        F: FnMut(&Event) + 'static,
    {
        let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(&Event)>);

        Self::raw_new(
            target,
            event_type.into(),
            callback,
            &options.to_js(false),
            options.phase,
        )
    }

    /// This is exactly the same as [`EventListener::new_with_options`](#method.new_with_options), except the event will only fire once,
    /// and it accepts `FnOnce` instead of `FnMut`.
    ///
    /// It is recommended to use [`EventListener::once`](#method.once) instead, because it has better performance, and it is more convenient.
    #[inline]
    pub fn once_with_options<S, F>(
        target: &EventTarget,
        event_type: S,
        options: EventListenerOptions,
        callback: F,
    ) -> Self
    where
        S: Into<Cow<'static, str>>,
        F: FnOnce(&Event) + 'static,
    {
        let callback = Closure::once(callback);

        Self::raw_new(
            target,
            event_type.into(),
            callback,
            &options.to_js(true),
            options.phase,
        )
    }

    /// Keeps the `EventListener` alive forever, so it will never be dropped.
    ///
    /// This should only be used when you want the `EventListener` to last forever, otherwise it will leak memory!
    #[inline]
    pub fn forget(mut self) {
        // take() is necessary because of Rust's restrictions about Drop
        // This will never panic, because `callback` is always `Some`
        self.callback.take().unwrap_throw().forget()
    }

    /// Returns the `EventTarget`.
    #[inline]
    pub fn target(&self) -> &EventTarget {
        &self.target
    }

    /// Returns the event type.
    #[inline]
    pub fn event_type(&self) -> &str {
        &self.event_type
    }

    /// Returns the callback.
    #[inline]
    pub fn callback(&self) -> &Closure<dyn FnMut(&Event)> {
        // This will never panic, because `callback` is always `Some`
        self.callback.as_ref().unwrap_throw()
    }

    /// Returns whether the event listener is run during the capture or bubble phase.
    ///
    /// The official specification has [a good explanation](https://www.w3.org/TR/DOM-Level-3-Events/#event-flow)
    /// of capturing vs bubbling.
    #[inline]
    pub fn phase(&self) -> EventListenerPhase {
        self.phase
    }
}

impl Drop for EventListener {
    #[inline]
    fn drop(&mut self) {
        // This will only be None if forget() was called
        if let Some(callback) = &self.callback {
            self.target
                .remove_event_listener_with_callback_and_bool(
                    self.event_type(),
                    callback.as_ref().unchecked_ref(),
                    self.phase.is_capture(),
                )
                .unwrap_throw();
        }
    }
}
