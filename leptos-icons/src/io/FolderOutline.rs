#[cfg(feature = "IoFolderOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoFolderOutline")]
/// *This icon requires the feature* `IoFolderOutline` *to be enabled*.
#[component]
pub fn FolderOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M440,432H72a40,40,0,0,1-40-40V120A40,40,0,0,1,72,80h75.89a40,40,0,0,1,22.19,6.72l27.84,18.56A40,40,0,0,0,220.11,112H440a40,40,0,0,1,40,40V392A40,40,0,0,1,440,432Z" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="32" y1="192" x2="480" y2="192" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}