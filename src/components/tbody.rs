use leptos::{html::Tbody, prelude::*};

/// Default tbody renderer. Please note that this is **NOT** a `#[component]`.
///
/// # Arguments
///
/// * `content` - The content of the renderer. It's like the children of this view.
/// * `class` - The class attribute that is passed to the root element
/// * `node_ref` - The `NodeRef` referencing the root tbody element.
///
/// This render function has to render exactly one root element.
#[allow(non_snake_case)]
pub fn DefaultTableBodyRenderer(
    content: AnyView,
    class: Signal<String>,
    node_ref: NodeRef<Tbody>,
) -> AnyView {
    view! {
        <tbody class=class node_ref=node_ref>
            {content}
        </tbody>
    }
    .into_any()
}
