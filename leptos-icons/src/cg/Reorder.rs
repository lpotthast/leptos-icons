#[cfg(feature = "CgReorder")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgReorder")]
/// *This icon requires the feature* `CgReorder` *to be enabled*.
#[component]
pub fn Reorder(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M3 4C3 3.44772 3.44772 3 4 3H12C12.5523 3 13 3.44772 13 4C13 4.55228 12.5523 5 12 5H4C3.44772 5 3 4.55228 3 4Z" fill="currentColor" fill-opacity="0.5" /><path d="M3 12C3 11.4477 3.44772 11 4 11H12C12.5523 11 13 11.4477 13 12C13 12.5523 12.5523 13 12 13H4C3.44772 13 3 12.5523 3 12Z" fill="currentColor" fill-opacity="0.5" /><path d="M3 16C3 15.4477 3.44772 15 4 15H12C12.5523 15 13 15.4477 13 16C13 16.5523 12.5523 17 12 17H4C3.44772 17 3 16.5523 3 16Z" fill="currentColor" fill-opacity="0.5" /><path d="M3 20C3 19.4477 3.44772 19 4 19H12C12.5523 19 13 19.4477 13 20C13 20.5523 12.5523 21 12 21H4C3.44772 21 3 20.5523 3 20Z" fill="currentColor" fill-opacity="0.5" /><path fill-rule="evenodd" clip-rule="evenodd" d="M15.1707 9C15.5825 10.1652 16.6938 11 18 11C19.6569 11 21 9.65685 21 8C21 6.34315 19.6569 5 18 5C16.6938 5 15.5825 5.83481 15.1707 7H4C3.44772 7 3 7.44772 3 8C3 8.55228 3.44772 9 4 9H15.1707ZM19 8C19 8.55228 18.5523 9 18 9C17.4477 9 17 8.55228 17 8C17 7.44772 17.4477 7 18 7C18.5523 7 19 7.44772 19 8Z" fill="currentColor" /></svg>
   }
}