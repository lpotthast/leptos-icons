#[cfg(feature = "TbDeviceImacSearch")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbDeviceImacSearch")]
/// *This icon requires the feature* `TbDeviceImacSearch` *to be enabled*.
#[component]
pub fn DeviceImacSearch(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-device-imac-search" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M11 17h-7a1 1 0 0 1 -1 -1v-12a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v8" /><path d="M3 13h10" /><path d="M8 21h4" /><path d="M10 17l-.5 4" /><path d="M18 18m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /><path d="M20.2 20.2l1.8 1.8" /></svg>
   }
}