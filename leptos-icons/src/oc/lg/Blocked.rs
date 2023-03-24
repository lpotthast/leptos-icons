#[cfg(feature = "OcLgBlocked")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgBlocked")]
/// *This icon requires the feature* `OcLgBlocked` *to be enabled*.
#[component]
pub fn Blocked(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M7.638 2.22a.749.749 0 0 1 .53-.22h7.664c.199 0 .389.079.53.22l5.418 5.418c.141.14.22.332.22.53v7.664a.749.749 0 0 1-.22.53l-5.418 5.418a.749.749 0 0 1-.53.22H8.168a.749.749 0 0 1-.53-.22l-5.42-5.418a.752.752 0 0 1-.219-.53V8.168c0-.199.079-.389.22-.53l5.418-5.42ZM8.48 3.5 3.5 8.48v7.04l4.98 4.98h7.04l4.98-4.98V8.48L15.52 3.5ZM7 11.75a.75.75 0 0 1 .75-.75h8.5a.75.75 0 0 1 0 1.5h-8.5a.75.75 0 0 1-.75-.75Z" /></svg>
   }
}