#[cfg(feature = "OcLgColumns")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgColumns")]
/// *This icon requires the feature* `OcLgColumns` *to be enabled*.
#[component]
pub fn Columns(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M3.75 2h5.5c.966 0 1.75.784 1.75 1.75v16.5A1.75 1.75 0 0 1 9.25 22h-5.5A1.75 1.75 0 0 1 2 20.25V3.75C2 2.784 2.784 2 3.75 2Zm11 0h5.5c.966 0 1.75.784 1.75 1.75v16.5A1.75 1.75 0 0 1 20.25 22h-5.5A1.75 1.75 0 0 1 13 20.25V3.75c0-.966.784-1.75 1.75-1.75ZM3.5 3.75v16.5c0 .138.112.25.25.25h5.5a.25.25 0 0 0 .25-.25V3.75a.25.25 0 0 0-.25-.25h-5.5a.25.25 0 0 0-.25.25Zm11 0v16.5c0 .138.112.25.25.25h5.5a.25.25 0 0 0 .25-.25V3.75a.25.25 0 0 0-.25-.25h-5.5a.25.25 0 0 0-.25.25Z" /></svg>
   }
}