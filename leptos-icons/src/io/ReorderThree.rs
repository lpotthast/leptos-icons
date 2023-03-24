#[cfg(feature = "IoReorderThree")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoReorderThree")]
/// *This icon requires the feature* `IoReorderThree` *to be enabled*.
#[component]
pub fn ReorderThree(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><line x1="102" y1="256" x2="410" y2="256" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:44px" /><line x1="102" y1="176" x2="410" y2="176" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:44px" /><line x1="102" y1="336" x2="410" y2="336" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:44px" /></svg>
   }
}