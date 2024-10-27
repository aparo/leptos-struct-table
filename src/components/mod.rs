mod cell;
mod renderer;
mod renderer_fn;
mod table_content;
mod tbody;
mod thead;
pub use cell::*;
use leptos::{prelude::*, tachys::view::any_view::AnyView};
pub use renderer::*;
pub use table_content::*;
pub use tbody::*;
pub use thead::*;

#[macro_export]
macro_rules! wrapper_render_fn {
    (
        #[$doc_name:meta]
        $name:ident,
        $tag:ident,
        $(#[$additional_doc:meta])*
    ) => {
        /// Default
        #[$doc_name]
        /// renderer. Please note that this is **NOT** a `#[component]`.
        ///
        /// # Arguments
        ///
        /// * `content` - The content of the renderer. It's like the children of this view.
        /// * `class` - The class attribute that is passed to the root element
        $(#[$additional_doc])*
        #[allow(non_snake_case)]
        pub fn $name<T>(content: leptos::prelude::View<T>, class: Signal<String>)-> impl IntoView
        where
            T: Sized + Render + RenderHtml + Send,
          {
            view! {
                <$tag class=class>
                    {content}
                </$tag>
            }
        }
    };
}
