#![allow(unused_variables)]

#[cfg(feature = "chrono")]
mod chrono;
#[cfg(feature = "chrono")]
pub use self::chrono::*;

use core::fmt::Display;
use leptos::*;

/// The default cell renderer. Uses the `<td>` element.
#[component]
pub fn DefaultTableCellRenderer<T>(
    cx: Scope,
    /// The class attribute for the cell element. Generated by the classes provider.
    #[prop(into)]
    class: MaybeSignal<String>,
    /// The value to display.
    #[prop(into)]
    value: MaybeSignal<T>,
    /// The index of the column. Starts at 0.
    index: usize,
) -> impl IntoView
where
    T: IntoView + Clone + 'static,
{
    view! { cx,
        <td class=class>{value}</td>
    }
}

/// The default number cell renderer. Uses the `<td>` element.
#[component]
pub fn DefaultNumberTableCellRenderer<T>(
    cx: Scope,
    /// The class attribute for the cell element. Generated by the classes provider.
    #[prop(into)]
    class: MaybeSignal<String>,
    /// The value to display.
    #[prop(into)]
    value: MaybeSignal<T>,
    /// The index of the column. Starts at 0.
    index: usize,
    /// The number of digits to display after the decimal point. Provided by the `#[table(format(precision=X))]` attribute of the field.
    #[prop(optional)]
    precision: Option<usize>,
) -> impl IntoView
where
    T: Display + Clone + 'static,
{
    let text = create_memo(cx, move |_| match precision {
        Some(precision) => format!("{:.precision$}", value()),
        None => format!("{}", value()),
    });

    view! { cx,
        <td class=class>{text}</td>
    }
}