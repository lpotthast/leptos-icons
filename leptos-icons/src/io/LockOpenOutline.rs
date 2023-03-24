#[cfg(feature = "IoLockOpenOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoLockOpenOutline")]
/// *This icon requires the feature* `IoLockOpenOutline` *to be enabled*.
#[component]
pub fn LockOpenOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M336,112a80,80,0,0,0-160,0v96" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><rect x="96" y="208" width="320" height="272" rx="48" ry="48" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}