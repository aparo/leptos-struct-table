//! Support for [uuid::Uuid] type.

use leptos::*;

/// CellValue implementation for uuid for uuid to work with the TableRow derive
/// ``` 
/// # use leptos_struct_table::*;
/// # use leptos::*;
/// 
/// #[derive(TableRow, Clone)]
/// #[table]
/// struct SomeStruct {
///     my_field: ::uuid::Uuid
/// }
/// ```
impl crate::CellValue for uuid::Uuid {
    fn render_value(self, _options: &crate::RenderOptions) -> impl IntoView {
        view! {
            <>{self.to_string()}</>
        }
    }
}