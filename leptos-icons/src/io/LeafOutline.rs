#[cfg(feature = "IoLeafOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoLeafOutline")]
/// *This icon requires the feature* `IoLeafOutline` *to be enabled*.
#[component]
pub fn LeafOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M321.89,171.42C233,114,141,155.22,56,65.22c-19.8-21-8.3,235.5,98.1,332.7C231.89,468.92,352,461,392.5,392S410.78,228.83,321.89,171.42Z" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M173,253c86,81,175,129,292,147" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}