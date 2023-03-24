#[cfg(feature = "FiCornerLeftDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiCornerLeftDown")]
/// *This icon requires the feature* `FiCornerLeftDown` *to be enabled*.
#[component]
pub fn CornerLeftDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="14 15 9 20 4 15" /><path d="M20 4h-7a4 4 0 0 0-4 4v12" /></svg>
   }
}