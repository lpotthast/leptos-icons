#[cfg(feature = "ImStarFull")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImStarFull")]
/// *This icon requires the feature* `ImStarFull` *to be enabled*.
#[component]
pub fn StarFull(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M16 6.204l-5.528-0.803-2.472-5.009-2.472 5.009-5.528 0.803 4 3.899-0.944 5.505 4.944-2.599 4.944 2.599-0.944-5.505 4-3.899z" /></svg>
   }
}