#[cfg(feature = "SiContactlesspayment")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiContactlesspayment")]
/// *This icon requires the feature* `SiContactlesspayment` *to be enabled*.
#[component]
pub fn Contactlesspayment(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M8.878 19.1c-2.861-.25-5.385-1.312-7.03-2.954-.936-.936-1.504-1.952-1.76-3.149-.118-.544-.117-1.448 0-1.997.357-1.664 1.433-3.12 3.126-4.23 3.862-2.533 9.778-2.52 13.604.03.508.338.836.607 1.296 1.06.58.573.988 1.142 1.418 1.98.016.03.095.07.176.086.27.056.633.268.924.54.226.21.401.44.832 1.093.435.658 2.404 3.697 2.524 3.896.046.078-.046.202-.15.202-.073 0-.126-.05-.234-.22l-1.116-1.732c-1.9-2.942-2.003-3.078-2.532-3.343l-.278-.14h-.817c-.816 0-.817.001-.837.087-.108.47-.128.586-.104.608.015.014.478.278 1.03.586l1.002.562-.062.131c-.035.072-.074.132-.088.132-.014 0-.858-.468-1.875-1.039-1.018-.57-1.91-1.054-1.982-1.074-.29-.08-.734.24-.734.529 0 .292.087.378 1.874 1.841 1.115.914 1.753 1.462 1.85 1.592.178.236.357.584.523 1.012.29.754.763 1.174 1.538 1.368l.255.064-.021.14a.667.667 0 01-.037.157c-.033.037-.547-.127-.844-.27-.465-.223-.785-.541-1.099-1.094a5.565 5.565 0 00-.405.139c-.391.142-.41.154-.78.515-1.576 1.538-3.836 2.521-6.566 2.855-.443.055-2.15.079-2.621.037zm2.717-.473c1.252-.166 2.223-.414 3.24-.83 1.067-.437 2.073-1.065 2.777-1.733l.277-.264-.698-.333c-.746-.357-.82-.416-.915-.735-.06-.2.008-.415.198-.621l.149-.161-.324-.246c-.256-.195-.336-.28-.386-.41a.65.65 0 01.079-.616l.098-.138-1.023-.18a26.154 26.154 0 01-1.166-.222c-.153-.046-.31-.22-.35-.391-.022-.092.48-2.54.592-2.89a.67.67 0 01.426-.376c.169-.031 3.185.499 3.335.587.238.138.313.362.238.705l-.032.144h.167c.092 0 .31-.012.483-.025l.316-.025-.157-.29c-.474-.882-1.33-1.768-2.388-2.472-.494-.328-1.547-.847-2.181-1.074a13.51 13.51 0 00-3.043-.674c-.575-.06-2.082-.06-2.64.001-1.339.146-2.554.449-3.617.902C2.59 7.309.935 9.046.512 11.027c-.11.517-.12 1.423-.018 1.894.224 1.04.713 1.941 1.509 2.78 1.62 1.707 4.097 2.756 7.066 2.994.397.031 2.121-.015 2.526-.068zm-.543-1.864c-.153-.044-.308-.227-.34-.4-.022-.114.004-.192.166-.506.656-1.27.941-2.437.941-3.855 0-1.37-.252-2.435-.879-3.713-.135-.274-.245-.529-.245-.565 0-.136.114-.333.237-.41.169-.107.452-.087.588.04.052.05.187.271.3.494.449.88.756 1.847.908 2.853.094.623.113 1.803.038 2.373a9.33 9.33 0 01-.86 2.909c-.27.555-.438.784-.58.784a.418.418 0 00-.101.015.421.421 0 01-.173-.019zm-1.826-.955c-.234-.069-.404-.357-.336-.573.017-.052.127-.293.246-.535.502-1.024.711-2.123.609-3.201-.084-.886-.243-1.445-.643-2.263-.269-.549-.281-.634-.125-.854.148-.21.519-.245.713-.066.135.124.555 1 .716 1.496a7.106 7.106 0 01-.232 5.016c-.352.826-.599 1.081-.948.98zm-1.882-.969a.57.57 0 01-.304-.374c-.021-.094.014-.198.187-.547.632-1.28.639-2.514.02-3.765-.271-.55-.277-.659-.047-.876.113-.107.161-.126.32-.126.267 0 .406.135.644.625.367.754.518 1.408.516 2.246 0 .86-.141 1.452-.533 2.247-.225.455-.336.575-.56.606a.525.525 0 01-.243-.036zm-1.757-.902a.592.592 0 01-.286-.34c-.043-.152-.008-.273.165-.557.222-.365.274-.56.274-1.038 0-.477-.052-.673-.274-1.038a2.046 2.046 0 01-.168-.329c-.071-.24.129-.535.398-.586.228-.043.385.06.595.387a2.96 2.96 0 010 3.155c-.215.332-.454.45-.704.346zm13.09 1.473c.203-.073.379-.141.39-.153.012-.011-.027-.141-.087-.289-.122-.302-.102-.295-.534-.212-.468.09-.739.01-1.25-.366-.14-.103-.283-.187-.32-.187-.036 0-.115.053-.177.119-.13.14-.15.36-.044.49.093.113 1.377.724 1.53.727.067.002.289-.056.492-.129zm-.202-.983c.19-.038.211-.051.178-.112-.036-.068-1.812-1.542-1.966-1.632-.127-.074-.23-.057-.335.056-.309.33-.273.4.526 1.013.964.738 1.066.782 1.597.675zm-2.384-2.231c0-.01-.201-.183-.448-.385-.608-.501-.738-.688-.738-1.058 0-.31.206-.608.538-.78a.966.966 0 01.625-.057c.118.038.648.325 1.383.748l.155.089.128-.606c.13-.62.133-.782.012-.82-.037-.011-.746-.14-1.575-.286-1.473-.26-1.634-.275-1.702-.164-.037.06-.597 2.656-.597 2.767 0 .055.028.124.062.152.034.029.512.132 1.062.23 1.062.191 1.095.196 1.095.17Z" /></svg>
   }
}