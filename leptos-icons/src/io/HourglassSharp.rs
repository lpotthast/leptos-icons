#[cfg(feature = "IoHourglassSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoHourglassSharp")]
/// *This icon requires the feature* `IoHourglassSharp` *to be enabled*.
#[component]
pub fn HourglassSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M416,32H96V144L204,256,96,368V480H416V368L308,256,416,144ZM272,224V336l91,96H148l92-96V224l-80-80H352Z" /></svg>
   }
}