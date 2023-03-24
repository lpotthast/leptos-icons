#[cfg(feature = "IoFemaleOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoFemaleOutline")]
/// *This icon requires the feature* `IoFemaleOutline` *to be enabled*.
#[component]
pub fn FemaleOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><circle cx="256" cy="184" r="152" fill="none" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" /><line x1="256" y1="336" x2="256" y2="480" fill="none" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" /><line x1="314" y1="416" x2="198" y2="416" fill="none" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" /></svg>
   }
}