#[cfg(feature = "OcSmCircleSlash")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmCircleSlash")]
/// *This icon requires the feature* `OcSmCircleSlash` *to be enabled*.
#[component]
pub fn CircleSlash(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M8 0a8 8 0 1 1 0 16A8 8 0 0 1 8 0ZM3.965 13.096a6.5 6.5 0 0 0 9.131-9.131ZM1.5 8a6.474 6.474 0 0 0 1.404 4.035l9.131-9.131A6.499 6.499 0 0 0 1.5 8Z" /></svg>
   }
}