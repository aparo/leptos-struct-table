use leptos::ev::MouseEvent;
use leptos::*;

/// Event emitted when a table row is clicked.
#[derive(Debug)]
pub struct TableRowEvent<K: 'static> {
    /// The key of the row. Value of the field of the struct with the `#[table(key)]` attribute.
    pub key: K,
    /// The index of the row. Starts at 0 for the first row.
    pub index: usize,
    /// The mouse event that triggered the event.
    pub mouse_event: MouseEvent,
}

/// The default table row renderer. Uses the `<tr>` element.
#[allow(unused_variables)]
#[component]
pub fn DefaultTableRowRenderer<K, F>(
    cx: Scope,
    /// The class attribute for the row element. Generated by the classes provider.
    #[prop(into)]
    class: MaybeSignal<String>,
    /// The index of the row. Starts at 0 for the first body row. The header row always has index 0 as well.
    #[prop(into)]
    key: K,
    /// The index of the row. Starts at 0 for the first body row.
    index: usize,
    /// The selected state of the row. True, when the row is selected.
    #[prop(into)]
    selected: Signal<bool>,
    /// The event handler for the click event. Has to be called with [`TableRowEvent`].
    on_click: F,
    children: Children,
) -> impl IntoView
where
    F: Fn(TableRowEvent<K>) + 'static,
    K: Clone + 'static,
{
    view! { cx,
        <tr class=class on:click=move |mouse_event| on_click(TableRowEvent {
            key: key.clone(),
            index,
            mouse_event,
        })>
            {children(cx)}
        </tr>
    }
}