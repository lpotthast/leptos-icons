#[cfg(feature = "TbDeviceMobileQuestion")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbDeviceMobileQuestion")]
/// *This icon requires the feature* `TbDeviceMobileQuestion` *to be enabled*.
#[component]
pub fn DeviceMobileQuestion(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-device-mobile-question" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M15 21h-7a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v6" /><path d="M19 22v.01" /><path d="M19 19a2.003 2.003 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /><path d="M11 4h2" /><path d="M12 17v.01" /></svg>
   }
}