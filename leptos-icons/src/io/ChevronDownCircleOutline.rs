#[cfg(feature = "IoChevronDownCircleOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoChevronDownCircleOutline")]
/// *This icon requires the feature* `IoChevronDownCircleOutline` *to be enabled*.
#[component]
pub fn ChevronDownCircleOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M256,64C150,64,64,150,64,256s86,192,192,192,192-86,192-192S362,64,256,64Z" style="fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" /><polyline points="352 216 256 312 160 216" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}