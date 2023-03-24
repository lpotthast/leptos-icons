#[cfg(feature = "CgSandClock")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgSandClock")]
/// *This icon requires the feature* `CgSandClock` *to be enabled*.
#[component]
pub fn SandClock(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M13 6H11V7C11 7.55228 11.4477 8 12 8C12.5523 8 13 7.55228 13 7V6Z" fill="currentColor" /><path fill-rule="evenodd" clip-rule="evenodd" d="M6 2V4H7V7C7 9.76142 9.23858 12 12 12C9.23858 12 7 14.2386 7 17V20H6V22H18V20H17V17C17 14.2386 14.7614 12 12 12C14.7614 12 17 9.76142 17 7V4H18V2H6ZM9 4H15V7C15 8.65685 13.6569 10 12 10C10.3431 10 9 8.65685 9 7V4ZM9 17V20H15V17C15 15.3431 13.6569 14 12 14C10.3431 14 9 15.3431 9 17Z" fill="currentColor" /></svg>
   }
}