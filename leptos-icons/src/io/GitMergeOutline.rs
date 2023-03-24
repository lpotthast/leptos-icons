#[cfg(feature = "IoGitMergeOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoGitMergeOutline")]
/// *This icon requires the feature* `IoGitMergeOutline` *to be enabled*.
#[component]
pub fn GitMergeOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><circle cx="129" cy="96" r="48" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><circle cx="129" cy="416" r="48" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="129" y1="144" x2="129" y2="368" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><circle cx="385" cy="288" r="48" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M129,144c0,96,112,144,208,144" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}