#[cfg(feature = "FiDroplet")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiDroplet")]
/// *This icon requires the feature* `FiDroplet` *to be enabled*.
#[component]
pub fn Droplet(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 2.69l5.66 5.66a8 8 0 1 1-11.31 0z" /></svg>
   }
}