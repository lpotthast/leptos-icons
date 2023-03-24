#[cfg(feature = "OcSmTabExternal")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmTabExternal")]
/// *This icon requires the feature* `OcSmTabExternal` *to be enabled*.
#[component]
pub fn TabExternal(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M3.25 4a.25.25 0 0 0-.25.25v9a.75.75 0 0 1-.75.75H.75a.75.75 0 0 1 0-1.5h.75V4.25c0-.966.784-1.75 1.75-1.75h9.5c.966 0 1.75.784 1.75 1.75v8.25h.75a.75.75 0 0 1 0 1.5h-1.5a.75.75 0 0 1-.75-.75v-9a.25.25 0 0 0-.25-.25h-9.5Z" /><path d="m7.97 7.97-2.75 2.75a.75.75 0 1 0 1.06 1.06l2.75-2.75 1.543 1.543a.25.25 0 0 0 .427-.177V6.25a.25.25 0 0 0-.25-.25H6.604a.25.25 0 0 0-.177.427L7.97 7.97Z" /></svg>
   }
}