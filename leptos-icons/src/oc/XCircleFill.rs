#[cfg(feature = "OcXCircleFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcXCircleFill")]
/// *This icon requires the feature* `OcXCircleFill` *to be enabled*.
#[component]
pub fn XCircleFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 12 12"><path d="M1.757 10.243a6.001 6.001 0 1 1 8.488-8.486 6.001 6.001 0 0 1-8.488 8.486ZM6 4.763l-2-2L2.763 4l2 2-2 2L4 9.237l2-2 2 2L9.237 8l-2-2 2-2L8 2.763Z" /></svg>
   }
}