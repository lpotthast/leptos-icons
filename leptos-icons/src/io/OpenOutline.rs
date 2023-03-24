#[cfg(feature = "IoOpenOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoOpenOutline")]
/// *This icon requires the feature* `IoOpenOutline` *to be enabled*.
#[component]
pub fn OpenOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M384,224V408a40,40,0,0,1-40,40H104a40,40,0,0,1-40-40V168a40,40,0,0,1,40-40H271.48" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><polyline points="336 64 448 64 448 176" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="224" y1="288" x2="440" y2="72" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}