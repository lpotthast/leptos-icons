#[cfg(feature = "IoSwapVerticalOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoSwapVerticalOutline")]
/// *This icon requires the feature* `IoSwapVerticalOutline` *to be enabled*.
#[component]
pub fn SwapVerticalOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polyline points="464 208 352 96 240 208" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="352" y1="113.13" x2="352" y2="416" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><polyline points="48 304 160 416 272 304" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="160" y1="398" x2="160" y2="96" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}