#[cfg(feature = "TbHelpSmall")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbHelpSmall")]
/// *This icon requires the feature* `TbHelpSmall` *to be enabled*.
#[component]
pub fn HelpSmall(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-help-small" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 16v.01" /><path d="M12 13a2 2 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /></svg>
   }
}