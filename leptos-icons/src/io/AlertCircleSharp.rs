#[cfg(feature = "IoAlertCircleSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoAlertCircleSharp")]
/// *This icon requires the feature* `IoAlertCircleSharp` *to be enabled*.
#[component]
pub fn AlertCircleSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polygon points="240 304 272 304 278 144 234 144 240 304" style="fill:none" /><path d="M256,48C141.31,48,48,141.31,48,256s93.31,208,208,208,208-93.31,208-208S370.69,48,256,48Zm20,319.91H236v-40h40ZM272,304H240l-6-160h44Z" /></svg>
   }
}