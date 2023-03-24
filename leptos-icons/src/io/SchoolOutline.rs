#[cfg(feature = "IoSchoolOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoSchoolOutline")]
/// *This icon requires the feature* `IoSchoolOutline` *to be enabled*.
#[component]
pub fn SchoolOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polygon points="32 192 256 64 480 192 256 320 32 192" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><polyline points="112 240 112 368 256 448 400 368 400 240" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="480" y1="368" x2="480" y2="192" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="256" y1="320" x2="256" y2="448" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}