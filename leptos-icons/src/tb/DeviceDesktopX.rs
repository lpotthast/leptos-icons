#[cfg(feature = "TbDeviceDesktopX")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbDeviceDesktopX")]
/// *This icon requires the feature* `TbDeviceDesktopX` *to be enabled*.
#[component]
pub fn DeviceDesktopX(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-device-desktop-x" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M13 16h-9a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v8" /><path d="M7 20h6.5" /><path d="M9 16v4" /><path d="M22 22l-5 -5" /><path d="M17 22l5 -5" /></svg>
   }
}