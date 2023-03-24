#[cfg(feature = "OcSmPaperAirplane")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmPaperAirplane")]
/// *This icon requires the feature* `OcSmPaperAirplane` *to be enabled*.
#[component]
pub fn PaperAirplane(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M.989 8 .064 2.68a1.342 1.342 0 0 1 1.85-1.462l13.402 5.744a1.13 1.13 0 0 1 0 2.076L1.913 14.782a1.343 1.343 0 0 1-1.85-1.463L.99 8Zm.603-5.288L2.38 7.25h4.87a.75.75 0 0 1 0 1.5H2.38l-.788 4.538L13.929 8Z" /></svg>
   }
}