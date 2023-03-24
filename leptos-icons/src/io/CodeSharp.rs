#[cfg(feature = "IoCodeSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoCodeSharp")]
/// *This icon requires the feature* `IoCodeSharp` *to be enabled*.
#[component]
pub fn CodeSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polygon points="161.98 397.63 0 256 161.98 114.37 189.63 145.98 64 256 189.63 366.02 161.98 397.63" /><polygon points="350.02 397.63 322.37 366.02 448 256 322.37 145.98 350.02 114.37 512 256 350.02 397.63" /></svg>
   }
}