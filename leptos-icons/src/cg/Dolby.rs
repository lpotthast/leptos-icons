#[cfg(feature = "CgDolby")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgDolby")]
/// *This icon requires the feature* `CgDolby` *to be enabled*.
#[component]
pub fn Dolby(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M0 4V20H24V4H0ZM10 12C10 9.79086 8.20914 8 6 8H4V16H6C8.20914 16 10 14.2091 10 12ZM18 16H20V8H18C15.7909 8 14 9.79086 14 12C14 14.2091 15.7909 16 18 16Z" fill="currentColor" /></svg>
   }
}