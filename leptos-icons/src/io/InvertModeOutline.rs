#[cfg(feature = "IoInvertModeOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoInvertModeOutline")]
/// *This icon requires the feature* `IoInvertModeOutline` *to be enabled*.
#[component]
pub fn InvertModeOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><circle fill="none" stroke="#000" stroke-miterlimit="10" stroke-width="32" cx="256" cy="256" r="208" /><path d="M256,176V336a80,80,0,0,1,0-160Z" /><path d="M256,48V176a80,80,0,0,1,0,160V464c114.88,0,208-93.12,208-208S370.88,48,256,48Z" /></svg>
   }
}