#[cfg(feature = "ImMic")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImMic")]
/// *This icon requires the feature* `ImMic` *to be enabled*.
#[component]
pub fn Mic(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M7.5 11c1.381 0 2.5-1.119 2.5-2.5v-6c0-1.381-1.119-2.5-2.5-2.5s-2.5 1.119-2.5 2.5v6c0 1.381 1.119 2.5 2.5 2.5zM11 7v1.5c0 1.933-1.567 3.5-3.5 3.5s-3.5-1.567-3.5-3.5v-1.5h-1v1.5c0 2.316 1.75 4.223 4 4.472v2.028h-2v1h5v-1h-2v-2.028c2.25-0.249 4-2.156 4-4.472v-1.5h-1z" /></svg>
   }
}