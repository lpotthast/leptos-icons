#[cfg(feature = "IoAlbumsOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoAlbumsOutline")]
/// *This icon requires the feature* `IoAlbumsOutline` *to be enabled*.
#[component]
pub fn AlbumsOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect x="64" y="176" width="384" height="256" rx="28.87" ry="28.87" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /><line x1="144" y1="80" x2="368" y2="80" style="stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px" /><line x1="112" y1="128" x2="400" y2="128" style="stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px" /></svg>
   }
}