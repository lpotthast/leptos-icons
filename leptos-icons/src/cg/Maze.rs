#[cfg(feature = "CgMaze")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgMaze")]
/// *This icon requires the feature* `CgMaze` *to be enabled*.
#[component]
pub fn Maze(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M11.3709 9.59273L8.77816 7L1 14.7782L3.59272 17.3709L11.3709 9.59273Z" fill="currentColor" /><path d="M15.2218 7L23 14.7782L20.424 17.3542L15.2218 12.152L10.0197 17.3542L7.44367 14.7782L15.2218 7Z" fill="currentColor" /></svg>
   }
}