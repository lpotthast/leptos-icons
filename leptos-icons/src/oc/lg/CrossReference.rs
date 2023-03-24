#[cfg(feature = "OcLgCrossReference")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgCrossReference")]
/// *This icon requires the feature* `OcLgCrossReference` *to be enabled*.
#[component]
pub fn CrossReference(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M16.5 2.25a.75.75 0 0 1 .75-.75h5.5a.75.75 0 0 1 .75.75v5.5a.75.75 0 0 1-1.5 0V4.06l-6.22 6.22a.75.75 0 1 1-1.06-1.06L20.94 3h-3.69a.75.75 0 0 1-.75-.75Z" /><path d="M3.25 4a.25.25 0 0 0-.25.25v12.5c0 .138.112.25.25.25h2.5a.75.75 0 0 1 .75.75v3.19l3.72-3.72a.75.75 0 0 1 .53-.22h10a.25.25 0 0 0 .25-.25v-6a.75.75 0 0 1 1.5 0v6a1.75 1.75 0 0 1-1.75 1.75h-9.69l-3.573 3.573A1.457 1.457 0 0 1 5 21.043V18.5H3.25a1.75 1.75 0 0 1-1.75-1.75V4.25c0-.966.784-1.75 1.75-1.75h11a.75.75 0 0 1 0 1.5h-11Z" /></svg>
   }
}