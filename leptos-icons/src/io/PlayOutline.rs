#[cfg(feature = "IoPlayOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoPlayOutline")]
/// *This icon requires the feature* `IoPlayOutline` *to be enabled*.
#[component]
pub fn PlayOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M112,111V401c0,17.44,17,28.52,31,20.16l247.9-148.37c12.12-7.25,12.12-26.33,0-33.58L143,90.84C129,82.48,112,93.56,112,111Z" style="fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" /></svg>
   }
}