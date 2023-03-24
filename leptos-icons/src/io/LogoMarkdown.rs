#[cfg(feature = "IoLogoMarkdown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoLogoMarkdown")]
/// *This icon requires the feature* `IoLogoMarkdown` *to be enabled*.
#[component]
pub fn LogoMarkdown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M475,64H37C16.58,64,0,81.38,0,102.77V409.19C0,430.59,16.58,448,37,448H475c20.38,0,37-17.41,37-38.81V102.77C512,81.38,495.42,64,475,64ZM288,368H224V256l-48,64-48-64V368H64V144h64l48,80,48-80h64Zm96,0L304,256h48.05L352,144h64V256h48Z" /></svg>
   }
}