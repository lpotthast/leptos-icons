#[cfg(feature = "CgImport")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgImport")]
/// *This icon requires the feature* `CgImport` *to be enabled*.
#[component]
pub fn Import(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M5 9.98193V19.9819H19V9.98193H15V7.98193H21V21.9819H3V7.98193H9V9.98193H5Z" fill="currentColor" /><path d="M13.0001 2H11.0001V14.0531L8.46451 11.5175L7.05029 12.9317L12 17.8815L16.9498 12.9317L15.5356 11.5175L13.0001 14.053V2Z" fill="currentColor" /></svg>
   }
}