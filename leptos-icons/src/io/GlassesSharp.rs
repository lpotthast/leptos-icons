#[cfg(feature = "IoGlassesSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoGlassesSharp")]
/// *This icon requires the feature* `IoGlassesSharp` *to be enabled*.
#[component]
pub fn GlassesSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M496,176H16v64H37.24L49.68,352H221.55L240,241.32V240a16,16,0,0,1,32,0v1.32L290.45,352H462.32l12.44-112H496Z" /></svg>
   }
}