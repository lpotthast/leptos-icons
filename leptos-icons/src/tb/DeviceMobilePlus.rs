#[cfg(feature = "TbDeviceMobilePlus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbDeviceMobilePlus")]
/// *This icon requires the feature* `TbDeviceMobilePlus` *to be enabled*.
#[component]
pub fn DeviceMobilePlus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-device-mobile-plus" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12.5 21h-4.5a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v7" /><path d="M16 19h6" /><path d="M19 16v6" /><path d="M11 4h2" /><path d="M12 17v.01" /></svg>
   }
}