#[cfg(feature = "ImDroplet")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImDroplet")]
/// *This icon requires the feature* `ImDroplet` *to be enabled*.
#[component]
pub fn Droplet(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M13.51 7.393c-1.027-2.866-3.205-5.44-5.51-7.393-2.305 1.953-4.482 4.527-5.51 7.393-0.635 1.772-0.698 3.696 0.197 5.397 1.029 1.955 3.104 3.21 5.313 3.21s4.284-1.255 5.313-3.21c0.895-1.701 0.832-3.624 0.197-5.397zM11.543 11.859c-0.684 1.301-2.075 2.141-3.543 2.141-0.861 0-1.696-0.29-2.377-0.791 0.207 0.027 0.416 0.041 0.627 0.041 1.835 0 3.573-1.050 4.428-2.676 0.701-1.333 0.64-2.716 0.373-3.818 0.227 0.44 0.42 0.878 0.576 1.311 0.353 0.985 0.625 2.443-0.084 3.791z" /></svg>
   }
}