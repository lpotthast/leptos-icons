#[cfg(feature = "IoArrowBackCircleOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoArrowBackCircleOutline")]
/// *This icon requires the feature* `IoArrowBackCircleOutline` *to be enabled*.
#[component]
pub fn ArrowBackCircleOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polyline points="249.38 336 170 256 249.38 176" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="181.03" y1="256" x2="342" y2="256" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M448,256c0-106-86-192-192-192S64,150,64,256s86,192,192,192S448,362,448,256Z" style="fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" /></svg>
   }
}