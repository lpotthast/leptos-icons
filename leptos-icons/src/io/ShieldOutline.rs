#[cfg(feature = "IoShieldOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoShieldOutline")]
/// *This icon requires the feature* `IoShieldOutline` *to be enabled*.
#[component]
pub fn ShieldOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M463.1,112.37C373.68,96.33,336.71,84.45,256,48,175.29,84.45,138.32,96.33,48.9,112.37,32.7,369.13,240.58,457.79,256,464,271.42,457.79,479.3,369.13,463.1,112.37Z" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}