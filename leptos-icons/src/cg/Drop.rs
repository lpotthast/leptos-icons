#[cfg(feature = "CgDrop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgDrop")]
/// *This icon requires the feature* `CgDrop` *to be enabled*.
#[component]
pub fn Drop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M6.34315 19.5208C3.21895 16.3966 3.21895 11.3312 6.34315 8.20705L12 2.5502L17.6569 8.20705C20.781 11.3312 20.781 16.3966 17.6569 19.5208C14.5327 22.645 9.46734 22.645 6.34315 19.5208Z" stroke="currentColor" stroke-width="2" /></svg>
   }
}