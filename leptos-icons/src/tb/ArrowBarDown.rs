#[cfg(feature = "TbArrowBarDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbArrowBarDown")]
/// *This icon requires the feature* `TbArrowBarDown` *to be enabled*.
#[component]
pub fn ArrowBarDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-arrow-bar-down" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 20l0 -10" /><path d="M12 20l4 -4" /><path d="M12 20l-4 -4" /><path d="M4 4l16 0" /></svg>
   }
}