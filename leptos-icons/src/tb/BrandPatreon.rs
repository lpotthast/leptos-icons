#[cfg(feature = "TbBrandPatreon")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbBrandPatreon")]
/// *This icon requires the feature* `TbBrandPatreon` *to be enabled*.
#[component]
pub fn BrandPatreon(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-brand-patreon" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M3 3h3v18h-3z" /><path d="M15 9.5m-6.5 0a6.5 6.5 0 1 0 13 0a6.5 6.5 0 1 0 -13 0" /></svg>
   }
}