#[cfg(feature = "SiReverbnation")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiReverbnation")]
/// *This icon requires the feature* `SiReverbnation` *to be enabled*.
#[component]
pub fn Reverbnation(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M24 9.324l-9.143-.03L11.971.57 9.143 9.294 0 9.324h.031l7.367 5.355-2.855 8.749h.029l7.459-5.386 7.396 5.386-2.855-8.73L24 9.315" /></svg>
   }
}