#[cfg(feature = "FiXSquare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiXSquare")]
/// *This icon requires the feature* `FiXSquare` *to be enabled*.
#[component]
pub fn XSquare(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="2" ry="2" /><line x1="9" y1="9" x2="15" y2="15" /><line x1="15" y1="9" x2="9" y2="15" /></svg>
   }
}