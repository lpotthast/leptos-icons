#[cfg(feature = "IoFolderSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoFolderSharp")]
/// *This icon requires the feature* `IoFolderSharp` *to be enabled*.
#[component]
pub fn FolderSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M16,420a28,28,0,0,0,28,28H468a28,28,0,0,0,28-28V208H16Z" /><path d="M496,124a28,28,0,0,0-28-28H212.84l-48-32H44A28,28,0,0,0,16,92v84H496Z" /></svg>
   }
}