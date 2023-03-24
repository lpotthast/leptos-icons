#[cfg(feature = "IoArrowRedoOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoArrowRedoOutline")]
/// *This icon requires the feature* `IoArrowRedoOutline` *to be enabled*.
#[component]
pub fn ArrowRedoOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M448,256,272,88v96C103.57,184,64,304.77,64,424c48.61-62.24,91.6-96,208-96v96Z" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}