#[cfg(feature = "IoChevronCollapseOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoChevronCollapseOutline")]
/// *This icon requires the feature* `IoChevronCollapseOutline` *to be enabled*.
#[component]
pub fn ChevronCollapseOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M136 104L256 208L376 104" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" /><path d="M136 408L256 304L376 408" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" /></svg>
   }
}