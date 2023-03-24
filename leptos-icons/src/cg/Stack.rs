#[cfg(feature = "CgStack")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgStack")]
/// *This icon requires the feature* `CgStack` *to be enabled*.
#[component]
pub fn Stack(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M20 4V16H22V2H8V4H20Z" fill="currentColor" /><path fill-rule="evenodd" clip-rule="evenodd" d="M2 8V22H16V8H2ZM14 10H4V20H14V10Z" fill="currentColor" /><path d="M17 7H5V5H19V19H17V7Z" fill="currentColor" /></svg>
   }
}