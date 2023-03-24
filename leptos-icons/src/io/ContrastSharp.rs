#[cfg(feature = "IoContrastSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoContrastSharp")]
/// *This icon requires the feature* `IoContrastSharp` *to be enabled*.
#[component]
pub fn ContrastSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M256,32C132.29,32,32,132.29,32,256S132.29,480,256,480,480,379.71,480,256,379.71,32,256,32ZM128.72,383.28A180,180,0,0,1,256,76V436A178.82,178.82,0,0,1,128.72,383.28Z" /></svg>
   }
}