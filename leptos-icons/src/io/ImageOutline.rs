#[cfg(feature = "IoImageOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoImageOutline")]
/// *This icon requires the feature* `IoImageOutline` *to be enabled*.
#[component]
pub fn ImageOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect x="48" y="80" width="416" height="352" rx="48" ry="48" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /><circle cx="336" cy="176" r="32" style="fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" /><path d="M304,335.79,213.34,245.3A32,32,0,0,0,169.47,244L48,352" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M224,432,347.34,308.66a32,32,0,0,1,43.11-2L464,368" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}