#[cfg(feature = "OcLgCommit")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgCommit")]
/// *This icon requires the feature* `OcLgCommit` *to be enabled*.
#[component]
pub fn Commit(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M0 11.75A.75.75 0 0 1 .75 11h5a.75.75 0 0 1 0 1.5h-5a.75.75 0 0 1-.75-.75Zm17.5 0a.75.75 0 0 1 .75-.75h5a.75.75 0 0 1 0 1.5h-5a.75.75 0 0 1-.75-.75Z" /><path d="M12 17.75a6 6 0 1 1 0-12 6 6 0 0 1 0 12Zm0-1.5a4.5 4.5 0 1 0 0-9 4.5 4.5 0 0 0 0 9Z" /></svg>
   }
}