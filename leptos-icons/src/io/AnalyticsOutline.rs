#[cfg(feature = "IoAnalyticsOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoAnalyticsOutline")]
/// *This icon requires the feature* `IoAnalyticsOutline` *to be enabled*.
#[component]
pub fn AnalyticsOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><line x1="344" y1="280" x2="432" y2="192" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="232" y1="216" x2="296" y2="280" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="80" y1="320" x2="184" y2="216" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><circle cx="456" cy="168" r="24" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><circle cx="320" cy="304" r="24" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><circle cx="208" cy="192" r="24" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><circle cx="56" cy="344" r="24" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}