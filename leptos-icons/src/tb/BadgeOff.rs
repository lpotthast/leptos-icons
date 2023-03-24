#[cfg(feature = "TbBadgeOff")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbBadgeOff")]
/// *This icon requires the feature* `TbBadgeOff` *to be enabled*.
#[component]
pub fn BadgeOff(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-badge-off" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M7 7v10l5 3l5 -3m0 -4v-9l-5 3l-2.496 -1.497" /><path d="M3 3l18 18" /></svg>
   }
}