#[cfg(feature = "IoChatboxSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoChatboxSharp")]
/// *This icon requires the feature* `IoChatboxSharp` *to be enabled*.
#[component]
pub fn ChatboxSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M128,464V384H56a24,24,0,0,1-24-24V72A24,24,0,0,1,56,48H456a24,24,0,0,1,24,24V360a24,24,0,0,1-24,24H245.74ZM456,80h0Z" /></svg>
   }
}