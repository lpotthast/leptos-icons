#[cfg(feature = "CgBee")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgBee")]
/// *This icon requires the feature* `CgBee` *to be enabled*.
#[component]
pub fn Bee(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M17.9513 15.571C17.7695 17.2187 16.9205 18.6654 15.6805 19.635C15.332 20.4692 14.7092 21.1601 13.9247 21.5951C13.7025 22.4574 12.9196 23.0944 11.9879 23.0944C11.0562 23.0944 10.2733 22.4574 10.0511 21.5951C9.26672 21.1602 8.64392 20.4693 8.29547 19.6353C7.05955 18.6691 6.21203 17.229 6.02621 15.5883C4.93527 16.4161 3.37955 16.4118 2.28618 15.4944C1.01696 14.4294 0.851409 12.5371 1.91641 11.2679L6.00966 6.38975C6.27125 3.31753 8.84785 0.905579 11.9878 0.905579C15.1166 0.905579 17.6861 3.30046 17.9631 6.35711L22.0837 11.2679C23.1487 12.5371 22.9831 14.4294 21.7139 15.4944C20.613 16.4182 19.0433 16.4161 17.9513 15.571ZM15.6254 12.9056L13.9472 10.9056H10.0529L8.37467 12.9056H15.6254ZM17.9878 12.6095L19.0195 13.839C19.3745 14.2621 20.0053 14.3173 20.4283 13.9623C20.8514 13.6073 20.9066 12.9765 20.5516 12.5535L17.9878 9.49803V12.6095ZM15.9878 8.90558V6.90558C15.9878 4.69644 14.1969 2.90558 11.9878 2.90558C9.77865 2.90558 7.98779 4.69644 7.98779 6.90558V8.90558H15.9878ZM4.98059 13.839L5.98779 12.6387V9.52726L3.4485 12.5535C3.0935 12.9765 3.14869 13.6073 3.57176 13.9623C3.99483 14.3173 4.62559 14.2621 4.98059 13.839ZM11.9878 18.9056C9.77865 18.9056 7.98779 17.1147 7.98779 14.9056H15.9878C15.9878 17.1147 14.1969 18.9056 11.9878 18.9056Z" fill="currentColor" /></svg>
   }
}