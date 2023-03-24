#[cfg(feature = "FiAlignJustify")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiAlignJustify")]
/// *This icon requires the feature* `FiAlignJustify` *to be enabled*.
#[component]
pub fn AlignJustify(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="21" y1="10" x2="3" y2="10" /><line x1="21" y1="6" x2="3" y2="6" /><line x1="21" y1="14" x2="3" y2="14" /><line x1="21" y1="18" x2="3" y2="18" /></svg>
   }
}