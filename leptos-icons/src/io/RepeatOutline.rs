#[cfg(feature = "IoRepeatOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoRepeatOutline")]
/// *This icon requires the feature* `IoRepeatOutline` *to be enabled*.
#[component]
pub fn RepeatOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polyline points="320 120 368 168 320 216" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M352,168H144a80.24,80.24,0,0,0-80,80v16" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><polyline points="192 392 144 344 192 296" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M160,344H368a80.24,80.24,0,0,0,80-80V248" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}