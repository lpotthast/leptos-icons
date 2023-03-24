#[cfg(feature = "FiFile")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiFile")]
/// *This icon requires the feature* `FiFile` *to be enabled*.
#[component]
pub fn File(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z" /><polyline points="13 2 13 9 20 9" /></svg>
   }
}