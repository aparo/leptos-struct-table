use leptos::html::Tbody;
use leptos::prelude::*;
use leptos::tachys::html::node_ref::NodeRefContainer;
use leptos::tachys::view::any_view::AnyView;
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
) -> impl IntoView {
    let tbody_ref = NodeRef::<Tbody>::new();
    tbody_ref.try_with_untracked(move |e| e.as_ref().map(|f| node_ref.load(&f)));

    view! { <tbody class=class node_ref=tbody_ref>{content}</tbody> }
}
