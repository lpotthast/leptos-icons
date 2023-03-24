#[cfg(feature = "OcLgMirror")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgMirror")]
/// *This icon requires the feature* `OcLgMirror` *to be enabled*.
#[component]
pub fn Mirror(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M21.553 6.064A.75.75 0 0 1 22 6.75v10.5a.75.75 0 0 1-1.256.554l-5.75-5.25a.748.748 0 0 1 0-1.108l5.75-5.25a.75.75 0 0 1 .809-.132ZM2.447 17.936A.75.75 0 0 1 2 17.25V6.75a.75.75 0 0 1 1.256-.554l5.75 5.25a.748.748 0 0 1 0 1.108l-5.75 5.25a.75.75 0 0 1-.809.132ZM7.387 12 3.5 8.45v7.1L7.388 12Zm9.226 0 3.887 3.55v-7.1L16.612 12ZM12 2.75a.75.75 0 0 1 .75.75v1a.75.75 0 0 1-1.5 0v-1a.75.75 0 0 1 .75-.75Zm0 4a.75.75 0 0 1 .75.75v1a.75.75 0 0 1-1.5 0v-1a.75.75 0 0 1 .75-.75Zm0 8a.75.75 0 0 1 .75.75v1a.75.75 0 0 1-1.5 0v-1a.75.75 0 0 1 .75-.75Zm0 4a.75.75 0 0 1 .75.75v1a.75.75 0 0 1-1.5 0v-1a.75.75 0 0 1 .75-.75Zm0-8a.75.75 0 0 1 .75.75v1a.75.75 0 0 1-1.5 0v-1a.75.75 0 0 1 .75-.75Z" /></svg>
   }
}