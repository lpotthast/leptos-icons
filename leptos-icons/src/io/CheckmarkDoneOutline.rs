#[cfg(feature = "IoCheckmarkDoneOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoCheckmarkDoneOutline")]
/// *This icon requires the feature* `IoCheckmarkDoneOutline` *to be enabled*.
#[component]
pub fn CheckmarkDoneOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polyline points="464 128 240 384 144 288" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="144" y1="384" x2="48" y2="288" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="368" y1="128" x2="232" y2="284" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}