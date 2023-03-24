#[cfg(feature = "TbArrowsDownUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbArrowsDownUp")]
/// *This icon requires the feature* `TbArrowsDownUp` *to be enabled*.
#[component]
pub fn ArrowsDownUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-arrows-down-up" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M17 3l0 18" /><path d="M10 18l-3 3l-3 -3" /><path d="M7 21l0 -18" /><path d="M20 6l-3 -3l-3 3" /></svg>
   }
}