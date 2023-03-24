#[cfg(feature = "IoCheckmarkCircleSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoCheckmarkCircleSharp")]
/// *This icon requires the feature* `IoCheckmarkCircleSharp` *to be enabled*.
#[component]
pub fn CheckmarkCircleSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M256,48C141.31,48,48,141.31,48,256s93.31,208,208,208,208-93.31,208-208S370.69,48,256,48ZM218,360.38,137.4,270.81l23.79-21.41,56,62.22L350,153.46,374.54,174Z" /></svg>
   }
}