#[cfg(feature = "IoPinSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoPinSharp")]
/// *This icon requires the feature* `IoPinSharp` *to be enabled*.
#[component]
pub fn PinSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M339,99a83,83,0,1,0-102,80.8V464l19,32,19-32V179.8A83.28,83.28,0,0,0,339,99Zm-59-6a21,21,0,1,1,21-21A21,21,0,0,1,280,93Z" /></svg>
   }
}