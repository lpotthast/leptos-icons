#[cfg(feature = "TbDeviceLaptopOff")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbDeviceLaptopOff")]
/// *This icon requires the feature* `TbDeviceLaptopOff` *to be enabled*.
#[component]
pub fn DeviceLaptopOff(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-device-laptop-off" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M3 19h16" /><path d="M10 6h8a1 1 0 0 1 1 1v8m-3 1h-10a1 1 0 0 1 -1 -1v-8a1 1 0 0 1 1 -1" /><path d="M3 3l18 18" /></svg>
   }
}