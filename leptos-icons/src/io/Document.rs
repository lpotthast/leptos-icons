#[cfg(feature = "IoDocument")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoDocument")]
/// *This icon requires the feature* `IoDocument` *to be enabled*.
#[component]
pub fn Document(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M428,224H288a48,48,0,0,1-48-48V36a4,4,0,0,0-4-4H144A64,64,0,0,0,80,96V416a64,64,0,0,0,64,64H368a64,64,0,0,0,64-64V228A4,4,0,0,0,428,224Z" /><path d="M419.22,188.59,275.41,44.78A2,2,0,0,0,272,46.19V176a16,16,0,0,0,16,16H417.81A2,2,0,0,0,419.22,188.59Z" /></svg>
   }
}