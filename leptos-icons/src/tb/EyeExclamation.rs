#[cfg(feature = "TbEyeExclamation")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbEyeExclamation")]
/// *This icon requires the feature* `TbEyeExclamation` *to be enabled*.
#[component]
pub fn EyeExclamation(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-eye-exclamation" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M10 12a2 2 0 1 0 4 0a2 2 0 0 0 -4 0" /><path d="M14.473 17.659a8.897 8.897 0 0 1 -2.473 .341c-3.6 0 -6.6 -2 -9 -6c2.4 -4 5.4 -6 9 -6c3.6 0 6.6 2 9 6" /><path d="M19 16v3" /><path d="M19 22v.01" /></svg>
   }
}