#[cfg(feature = "IoClipboardSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoClipboardSharp")]
/// *This icon requires the feature* `IoClipboardSharp` *to be enabled*.
#[component]
pub fn ClipboardSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M420,48H352V28a12,12,0,0,0-12-12H172a12,12,0,0,0-12,12V48H92A12,12,0,0,0,80,60V484a12,12,0,0,0,12,12H420a12,12,0,0,0,12-12V60A12,12,0,0,0,420,48Zm-84.13,64H176.13V80H335.87Z" /></svg>
   }
}