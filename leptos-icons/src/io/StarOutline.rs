#[cfg(feature = "IoStarOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoStarOutline")]
/// *This icon requires the feature* `IoStarOutline` *to be enabled*.
#[component]
pub fn StarOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M480,208H308L256,48,204,208H32l140,96L118,464,256,364,394,464,340,304Z" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}