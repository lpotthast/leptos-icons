#[cfg(feature = "IoLockClosedSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoLockClosedSharp")]
/// *This icon requires the feature* `IoLockClosedSharp` *to be enabled*.
#[component]
pub fn LockClosedSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M420,192H352V112a96,96,0,1,0-192,0v80H92a12,12,0,0,0-12,12V484a12,12,0,0,0,12,12H420a12,12,0,0,0,12-12V204A12,12,0,0,0,420,192Zm-106,0H198V111.25a58,58,0,1,1,116,0Z" /></svg>
   }
}