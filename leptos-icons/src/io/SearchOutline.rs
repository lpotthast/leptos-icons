#[cfg(feature = "IoSearchOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoSearchOutline")]
/// *This icon requires the feature* `IoSearchOutline` *to be enabled*.
#[component]
pub fn SearchOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M221.09,64A157.09,157.09,0,1,0,378.18,221.09,157.1,157.1,0,0,0,221.09,64Z" style="fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" /><line x1="338.29" y1="338.29" x2="448" y2="448" style="fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px" /></svg>
   }
}