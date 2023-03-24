#[cfg(feature = "IoBarChartSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoBarChartSharp")]
/// *This icon requires the feature* `IoBarChartSharp` *to be enabled*.
#[component]
pub fn BarChartSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polygon points="496 496 16 496 16 16 48 16 48 464 496 464 496 496" /><path d="M192,432H80V208H192Z" /><path d="M336,432H224V160H336Z" /><path d="M479.64,432h-112V96h112Z" /></svg>
   }
}