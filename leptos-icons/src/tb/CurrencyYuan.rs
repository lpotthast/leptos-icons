#[cfg(feature = "TbCurrencyYuan")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbCurrencyYuan")]
/// *This icon requires the feature* `TbCurrencyYuan` *to be enabled*.
#[component]
pub fn CurrencyYuan(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-currency-yuan" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 19v-7l-5 -7" /><path d="M17 5l-5 7" /><path d="M8 13h8" /></svg>
   }
}