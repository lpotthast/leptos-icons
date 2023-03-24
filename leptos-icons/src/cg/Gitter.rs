#[cfg(feature = "CgGitter")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgGitter")]
/// *This icon requires the feature* `CgGitter` *to be enabled*.
#[component]
pub fn Gitter(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M5 1.5H7V14.5H5V1.5Z" fill="currentColor" /><path d="M9 4.5H11V22.5H9V4.5Z" fill="currentColor" /><path d="M15 4.5H13V22.5H15V4.5Z" fill="currentColor" /><path d="M17 4.5H19V14.5H17V4.5Z" fill="currentColor" /></svg>
   }
}