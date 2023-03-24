#[cfg(feature = "IoPlayBackSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoPlayBackSharp")]
/// *This icon requires the feature* `IoPlayBackSharp` *to be enabled*.
#[component]
pub fn PlayBackSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polygon points="496 400 256 256 496 112 496 400" /><polygon points="256 400 16 256 256 112 256 400" /></svg>
   }
}