#[cfg(feature = "TbDeviceIpadX")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbDeviceIpadX")]
/// *This icon requires the feature* `TbDeviceIpadX` *to be enabled*.
#[component]
pub fn DeviceIpadX(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-device-ipad-x" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M22 22l-5 -5" /><path d="M17 22l5 -5" /><path d="M13 21h-7a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v9" /><path d="M9 18h4" /></svg>
   }
}