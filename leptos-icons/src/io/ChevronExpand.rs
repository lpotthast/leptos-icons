#[cfg(feature = "IoChevronExpand")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoChevronExpand")]
/// *This icon requires the feature* `IoChevronExpand` *to be enabled*.
#[component]
pub fn ChevronExpand(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M136 208L256 104L376 208" fill="none" stroke="currentColor" stroke-width="48" stroke-linecap="round" stroke-linejoin="round" /><path d="M136 304L256 408L376 304" fill="none" stroke="currentColor" stroke-width="48" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}