#[cfg(feature = "IoVolumeOffSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoVolumeOffSharp")]
/// *This icon requires the feature* `IoVolumeOffSharp` *to be enabled*.
#[component]
pub fn VolumeOffSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polygon points="237.65 176.1 144 176.1 144 335.9 237.65 335.9 368 440 368 72 237.65 176.1" /></svg>
   }
}