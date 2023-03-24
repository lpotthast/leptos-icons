#[cfg(feature = "IoPlayForwardSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoPlayForwardSharp")]
/// *This icon requires the feature* `IoPlayForwardSharp` *to be enabled*.
#[component]
pub fn PlayForwardSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polygon points="16 400 256 256 16 112 16 400" /><polygon points="256 400 496 256 256 112 256 400" /></svg>
   }
}