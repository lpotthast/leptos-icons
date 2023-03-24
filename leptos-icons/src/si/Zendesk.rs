#[cfg(feature = "SiZendesk")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiZendesk")]
/// *This icon requires the feature* `SiZendesk` *to be enabled*.
#[component]
pub fn Zendesk(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M12.914 2.904V16.29L24 2.905H12.914zM0 2.906C0 5.966 2.483 8.45 5.543 8.45s5.542-2.484 5.543-5.544H0zm11.086 4.807L0 21.096h11.086V7.713zm7.37 7.84c-3.063 0-5.542 2.48-5.542 5.543H24c0-3.06-2.48-5.543-5.543-5.543z" /></svg>
   }
}