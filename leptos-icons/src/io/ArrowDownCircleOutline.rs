#[cfg(feature = "IoArrowDownCircleOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoArrowDownCircleOutline")]
/// *This icon requires the feature* `IoArrowDownCircleOutline` *to be enabled*.
#[component]
pub fn ArrowDownCircleOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polyline points="176 262.62 256 342 336 262.62" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="256" y1="330.97" x2="256" y2="170" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M256,64C150,64,64,150,64,256s86,192,192,192,192-86,192-192S362,64,256,64Z" style="fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" /></svg>
   }
}