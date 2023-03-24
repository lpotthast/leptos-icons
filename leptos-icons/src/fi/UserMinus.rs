#[cfg(feature = "FiUserMinus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiUserMinus")]
/// *This icon requires the feature* `FiUserMinus` *to be enabled*.
#[component]
pub fn UserMinus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" /><circle cx="8.5" cy="7" r="4" /><line x1="23" y1="11" x2="17" y2="11" /></svg>
   }
}