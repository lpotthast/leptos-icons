#[cfg(feature = "IoBasketballOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoBasketballOutline")]
/// *This icon requires the feature* `IoBasketballOutline` *to be enabled*.
#[component]
pub fn BasketballOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><circle cx="256" cy="256" r="192" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M432.94,255.05A192,192,0,0,1,256.63,74.35" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M255,433.61A192,192,0,0,0,74.29,256.69" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="120.24" y1="120.24" x2="391.76" y2="391.76" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="120.24" y1="391.76" x2="391.76" y2="120.24" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}