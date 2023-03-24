#[cfg(feature = "FiUnlock")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiUnlock")]
/// *This icon requires the feature* `FiUnlock` *to be enabled*.
#[component]
pub fn Unlock(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="11" width="18" height="11" rx="2" ry="2" /><path d="M7 11V7a5 5 0 0 1 9.9-1" /></svg>
   }
}