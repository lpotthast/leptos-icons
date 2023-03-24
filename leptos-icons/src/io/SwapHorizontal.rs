#[cfg(feature = "IoSwapHorizontal")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoSwapHorizontal")]
/// *This icon requires the feature* `IoSwapHorizontal` *to be enabled*.
#[component]
pub fn SwapHorizontal(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polyline points="304 48 416 160 304 272" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="398.87" y1="160" x2="96" y2="160" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><polyline points="208 464 96 352 208 240" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="114" y1="352" x2="416" y2="352" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}