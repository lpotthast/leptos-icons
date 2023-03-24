#[cfg(feature = "IoPodium")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoPodium")]
/// *This icon requires the feature* `IoPodium` *to be enabled*.
#[component]
pub fn Podium(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M320,32H192a32,32,0,0,0-32,32V476a4,4,0,0,0,4,4H348a4,4,0,0,0,4-4V64A32,32,0,0,0,320,32Z" /><path d="M464,192H392a8,8,0,0,0-8,8V472a8,8,0,0,0,8,8h80a24,24,0,0,0,24-24V224A32,32,0,0,0,464,192Z" /><path d="M48,128a32,32,0,0,0-32,32V456a24,24,0,0,0,24,24h80a8,8,0,0,0,8-8V136a8,8,0,0,0-8-8Z" /></svg>
   }
}