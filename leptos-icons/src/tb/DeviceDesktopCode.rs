#[cfg(feature = "TbDeviceDesktopCode")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbDeviceDesktopCode")]
/// *This icon requires the feature* `TbDeviceDesktopCode` *to be enabled*.
#[component]
pub fn DeviceDesktopCode(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-device-desktop-code" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12.5 16h-8.5a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v8" /><path d="M7 20h4" /><path d="M9 16v4" /><path d="M20 21l2 -2l-2 -2" /><path d="M17 17l-2 2l2 2" /></svg>
   }
}