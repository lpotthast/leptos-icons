#[cfg(feature = "IoChevronBackCircleOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoChevronBackCircleOutline")]
/// *This icon requires the feature* `IoChevronBackCircleOutline` *to be enabled*.
#[component]
pub fn ChevronBackCircleOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M256,64C150,64,64,150,64,256s86,192,192,192,192-86,192-192S362,64,256,64Z" style="fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" /><polyline points="296 352 200 256 296 160" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}