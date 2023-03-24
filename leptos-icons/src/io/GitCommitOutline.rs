#[cfg(feature = "IoGitCommitOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoGitCommitOutline")]
/// *This icon requires the feature* `IoGitCommitOutline` *to be enabled*.
#[component]
pub fn GitCommitOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><circle cx="256" cy="256" r="96" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="160" y1="256" x2="48" y2="256" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="464" y1="256" x2="352" y2="256" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}