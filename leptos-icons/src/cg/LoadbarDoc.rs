#[cfg(feature = "CgLoadbarDoc")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgLoadbarDoc")]
/// *This icon requires the feature* `CgLoadbarDoc` *to be enabled*.
#[component]
pub fn LoadbarDoc(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M17 5H7C6.44772 5 6 5.44772 6 6V18C6 18.5523 6.44772 19 7 19H17C17.5523 19 18 18.5523 18 18V6C18 5.44772 17.5523 5 17 5ZM7 3C5.34315 3 4 4.34315 4 6V18C4 19.6569 5.34315 21 7 21H17C18.6569 21 20 19.6569 20 18V6C20 4.34315 18.6569 3 17 3H7Z" fill="currentColor" /><path d="M8 7H16V9H8V7Z" fill="currentColor" /><path d="M8 11H16V13H8V11Z" fill="currentColor" /><path d="M8 15H13V17H8V15Z" fill="currentColor" /></svg>
   }
}