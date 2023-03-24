#[cfg(feature = "IoFileTraySharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoFileTraySharp")]
/// *This icon requires the feature* `IoFileTraySharp` *to be enabled*.
#[component]
pub fn FileTraySharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M448,64H64L32,256V448H480V256ZM436,256H320a64,64,0,0,1-128,0H76L98,106H414Z" /></svg>
   }
}