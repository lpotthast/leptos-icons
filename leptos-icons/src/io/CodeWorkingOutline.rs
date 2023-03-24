#[cfg(feature = "IoCodeWorkingOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoCodeWorkingOutline")]
/// *This icon requires the feature* `IoCodeWorkingOutline` *to be enabled*.
#[component]
pub fn CodeWorkingOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><circle cx="256" cy="256" r="26" /><circle cx="346" cy="256" r="26" /><circle cx="166" cy="256" r="26" /><polyline points="160 368 32 256 160 144" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><polyline points="352 368 480 256 352 144" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}