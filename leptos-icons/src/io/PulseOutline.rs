#[cfg(feature = "IoPulseOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoPulseOutline")]
/// *This icon requires the feature* `IoPulseOutline` *to be enabled*.
#[component]
pub fn PulseOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polyline points="48 320 112 320 176 64 240 448 304 224 336 320 400 320" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><circle cx="432" cy="320" r="32" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}