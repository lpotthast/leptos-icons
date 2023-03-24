#[cfg(feature = "IoMaleSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoMaleSharp")]
/// *This icon requires the feature* `IoMaleSharp` *to be enabled*.
#[component]
pub fn MaleSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M330,48V92h58.89L328.5,152.39c-68.2-52.86-167-48-229.54,14.57h0C31.12,234.81,31.12,345.19,99,413A174.21,174.21,0,0,0,345,413c62.57-62.58,67.43-161.34,14.57-229.54L420,123.11V182h44V48ZM313.92,381.92a130.13,130.13,0,0,1-183.84,0c-50.69-50.68-50.69-133.16,0-183.84s133.16-50.69,183.84,0S364.61,331.24,313.92,381.92Z" /></svg>
   }
}