#[cfg(feature = "OcAlertFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcAlertFill")]
/// *This icon requires the feature* `OcAlertFill` *to be enabled*.
#[component]
pub fn AlertFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 12 12"><path d="M4.855.708c.5-.896 1.79-.896 2.29 0l4.675 8.351a1.312 1.312 0 0 1-1.146 1.954H1.33A1.313 1.313 0 0 1 .183 9.058ZM7 7V3H5v4Zm-1 3a1 1 0 1 0 0-2 1 1 0 0 0 0 2Z" /></svg>
   }
}