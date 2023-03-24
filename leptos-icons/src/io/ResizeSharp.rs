#[cfg(feature = "IoResizeSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoResizeSharp")]
/// *This icon requires the feature* `IoResizeSharp` *to be enabled*.
#[component]
pub fn ResizeSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polyline points="304 96 416 96 416 208" style="fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px" /><line x1="405.77" y1="106.2" x2="111.98" y2="400.02" style="fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px" /><polyline points="208 416 96 416 96 304" style="fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px" /></svg>
   }
}