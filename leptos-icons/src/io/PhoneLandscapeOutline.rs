#[cfg(feature = "IoPhoneLandscapeOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoPhoneLandscapeOutline")]
/// *This icon requires the feature* `IoPhoneLandscapeOutline` *to be enabled*.
#[component]
pub fn PhoneLandscapeOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect x="128" y="16" width="256" height="480" rx="48" ry="48" transform="translate(0 512) rotate(-90)" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M16,336V312a8,8,0,0,1,8-8h0a16,16,0,0,0,16-16V224a16,16,0,0,0-16-16h0a8,8,0,0,1-8-8V176" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}