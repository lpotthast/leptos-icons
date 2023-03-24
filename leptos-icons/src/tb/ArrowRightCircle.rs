#[cfg(feature = "TbArrowRightCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbArrowRightCircle")]
/// *This icon requires the feature* `TbArrowRightCircle` *to be enabled*.
#[component]
pub fn ArrowRightCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-arrow-right-circle" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M18 15l3 -3l-3 -3" /><path d="M5 12m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /><path d="M7 12h14" /></svg>
   }
}