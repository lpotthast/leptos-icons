#[cfg(feature = "TbBaselineDensityLarge")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbBaselineDensityLarge")]
/// *This icon requires the feature* `TbBaselineDensityLarge` *to be enabled*.
#[component]
pub fn BaselineDensityLarge(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-baseline-density-large" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M4 4h16" /><path d="M4 20h16" /></svg>
   }
}