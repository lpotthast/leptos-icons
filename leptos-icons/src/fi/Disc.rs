#[cfg(feature = "FiDisc")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiDisc")]
/// *This icon requires the feature* `FiDisc` *to be enabled*.
#[component]
pub fn Disc(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10" /><circle cx="12" cy="12" r="3" /></svg>
   }
}