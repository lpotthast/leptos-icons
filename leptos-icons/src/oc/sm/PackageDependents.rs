#[cfg(feature = "OcSmPackageDependents")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmPackageDependents")]
/// *This icon requires the feature* `OcSmPackageDependents` *to be enabled*.
#[component]
pub fn PackageDependents(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M6.122.392a1.75 1.75 0 0 1 1.756 0l5.25 3.045c.54.313.872.89.872 1.514V7.25a.75.75 0 0 1-1.5 0V5.677L7.75 8.432v6.384a1 1 0 0 1-1.502.865L.872 12.563A1.75 1.75 0 0 1 0 11.049V4.951c0-.624.332-1.2.872-1.514ZM7.125 1.69a.248.248 0 0 0-.25 0l-4.63 2.685L7 7.133l4.755-2.758ZM1.5 11.049a.25.25 0 0 0 .125.216l4.625 2.683V8.432L1.5 5.677Zm10.828 3.684 1.173-1.233H10.25a.75.75 0 0 1 0-1.5h3.251l-1.173-1.233a.75.75 0 1 1 1.087-1.034l2.378 2.5a.75.75 0 0 1 0 1.034l-2.378 2.5a.75.75 0 0 1-1.087-1.034Z" /></svg>
   }
}