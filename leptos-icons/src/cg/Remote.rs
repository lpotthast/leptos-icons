#[cfg(feature = "CgRemote")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgRemote")]
/// *This icon requires the feature* `CgRemote` *to be enabled*.
#[component]
pub fn Remote(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M17.0514 4.32178L18.4656 5.73599L14.223 9.97863L18.4656 14.2213L17.0514 15.6355L11.3946 9.97863L17.0514 4.32178Z" fill="currentColor" /><path d="M6.94864 19.6785L5.53442 18.2643L9.77706 14.0216L5.53442 9.77897L6.94864 8.36476L12.6055 14.0216L6.94864 19.6785Z" fill="currentColor" /></svg>
   }
}