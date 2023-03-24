#[cfg(feature = "IoJournal")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoJournal")]
/// *This icon requires the feature* `IoJournal` *to be enabled*.
#[component]
pub fn Journal(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M290,32H144A64.07,64.07,0,0,0,80,96V416a64.07,64.07,0,0,0,64,64H290Z" /><path d="M368,32H350V480h18a64.07,64.07,0,0,0,64-64V96A64.07,64.07,0,0,0,368,32Z" /></svg>
   }
}