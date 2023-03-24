#[cfg(feature = "ImFlattr")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImFlattr")]
/// *This icon requires the feature* `ImFlattr` *to be enabled*.
#[component]
pub fn Flattr(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M5.743 0c-3.802 0-5.743 2.19-5.743 6.279v0 8.579l3.725-3.729v-4.358c0-1.694 0.449-2.772 1.955-3.014v0c0.526-0.103 1.621-0.067 2.317-0.067v0 2.587c0 0.024 0.003 0.066 0.009 0.087v0c0.029 0.105 0.124 0.181 0.236 0.182v0c0.063 0 0.123-0.033 0.184-0.093v0l6.455-6.453-9.139-0.001zM12.275 4.871v4.358c0 1.694-0.449 2.772-1.955 3.014v0c-0.526 0.103-1.621 0.067-2.317 0.067v0-2.587c0-0.023-0.003-0.066-0.009-0.087v0c-0.029-0.105-0.124-0.182-0.236-0.182v0c-0.064-0-0.123 0.033-0.184 0.093v0l-6.455 6.453 9.139 0.001c3.802 0 5.743-2.19 5.743-6.279v0-8.579l-3.725 3.729z" /></svg>
   }
}