#[cfg(feature = "IoArrowDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoArrowDown")]
/// *This icon requires the feature* `IoArrowDown` *to be enabled*.
#[component]
pub fn ArrowDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polyline points="112 268 256 412 400 268" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:48px" /><line x1="256" y1="392" x2="256" y2="100" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:48px" /></svg>
   }
}