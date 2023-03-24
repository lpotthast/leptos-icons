#[cfg(feature = "OcSmCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmCircle")]
/// *This icon requires the feature* `OcSmCircle` *to be enabled*.
#[component]
pub fn Circle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8Zm8-6.5a6.5 6.5 0 1 0 0 13 6.5 6.5 0 0 0 0-13Z" /></svg>
   }
}