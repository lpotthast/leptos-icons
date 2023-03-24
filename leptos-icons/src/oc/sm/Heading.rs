#[cfg(feature = "OcSmHeading")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmHeading")]
/// *This icon requires the feature* `OcSmHeading` *to be enabled*.
#[component]
pub fn Heading(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M3.75 2a.75.75 0 0 1 .75.75V7h7V2.75a.75.75 0 0 1 1.5 0v10.5a.75.75 0 0 1-1.5 0V8.5h-7v4.75a.75.75 0 0 1-1.5 0V2.75A.75.75 0 0 1 3.75 2Z" /></svg>
   }
}