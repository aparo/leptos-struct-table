use std::ops::Deref;

use leptos::html::{Body, ElementType};
use leptos::prelude::*;
use leptos_use::core::{ElementMaybeSignal, IntoElementMaybeSignal};
use leptos_use::use_document;
use std::fmt::Debug;
// use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
#[derive(Clone)]
pub struct ScrollContainer<E>(pub Option<NodeRef<E>>)
where
    E: ElementType + 'static,
    E::Output: 'static;
// E::Output: JsCast + Clone + Deref<Target = web_sys::HtmlElement> + 'static;

// unsafe impl Send for ScrollContainer {}
// unsafe impl Sync for ScrollContainer {}

impl<E> Default for ScrollContainer<E>
where
    E: ElementType + 'static,
    E::Output: JsCast + Clone + Deref<Target = web_sys::HtmlElement> + 'static,
{
    fn default() -> Self {
        let node_ref = use_document().body().as_ref().map(|w| {
            let element: web_sys::Element = w.unchecked_ref::<web_sys::Element>().clone();
            let node_ref = NodeRef::<E>::new();
            node_ref
        });

        Self(node_ref)
    }
}
// impl From<web_sys::Element> for ScrollContainer {
//     fn from(element: web_sys::Element) -> Self {
//         Self(Some(element.clone()))
//     }
// }

// impl From<Option<web_sys::Element>> for ScrollContainer {
//     fn from(element: Option<web_sys::Element>) -> Self {
//         Self(element.clone())
//     }
// }

impl<T> From<NodeRef<T>> for ScrollContainer<T>
where
    T: ElementType + 'static,
    T::Output: 'static,
{
    fn from(node_ref: NodeRef<T>) -> Self {
        Self(Some(node_ref))
    }
}

// impl<T> From<NodeRef<T>> for ScrollContainer<T>
// where
//     T: ElementType + 'static,
//     T::Output: JsCast + Clone + Deref<Target = web_sys::HtmlElement> + 'static,
// {
//     fn from(node_ref: NodeRef<T>) -> Self {
//         Self(node_ref.try_read_untracked().map(|el| {
//             let el: &web_sys::Element = &el.into_any();
//             el.clone()
//         }))
//     }
// }

// impl From<&str> for ScrollContainer {
//     fn from(selector: &str) -> Self {
//         let selector = selector.to_owned();

//         Self(Signal::derive(move || {
//             use_document().query_selector(&selector).unwrap_or_default()
//         }))
//     }
// }

// impl<M> From<ScrollContainer> for ElementMaybeSignal<web_sys::Element, M> {
//     fn from(scroll_container: ScrollContainer) -> Self {
//         scroll_container.0.into()
//     }
// }
