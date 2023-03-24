#[cfg(feature = "IoArrowBackOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoArrowBackOutline")]
/// *This icon requires the feature* `IoArrowBackOutline` *to be enabled*.
#[component]
pub fn ArrowBackOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polyline points="244 400 100 256 244 112" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:48px" /><line x1="120" y1="256" x2="412" y2="256" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:48px" /></svg>
   }
}