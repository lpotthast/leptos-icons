#[cfg(feature = "IoQrCodeSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoQrCodeSharp")]
/// *This icon requires the feature* `IoQrCodeSharp` *to be enabled*.
#[component]
pub fn QrCodeSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect x="336" y="336" width="80" height="80" /><rect x="272" y="272" width="64" height="64" /><rect x="416" y="416" width="64" height="64" /><rect x="432" y="272" width="48" height="48" /><rect x="272" y="432" width="48" height="48" /><rect x="336" y="96" width="80" height="80" /><path d="M480,240H272V32H480ZM316,196H436V76H316Z" /><rect x="96" y="96" width="80" height="80" /><path d="M240,240H32V32H240ZM76,196H196V76H76Z" /><rect x="96" y="336" width="80" height="80" /><path d="M240,480H32V272H240ZM76,436H196V316H76Z" /></svg>
   }
}