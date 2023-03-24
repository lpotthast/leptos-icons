#[cfg(feature = "IoReaderSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoReaderSharp")]
/// *This icon requires the feature* `IoReaderSharp` *to be enabled*.
#[component]
pub fn ReaderSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M80,44V468a12,12,0,0,0,12,12H420a12,12,0,0,0,12-12V44a12,12,0,0,0-12-12H92A12,12,0,0,0,80,44ZM272,304H160V272H272Zm80-80H160V192H352Zm0-80H160V112H352Z" /></svg>
   }
}