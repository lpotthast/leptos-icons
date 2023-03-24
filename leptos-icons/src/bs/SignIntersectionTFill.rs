#[cfg(feature = "BsSignIntersectionTFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BsSignIntersectionTFill")]
/// *This icon requires the feature* `BsSignIntersectionTFill` *to be enabled*.
#[component]
pub fn SignIntersectionTFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-sign-intersection-t-fill" viewBox="0 0 16 16"><path d="M9.05.435c-.58-.58-1.52-.58-2.1 0L.436 6.95c-.58.58-.58 1.519 0 2.098l6.516 6.516c.58.58 1.519.58 2.098 0l6.516-6.516c.58-.58.58-1.519 0-2.098L9.05.435ZM5 5h6v1.5H8.75V12h-1.5V6.5H5V5Z" /></svg>
   }
}