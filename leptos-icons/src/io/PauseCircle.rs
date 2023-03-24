#[cfg(feature = "IoPauseCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoPauseCircle")]
/// *This icon requires the feature* `IoPauseCircle` *to be enabled*.
#[component]
pub fn PauseCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M256,48C141.31,48,48,141.31,48,256s93.31,208,208,208,208-93.31,208-208S370.69,48,256,48ZM224,320a16,16,0,0,1-32,0V192a16,16,0,0,1,32,0Zm96,0a16,16,0,0,1-32,0V192a16,16,0,0,1,32,0Z" /></svg>
   }
}