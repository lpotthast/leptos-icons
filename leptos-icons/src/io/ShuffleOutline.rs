#[cfg(feature = "IoShuffleOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoShuffleOutline")]
/// *This icon requires the feature* `IoShuffleOutline` *to be enabled*.
#[component]
pub fn ShuffleOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polyline points="400 304 448 352 400 400" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><polyline points="400 112 448 160 400 208" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M64,352h85.19a80,80,0,0,0,66.56-35.62L256,256" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M64,160h85.19a80,80,0,0,1,66.56,35.62l80.5,120.76A80,80,0,0,0,362.81,352H416" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M416,160H362.81a80,80,0,0,0-66.56,35.62L288,208" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}