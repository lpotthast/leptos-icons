#[cfg(feature = "OcSmRelFilePath")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmRelFilePath")]
/// *This icon requires the feature* `OcSmRelFilePath` *to be enabled*.
#[component]
pub fn RelFilePath(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M13.94 3.045a.75.75 0 0 0-1.38-.59l-4.5 10.5a.75.75 0 1 0 1.38.59l4.5-10.5ZM5 11.5a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0Z" /></svg>
   }
}