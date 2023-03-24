#[cfg(feature = "TbBrandPepsi")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbBrandPepsi")]
/// *This icon requires the feature* `TbBrandPepsi` *to be enabled*.
#[component]
pub fn BrandPepsi(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-brand-pepsi" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /><path d="M4 16c5.713 -2.973 11 -3.5 13.449 -11.162" /><path d="M5 17.5c5.118 -2.859 15 0 14 -11" /></svg>
   }
}