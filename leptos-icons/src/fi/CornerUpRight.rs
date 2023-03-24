#[cfg(feature = "FiCornerUpRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiCornerUpRight")]
/// *This icon requires the feature* `FiCornerUpRight` *to be enabled*.
#[component]
pub fn CornerUpRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="15 14 20 9 15 4" /><path d="M4 20v-7a4 4 0 0 1 4-4h12" /></svg>
   }
}