use crate::{
    default_th_sorting_style, ChangeEvent, ColumnSort, EventHandler, TableHeadEvent, TableRow,
};
use leptos::{html::Tbody, prelude::*};
use std::fmt::Debug;

pub trait TableRender<Row, Err: Debug = String>
where
    Row: TableRow + Clone + Sync + Send,
{
    /// Renderer function for the table head. Defaults to [`DefaultTableHeadRenderer`]. For a full example see the
    /// [custom_renderers_svg example](https://github.com/Synphonyte/leptos-struct-table/blob/master/examples/custom_renderers_svg/src/main.rs).
    fn thead_cell_renderer<F>(
        &self,
        class: Signal<String>,
        inner_class: String,
        index: usize,
        sort_priority: Signal<Option<usize>>,
        sort_direction: Signal<ColumnSort>,
        on_click: F,
        children: Children,
    ) -> impl IntoView
    where
        F: Fn(TableHeadEvent) + 'static;
    // /// Renderer function for the table body. Defaults to [`DefaultTableBodyRenderer`]. For a full example see the
    // /// [custom_renderers_svg example](https://github.com/Synphonyte/leptos-struct-table/blob/master/examples/custom_renderers_svg/src/main.rs).
    ///
    /// # Arguments
    ///
    /// * `content` - The content of the renderer. It's like the children of this view.
    /// * `class` - The class attribute that is passed to the root element
    /// * `node_ref` - The `NodeRef` referencing the root tbody element.
    ///
    /// This render function has to render exactly one root element.
    fn tbody_renderer(
        &self,
        content: AnyView,
        class: Signal<String>,
        node_ref: NodeRef<Tbody>,
    ) -> impl IntoView;
    // /// Renderer function for the table head row. Defaults to [`DefaultTableHeadRowRenderer`]. For a full example see the
    // /// [custom_renderers_svg example](https://github.com/Synphonyte/leptos-struct-table/blob/master/examples/custom_renderers_svg/src/main.rs).
    fn thead_row_renderer(&self, content: AnyView, class: Signal<String>) -> impl IntoView;
    fn thead_renderer(&self, content: AnyView, class: Signal<String>) -> impl IntoView;

    // /// The row renderer. Defaults to [`DefaultTableRowRenderer`]. For a full example see the
    // /// [custom_renderers_svg example](https://github.com/Synphonyte/leptos-struct-table/blob/master/examples/custom_renderers_svg/src/main.rs).
    fn row_renderer(
        &self,
        class: Signal<String>,
        row: Row,
        index: usize,
        selected: Signal<bool>,
        on_select: EventHandler<web_sys::MouseEvent>,
        on_change: EventHandler<ChangeEvent<Row>>,
    ) -> impl IntoView;

    // /// The row renderer for when that row is currently being loaded.
    // /// Defaults to [`DefaultLoadingRowRenderer`]. For a full example see the
    // /// [custom_renderers_svg example](https://github.com/Synphonyte/leptos-struct-table/blob/master/examples/custom_renderers_svg/src/main.rs).
    fn loading_row_renderer(
        &self,
        class: Signal<String>,
        get_cell_class: Callback<usize, String>,
        get_inner_cell_class: Callback<usize, String>,
        index: usize,
        col_count: usize,
    ) -> impl IntoView;

    // /// The row renderer for when that row failed to load.
    // /// Defaults to [`DefaultErrorRowRenderer`]. For a full example see the
    // /// [custom_renderers_svg example](https://github.com/Synphonyte/leptos-struct-table/blob/master/examples/custom_renderers_svg/src/main.rs).
    fn error_row_renderer(&self, err: String, index: usize, col_count: usize) -> impl IntoView;
    // /// The row placeholder renderer. Defaults to [`DefaultRowPlaceholderRenderer`].
    // /// This is used in place of rows that are not shown
    // /// before and after the currently visible rows.
    fn row_placeholder_renderer(&self, height: Signal<f64>) -> impl IntoView;

    // #[prop(optional, into)]
    // row_placeholder_renderer: RowPlaceholderRendererFn,
}

