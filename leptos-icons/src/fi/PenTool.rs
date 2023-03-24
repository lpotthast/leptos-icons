#[cfg(feature = "FiPenTool")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiPenTool")]
/// *This icon requires the feature* `FiPenTool` *to be enabled*.
#[component]
pub fn PenTool(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 19l7-7 3 3-7 7-3-3z" /><path d="M18 13l-1.5-7.5L2 2l3.5 14.5L13 18l5-5z" /><path d="M2 2l7.586 7.586" /><circle cx="11" cy="11" r="2" /></svg>
   }
}