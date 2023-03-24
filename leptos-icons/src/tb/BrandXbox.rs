#[cfg(feature = "TbBrandXbox")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbBrandXbox")]
/// *This icon requires the feature* `TbBrandXbox` *to be enabled*.
#[component]
pub fn BrandXbox(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-brand-xbox" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /><path d="M6.5 5c7.72 2.266 10.037 7.597 12.5 12.5" /><path d="M17.5 5c-7.72 2.266 -10.037 7.597 -12.5 12.5" /></svg>
   }
}