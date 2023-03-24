#[cfg(feature = "TbHighlight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbHighlight")]
/// *This icon requires the feature* `TbHighlight` *to be enabled*.
#[component]
pub fn Highlight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-highlight" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M3 19h4l10.5 -10.5a2.828 2.828 0 1 0 -4 -4l-10.5 10.5v4" /><path d="M12.5 5.5l4 4" /><path d="M4.5 13.5l4 4" /><path d="M21 15v4h-8l4 -4z" /></svg>
   }
}