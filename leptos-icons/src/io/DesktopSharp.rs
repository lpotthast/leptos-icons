#[cfg(feature = "IoDesktopSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoDesktopSharp")]
/// *This icon requires the feature* `IoDesktopSharp` *to be enabled*.
#[component]
pub fn DesktopSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M480,48H32A16,16,0,0,0,16,64V384a16,16,0,0,0,16,16H200v32H128v32H384V432H312V400H480a16,16,0,0,0,16-16V64A16,16,0,0,0,480,48ZM460,84V300H52V84ZM240.13,354.08a16,16,0,1,1,13.79,13.79A16,16,0,0,1,240.13,354.08Z" /></svg>
   }
}