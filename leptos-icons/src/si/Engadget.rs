#[cfg(feature = "SiEngadget")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiEngadget")]
/// *This icon requires the feature* `SiEngadget` *to be enabled*.
#[component]
pub fn Engadget(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M0 20.067a3.9 3.9 0 0 0 4 3.866h16v-4H4v-4h15.733A4.231 4.231 0 0 0 24 12.067V4.333A4.483 4.483 0 0 0 19.733.067H4a4.346 4.346 0 0 0-4 4.266Zm20-8.134H4v-8h16Z" /></svg>
   }
}