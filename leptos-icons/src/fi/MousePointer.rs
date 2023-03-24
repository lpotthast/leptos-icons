#[cfg(feature = "FiMousePointer")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiMousePointer")]
/// *This icon requires the feature* `FiMousePointer` *to be enabled*.
#[component]
pub fn MousePointer(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 3l7.07 16.97 2.51-7.39 7.39-2.51L3 3z" /><path d="M13 13l6 6" /></svg>
   }
}