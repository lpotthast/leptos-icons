#[cfg(feature = "OcLgSquare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgSquare")]
/// *This icon requires the feature* `OcLgSquare` *to be enabled*.
#[component]
pub fn Square(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M6 7.75C6 6.784 6.784 6 7.75 6h8.5c.966 0 1.75.784 1.75 1.75v8.5A1.75 1.75 0 0 1 16.25 18h-8.5A1.75 1.75 0 0 1 6 16.25Zm1.75-.25a.25.25 0 0 0-.25.25v8.5c0 .138.112.25.25.25h8.5a.25.25 0 0 0 .25-.25v-8.5a.25.25 0 0 0-.25-.25Z" /></svg>
   }
}