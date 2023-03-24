#[cfg(feature = "SiKahoot")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiKahoot")]
/// *This icon requires the feature* `SiKahoot` *to be enabled*.
#[component]
pub fn Kahoot(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M20.557 18.87l2.747-17.513L16.174 0zM.696 2.348v19.078l4.035.14-.035-6.679 2.487-2.4 2.626 9.078h3.565L10.087 9.722l4.957-5.444-3.496-1.339L4.73 9.443V1.322zm18.295 17.86l-.99 2.331L20.12 24l2.088-1.235-.887-2.556Z" /></svg>
   }
}