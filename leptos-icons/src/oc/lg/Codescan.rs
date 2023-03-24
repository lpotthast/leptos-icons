#[cfg(feature = "OcLgCodescan")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgCodescan")]
/// *This icon requires the feature* `OcLgCodescan` *to be enabled*.
#[component]
pub fn Codescan(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M11.97 6.97a.75.75 0 0 0 0 1.06l2.47 2.47-2.47 2.47a.75.75 0 1 0 1.06 1.06l3-3a.75.75 0 0 0 0-1.06l-3-3a.75.75 0 0 0-1.06 0ZM9.03 8.03a.75.75 0 0 0-1.06-1.06l-3 3a.75.75 0 0 0 0 1.06l3 3a.75.75 0 0 0 1.06-1.06L6.56 10.5l2.47-2.47Z" /><path d="M10.5 0C16.299 0 21 4.701 21 10.5a10.457 10.457 0 0 1-2.564 6.875l4.344 4.345a.749.749 0 0 1-.326 1.275.749.749 0 0 1-.734-.215l-4.345-4.344A10.459 10.459 0 0 1 10.5 21C4.701 21 0 16.299 0 10.5S4.701 0 10.5 0Zm-9 10.5a9 9 0 0 0 9 9 9 9 0 0 0 9-9 9 9 0 0 0-9-9 9 9 0 0 0-9 9Z" /></svg>
   }
}