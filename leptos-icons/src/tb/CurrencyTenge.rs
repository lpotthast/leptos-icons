#[cfg(feature = "TbCurrencyTenge")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbCurrencyTenge")]
/// *This icon requires the feature* `TbCurrencyTenge` *to be enabled*.
#[component]
pub fn CurrencyTenge(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-currency-tenge" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M6 5h12" /><path d="M6 9h12" /><path d="M12 9v10" /></svg>
   }
}