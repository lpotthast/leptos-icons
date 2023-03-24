#[cfg(feature = "ImAccessibility")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImAccessibility")]
/// *This icon requires the feature* `ImAccessibility` *to be enabled*.
#[component]
pub fn Accessibility(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M6.5 1.5c0-0.828 0.672-1.5 1.5-1.5s1.5 0.672 1.5 1.5c0 0.828-0.672 1.5-1.5 1.5s-1.5-0.672-1.5-1.5z" /><path fill="#000000" d="M10 5l5.15-2.221-0.371-0.929-6.279 2.15h-1l-6.279-2.15-0.371 0.929 5.15 2.221v4l-2.051 6.634 0.935 0.355 2.902-6.489h0.429l2.902 6.489 0.935-0.355-2.051-6.634z" /></svg>
   }
}