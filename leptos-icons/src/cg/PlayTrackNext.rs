#[cfg(feature = "CgPlayTrackNext")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgPlayTrackNext")]
/// *This icon requires the feature* `CgPlayTrackNext` *to be enabled*.
#[component]
pub fn PlayTrackNext(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M6 17L14 12L6 7V17Z" fill="currentColor" /><path d="M18 7H15V12V17H18V7Z" fill="currentColor" /></svg>
   }
}