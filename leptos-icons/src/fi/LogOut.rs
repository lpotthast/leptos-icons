#[cfg(feature = "FiLogOut")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiLogOut")]
/// *This icon requires the feature* `FiLogOut` *to be enabled*.
#[component]
pub fn LogOut(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" /><polyline points="16 17 21 12 16 7" /><line x1="21" y1="12" x2="9" y2="12" /></svg>
   }
}