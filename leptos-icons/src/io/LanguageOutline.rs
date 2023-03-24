#[cfg(feature = "IoLanguageOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoLanguageOutline")]
/// *This icon requires the feature* `IoLanguageOutline` *to be enabled*.
#[component]
pub fn LanguageOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><line x1="48" y1="112" x2="336" y2="112" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="192" y1="64" x2="192" y2="112" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><polyline points="272 448 368 224 464 448" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="301.5" y1="384" x2="434.5" y2="384" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M281.3,112S257,206,199,277,80,384,80,384" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M256,336s-35-27-72-75-56-85-56-85" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}