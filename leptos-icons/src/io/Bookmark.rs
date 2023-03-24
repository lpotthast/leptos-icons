#[cfg(feature = "IoBookmark")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoBookmark")]
/// *This icon requires the feature* `IoBookmark` *to be enabled*.
#[component]
pub fn Bookmark(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M400,480a16,16,0,0,1-10.63-4L256,357.41,122.63,476A16,16,0,0,1,96,464V96a64.07,64.07,0,0,1,64-64H352a64.07,64.07,0,0,1,64,64V464a16,16,0,0,1-16,16Z" /></svg>
   }
}