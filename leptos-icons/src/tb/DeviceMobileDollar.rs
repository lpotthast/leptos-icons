#[cfg(feature = "TbDeviceMobileDollar")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbDeviceMobileDollar")]
/// *This icon requires the feature* `TbDeviceMobileDollar` *to be enabled*.
#[component]
pub fn DeviceMobileDollar(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-device-mobile-dollar" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M13 21h-5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v5" /><path d="M11 4h2" /><path d="M12 17v.01" /><path d="M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" /><path d="M19 21v1m0 -8v1" /></svg>
   }
}