#[cfg(feature = "OcLgNoEntry")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgNoEntry")]
/// *This icon requires the feature* `OcLgNoEntry` *to be enabled*.
#[component]
pub fn NoEntry(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 1c6.075 0 11 4.925 11 11s-4.925 11-11 11S1 18.075 1 12 5.925 1 12 1ZM2.5 12a9.5 9.5 0 0 0 9.5 9.5 9.5 9.5 0 0 0 9.5-9.5A9.5 9.5 0 0 0 12 2.5 9.5 9.5 0 0 0 2.5 12Zm15.75.75H5.75a.75.75 0 0 1 0-1.5h12.5a.75.75 0 0 1 0 1.5Z" /></svg>
   }
}