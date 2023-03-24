#[cfg(feature = "IoChevronForwardCircleOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoChevronForwardCircleOutline")]
/// *This icon requires the feature* `IoChevronForwardCircleOutline` *to be enabled*.
#[component]
pub fn ChevronForwardCircleOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M64,256c0,106,86,192,192,192s192-86,192-192S362,64,256,64,64,150,64,256Z" style="fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" /><polyline points="216 352 312 256 216 160" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}