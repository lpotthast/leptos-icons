#[cfg(feature = "TbChevronsUpRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbChevronsUpRight")]
/// *This icon requires the feature* `TbChevronsUpRight` *to be enabled*.
#[component]
pub fn ChevronsUpRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-chevrons-up-right" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M9 7h8v8" /><path d="M5 11h8v8" /></svg>
   }
}