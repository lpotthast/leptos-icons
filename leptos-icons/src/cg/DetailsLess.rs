#[cfg(feature = "CgDetailsLess")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgDetailsLess")]
/// *This icon requires the feature* `CgDetailsLess` *to be enabled*.
#[component]
pub fn DetailsLess(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M3 9C2.44772 9 2 9.44772 2 10C2 10.5523 2.44772 11 3 11H21C21.5523 11 22 10.5523 22 10C22 9.44772 21.5523 9 21 9H3Z" fill="currentColor" /><path d="M3 13C2.44772 13 2 13.4477 2 14C2 14.5523 2.44772 15 3 15H15C15.5523 15 16 14.5523 16 14C16 13.4477 15.5523 13 15 13H3Z" fill="currentColor" /></svg>
   }
}