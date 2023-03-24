#[cfg(feature = "IoRefresh")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoRefresh")]
/// *This icon requires the feature* `IoRefresh` *to be enabled*.
#[component]
pub fn Refresh(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M320,146s24.36-12-64-12A160,160,0,1,0,416,294" style="fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px" /><polyline points="256 58 336 138 256 218" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}