#[cfg(feature = "FiGitPullRequest")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiGitPullRequest")]
/// *This icon requires the feature* `FiGitPullRequest` *to be enabled*.
#[component]
pub fn GitPullRequest(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="18" cy="18" r="3" /><circle cx="6" cy="6" r="3" /><path d="M13 6h3a2 2 0 0 1 2 2v7" /><line x1="6" y1="9" x2="6" y2="21" /></svg>
   }
}