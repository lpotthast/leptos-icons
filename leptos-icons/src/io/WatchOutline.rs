#[cfg(feature = "IoWatchOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoWatchOutline")]
/// *This icon requires the feature* `IoWatchOutline` *to be enabled*.
#[component]
pub fn WatchOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect x="112" y="112" width="288" height="288" rx="64" ry="64" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /><path d="M176,112V40a8,8,0,0,1,8-8H328a8,8,0,0,1,8,8v72" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /><path d="M336,400v72a8,8,0,0,1-8,8H184a8,8,0,0,1-8-8V400" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}