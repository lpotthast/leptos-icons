#[cfg(feature = "IoScanSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoScanSharp")]
/// *This icon requires the feature* `IoScanSharp` *to be enabled*.
#[component]
pub fn ScanSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M388,466H320V422h68a34,34,0,0,0,34-34V320h44v68A78.09,78.09,0,0,1,388,466Z" /><path d="M466,192H422V124a34,34,0,0,0-34-34H320V46h68a78.09,78.09,0,0,1,78,78Z" /><path d="M192,466H124a78.09,78.09,0,0,1-78-78V320H90v68a34,34,0,0,0,34,34h68Z" /><path d="M90,192H46V124a78.09,78.09,0,0,1,78-78h68V90H124a34,34,0,0,0-34,34Z" /></svg>
   }
}