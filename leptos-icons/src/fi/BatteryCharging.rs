#[cfg(feature = "FiBatteryCharging")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiBatteryCharging")]
/// *This icon requires the feature* `FiBatteryCharging` *to be enabled*.
#[component]
pub fn BatteryCharging(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 18H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h3.19M15 6h2a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-3.19" /><line x1="23" y1="13" x2="23" y2="11" /><polyline points="11 6 7 12 13 12 9 18" /></svg>
   }
}