#[derive(Clone)]
pub struct DefaultTableRenderer;

impl<Row, Err> TableRender<Row, Err> for DefaultTableRenderer
where
    Row: TableRow + Clone + Sync + Send,
    Err: Debug,
{
    fn thead_cell_renderer<F>(
        &self,
        class: Signal<String>,
        inner_class: String,
        index: usize,
        sort_priority: Signal<Option<usize>>,
        sort_direction: Signal<ColumnSort>,
        on_click: F,
        children: Children,
    ) -> impl IntoView
    where
        F: Fn(TableHeadEvent) + 'static,
    {
        let style = default_th_sorting_style(sort_priority, sort_direction);

        view! {
            <th class=class
                on:click=move |mouse_event| on_click(TableHeadEvent {
                    index,
                    mouse_event,
                })
                style=style
            >
                <span class=inner_class>
                    {children()}
                </span>
            </th>
        }
    }

    /// Default tbody renderer. Please note that this is **NOT** a `#[component]`.
    ///
    /// # Arguments
    ///
    /// * `content` - The content of the renderer. It's like the children of this view.
    /// * `class` - The class attribute that is passed to the root element
    /// * `node_ref` - The `NodeRef` referencing the root tbody element.
    ///
    /// This render function has to render exactly one root element.
    fn tbody_renderer(
        &self,
        content: AnyView,
        class: Signal<String>,
        node_ref: NodeRef<Tbody>,
    ) -> impl IntoView {
        let tbody_ref = NodeRef::<Tbody>::new();
        // tbody_ref.try_with_untracked(move |e| e.as_ref().map(|f| node_ref.mount(&f, None)));

        view! { <tbody class=class node_ref=tbody_ref>{content}</tbody> }
    }

    fn thead_row_renderer(&self, content: AnyView, class: Signal<String>) -> impl IntoView {
        view! {
            <tr class = class>{
                content
            }</tr>
        }
    }

    fn thead_renderer(&self, content: AnyView, class: Signal<String>) -> impl IntoView {
        view! {
            <thead class = class>{
                content
            }</thead>
        }
    }

    /// The default table row renderer. Uses the `<tr>` element. Please note that this
    /// is **NOT** a `#[component]`.
    #[allow(unused_variables)]
    fn row_renderer(
        &self,
        // The class attribute for the row element. Generated by the classes provider.
        class: Signal<String>,
        // The row to render.
        row: Row,
        // The index of the row. Starts at 0 for the first body row.
        index: usize,
        // The selected state of the row. True, when the row is selected.
        selected: Signal<bool>,
        // Event handler callback when this row is selected
        on_select: EventHandler<web_sys::MouseEvent>,
        // Event handler callback for changes
        on_change: EventHandler<ChangeEvent<Row>>,
    ) -> impl IntoView {
        view! {
            <tr class=class on:click=move |mouse_event| on_select.run(mouse_event)>
                {row.render_row(index, on_change)}
            </tr>
        }
    }

    #[allow(unused_variables, unstable_name_collisions)]
    fn loading_row_renderer(
        &self,
        class: Signal<String>,
        get_cell_class: Callback<usize, String>,
        get_inner_cell_class: Callback<usize, String>,
        index: usize,
        col_count: usize,
    ) -> impl IntoView {
        view! {
            <tr class=class>
                {
                    (0..col_count).map(|col_index| view! {
                        <td class=get_cell_class.run(col_index)>
                            <div class=get_inner_cell_class.run(col_index)></div>
                            " "
                        </td>
                    }).collect_view()
                }
            </tr>
        }
    }

    #[allow(unused_variables, unstable_name_collisions)]
    fn error_row_renderer(&self, err: String, index: usize, col_count: usize) -> impl IntoView {
        view! { <tr><td colspan=col_count>{err}</td></tr> }
    }

    fn row_placeholder_renderer(&self, height: Signal<f64>) -> impl IntoView {
        view! { <tr style:height=move || format!("{}px", height.get()) style="display: block"></tr> }
    }
}
