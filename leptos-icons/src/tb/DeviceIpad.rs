#[cfg(feature = "TbDeviceIpad")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbDeviceIpad")]
/// *This icon requires the feature* `TbDeviceIpad` *to be enabled*.
#[component]
pub fn DeviceIpad(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-device-ipad" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M20 5v14a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2z" /><path d="M9 18h6" /></svg>
   }
}