#[cfg(feature = "IoFilter")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoFilter")]
/// *This icon requires the feature* `IoFilter` *to be enabled*.
#[component]
pub fn Filter(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M472,168H40a24,24,0,0,1,0-48H472a24,24,0,0,1,0,48Z" /><path d="M392,280H120a24,24,0,0,1,0-48H392a24,24,0,0,1,0,48Z" /><path d="M296,392H216a24,24,0,0,1,0-48h80a24,24,0,0,1,0,48Z" /></svg>
   }
}