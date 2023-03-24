#[cfg(feature = "SiClickhouse")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiClickhouse")]
/// *This icon requires the feature* `SiClickhouse` *to be enabled*.
#[component]
pub fn Clickhouse(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M21.333 10H24v4h-2.667ZM16 1.335h2.667v21.33H16Zm-5.333 0h2.666v21.33h-2.666ZM0 22.665V1.335h2.667v21.33zm5.333-21.33H8v21.33H5.333Z" /></svg>
   }
}