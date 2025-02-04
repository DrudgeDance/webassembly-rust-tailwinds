use leptos::*;
use leptos::ev::MouseEvent;
use leptos::html::Div;
use leptos::wasm_bindgen::JsCast;
use web_sys::{Node, Element};

pub fn handle_click_outside(event: MouseEvent, container_ref: NodeRef<Div>, set_is_open: WriteSignal<bool>) {
    if let Some(container) = container_ref.get() {
        if let Some(target) = event.target() {
            if let Some(element) = target.dyn_ref::<Element>() {
                let target_node: &Node = element.as_ref();
                if !container.contains(Some(target_node)) {
                    set_is_open.set(false);
                }
            }
        }
    }
} 