#[cfg(feature = "OcLgPersonAdd")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgPersonAdd")]
/// *This icon requires the feature* `OcLgPersonAdd` *to be enabled*.
#[component]
pub fn PersonAdd(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M4 9.5a5 5 0 1 1 7.916 4.062 7.973 7.973 0 0 1 5.018 7.166.75.75 0 1 1-1.499.044 6.469 6.469 0 0 0-12.932 0 .75.75 0 0 1-1.499-.044 7.972 7.972 0 0 1 5.059-7.181A4.994 4.994 0 0 1 4 9.5ZM9 6a3.5 3.5 0 1 0 0 7 3.5 3.5 0 0 0 0-7Zm10.25-5a.75.75 0 0 1 .75.75V4h2.25a.75.75 0 0 1 0 1.5H20v2.25a.75.75 0 0 1-1.5 0V5.5h-2.25a.75.75 0 0 1 0-1.5h2.25V1.75a.75.75 0 0 1 .75-.75Z" /></svg>
   }
}