#[cfg(feature = "SiFunimation")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiFunimation")]
/// *This icon requires the feature* `SiFunimation` *to be enabled*.
#[component]
pub fn Funimation(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M12 0a12 12 0 1 0 12 12A12.001 12.001 0 0 0 12 0ZM7.428 16.06h9.188s-.449 3.278-4.601 3.278c-4.15 0-4.587-3.278-4.587-3.278z" /></svg>
   }
}