#[cfg(feature = "SiUndertale")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiUndertale")]
/// *This icon requires the feature* `SiUndertale` *to be enabled*.
#[component]
pub fn Undertale(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M3 0v1.5H1.5V3H0v12h3v3h3v3h3v3h6v-3h3v-3h3v-3h3V3h-1.5V1.5H21V0h-3v1.5h-3V3h-1.5v3h-3V3H9V1.5H6V0z" /></svg>
   }
}