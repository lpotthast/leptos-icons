#[cfg(feature = "IoExitSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoExitSharp")]
/// *This icon requires the feature* `IoExitSharp` *to be enabled*.
#[component]
pub fn ExitSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M335.69,272h-161V240h161V92a12,12,0,0,0-12-12h-280a12,12,0,0,0-12,12V420a12,12,0,0,0,12,12h280a12,12,0,0,0,12-12Z" /><polygon points="419.06 272 355.06 336 377.69 358.63 480.31 256 377.69 153.37 355.06 176 419.06 240 335.69 240 335.69 272 419.06 272" /></svg>
   }
}