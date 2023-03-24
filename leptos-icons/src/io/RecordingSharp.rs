#[cfg(feature = "IoRecordingSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoRecordingSharp")]
/// *This icon requires the feature* `IoRecordingSharp` *to be enabled*.
#[component]
pub fn RecordingSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M384,138a117.93,117.93,0,0,0-91.84,192H219.84A118,118,0,1,0,128,374H384a118,118,0,0,0,0-236ZM54,256a74,74,0,1,1,74,74A74.09,74.09,0,0,1,54,256Zm330,74a74,74,0,1,1,74-74A74.09,74.09,0,0,1,384,330Z" /></svg>
   }
}