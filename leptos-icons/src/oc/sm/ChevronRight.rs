#[cfg(feature = "OcSmChevronRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmChevronRight")]
/// *This icon requires the feature* `OcSmChevronRight` *to be enabled*.
#[component]
pub fn ChevronRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M6.22 3.22a.75.75 0 0 1 1.06 0l4.25 4.25a.75.75 0 0 1 0 1.06l-4.25 4.25a.751.751 0 0 1-1.042-.018.751.751 0 0 1-.018-1.042L9.94 8 6.22 4.28a.75.75 0 0 1 0-1.06Z" /></svg>
   }
}