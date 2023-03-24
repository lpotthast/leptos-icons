#[cfg(feature = "OcSmFeedDiscussion")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmFeedDiscussion")]
/// *This icon requires the feature* `OcSmFeedDiscussion` *to be enabled*.
#[component]
pub fn FeedDiscussion(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M8 16A8 8 0 1 1 8 0a8 8 0 0 1 0 16ZM4 5v5a1 1 0 0 0 1 1h1v1.5a.5.5 0 0 0 .854.354L8.707 11H11a1 1 0 0 0 1-1V5a1 1 0 0 0-1-1H5a1 1 0 0 0-1 1Z" /></svg>
   }
}