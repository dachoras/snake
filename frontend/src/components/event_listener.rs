use wasm_bindgen::prelude::*;

pub struct EventListener {
    target: web_sys::EventTarget,
    event_type: &'static str,
    closure: Option<wasm_bindgen::closure::Closure<dyn FnMut(web_sys::Event)>>,
}

impl EventListener {
    pub fn new<F>(target: &web_sys::EventTarget, event_type: &'static str, mut callback: F) -> Self
    where
        F: FnMut(web_sys::Event) + 'static,
    {
        let closure = Closure::wrap(Box::new(move |e| callback(e)) as Box<dyn FnMut(web_sys::Event)>);
        target
            .add_event_listener_with_callback(event_type, closure.as_ref().unchecked_ref())
            .unwrap();
        Self {
            target: target.clone(),
            event_type,
            closure: Some(closure),
        }
    }
}

impl Drop for EventListener {
    fn drop(&mut self) {
        if let Some(closure) = self.closure.take() {
            let _ = self.target.remove_event_listener_with_callback(
                self.event_type,
                closure.as_ref().unchecked_ref(),
            );
        }
    }
}
