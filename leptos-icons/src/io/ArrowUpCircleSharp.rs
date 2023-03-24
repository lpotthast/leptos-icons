#[cfg(feature = "IoArrowUpCircleSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoArrowUpCircleSharp")]
/// *This icon requires the feature* `IoArrowUpCircleSharp` *to be enabled*.
#[component]
pub fn ArrowUpCircleSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M256,48C141.13,48,48,141.13,48,256s93.13,208,208,208,208-93.13,208-208S370.87,48,256,48Zm80.09,224L272,208.42V358H240V208.42L175.91,272,153.37,249.3,256,147.46,358.63,249.3Z" /></svg>
   }
}