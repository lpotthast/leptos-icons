#[cfg(feature = "IoChevronUpCircleSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoChevronUpCircleSharp")]
/// *This icon requires the feature* `IoChevronUpCircleSharp` *to be enabled*.
#[component]
pub fn ChevronUpCircleSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M256,48C141.13,48,48,141.13,48,256s93.13,208,208,208,208-93.13,208-208S370.87,48,256,48Zm96,270.63-96-96-96,96L137.37,296,256,177.37,374.63,296Z" /></svg>
   }
}