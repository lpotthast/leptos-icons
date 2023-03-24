#[cfg(feature = "IoTvOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoTvOutline")]
/// *This icon requires the feature* `IoTvOutline` *to be enabled*.
#[component]
pub fn TvOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect x="32" y="96" width="448" height="272" rx="32.14" ry="32.14" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /><line x1="128" y1="416" x2="384" y2="416" style="stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px" /></svg>
   }
}