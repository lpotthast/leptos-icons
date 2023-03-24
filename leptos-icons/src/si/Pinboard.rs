#[cfg(feature = "SiPinboard")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiPinboard")]
/// *This icon requires the feature* `SiPinboard` *to be enabled*.
#[component]
pub fn Pinboard(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M13.352 14.585l-4.509 4.614.72-4.062L3.428 7.57 0 7.753 7.58 0v2.953l7.214 6.646 4.513-1.105-4.689 4.982L24 24l-10.648-9.415z" /></svg>
   }
}