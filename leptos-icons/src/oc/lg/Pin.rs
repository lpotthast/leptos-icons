#[cfg(feature = "OcLgPin")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgPin")]
/// *This icon requires the feature* `OcLgPin` *to be enabled*.
#[component]
pub fn Pin(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M7.886 1.553a1.75 1.75 0 0 1 2.869.604l.633 1.629a5.667 5.667 0 0 0 3.725 3.395l3.959 1.131a1.749 1.749 0 0 1 .757 2.92L16.06 15l5.594 5.595a.749.749 0 0 1-.326 1.275.749.749 0 0 1-.734-.215L15 16.061l-3.768 3.768a1.749 1.749 0 0 1-2.92-.757l-1.131-3.96a5.665 5.665 0 0 0-3.395-3.724l-1.63-.633a1.75 1.75 0 0 1-.603-2.869Zm6.589 12.912 4.293-4.294a.25.25 0 0 0-.108-.417L14.7 8.623A7.165 7.165 0 0 1 9.99 4.33L9.357 2.7a.25.25 0 0 0-.41-.086L2.614 8.946a.25.25 0 0 0 .086.41l1.63.634a7.167 7.167 0 0 1 4.294 4.71l1.13 3.96a.25.25 0 0 0 .417.108l4.294-4.293Z" /></svg>
   }
}