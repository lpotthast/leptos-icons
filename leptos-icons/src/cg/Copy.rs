#[cfg(feature = "CgCopy")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgCopy")]
/// *This icon requires the feature* `CgCopy` *to be enabled*.
#[component]
pub fn Copy(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M13 7H7V5H13V7Z" fill="currentColor" /><path d="M13 11H7V9H13V11Z" fill="currentColor" /><path d="M7 15H13V13H7V15Z" fill="currentColor" /><path fill-rule="evenodd" clip-rule="evenodd" d="M3 19V1H17V5H21V23H7V19H3ZM15 17V3H5V17H15ZM17 7V19H9V21H19V7H17Z" fill="currentColor" /></svg>
   }
}