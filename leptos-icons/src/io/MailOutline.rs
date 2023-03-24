#[cfg(feature = "IoMailOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoMailOutline")]
/// *This icon requires the feature* `IoMailOutline` *to be enabled*.
#[component]
pub fn MailOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect x="48" y="96" width="416" height="320" rx="40" ry="40" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><polyline points="112 160 256 272 400 160" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}