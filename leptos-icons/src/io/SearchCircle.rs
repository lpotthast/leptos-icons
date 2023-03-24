#[cfg(feature = "IoSearchCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoSearchCircle")]
/// *This icon requires the feature* `IoSearchCircle` *to be enabled*.
#[component]
pub fn SearchCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M256,64C150.13,64,64,150.13,64,256s86.13,192,192,192,192-86.13,192-192S361.87,64,256,64Zm91.31,283.31a16,16,0,0,1-22.62,0l-42.84-42.83a88.08,88.08,0,1,1,22.63-22.63l42.83,42.84A16,16,0,0,1,347.31,347.31Z" /><circle cx="232" cy="232" r="56" /></svg>
   }
}