#[cfg(feature = "SiRenpy")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiRenpy")]
/// *This icon requires the feature* `SiRenpy` *to be enabled*.
#[component]
pub fn Renpy(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M10.166 0S6.908 1.035 6.25 4.744C4.885 5.647 4.194 6.368 4.19 6.66c0 .023.02.042.031.063.115-.054 5.565-2.614 9.422-2.967a8.31 8.31 0 01.707-.03c1.8-.007 4.532.365 4.569.37-.063-.269-1.722-1.02-2.817-1.217C13.504.009 10.166 0 10.166 0zm8.754 4.096c-.039-.003-3.665-.261-5.342-.096-3.94.388-9.242 2.675-9.356 2.725.16.27 1.288.38 1.848.384-.085 1.63-.804 4.532-.674 4.553.09.014.295-.547.295-.547s-.105.322-.201.692c.404-.02.597-.04.851-.018.057.005.143.01.184.078.134-.26.137-.494.137-.494s.056.211.043.494c.096-.082.209-.316.209-.316s-.009.201-.018.324c.08.048.213.033.213.033a6.8 6.8 0 01-.092.592c-.062.312-.16.488-.4.803-.297.002-.617.244-.748.57-.02.17.246.37.246.37l-.39 1.896a1.743 1.743 0 00-.583.937s-.078.912-.013 1.23c.04.202 1 .846 1.671.89.397.025 1.19-.618 1.19-.618l.506-.049c.276.346.528.441.931.553l.11.34s-.182.149-.182.26c0 .11.197.214.197.214s-1.108.156-1.074 1.53c.018.728 1.01 1.074 1.824 1.105.855.033 2.012-.513 2.012-.513s.184.066.412.048c.262-.02.549-.205.549-.205s-.426.331-.436 1.268c-.002.25.172.537.364.7.114.095.3.17.412.171.173.001-.359-.495-.375-.813-.028-.541.69-.626 1.135-.626.471-.001 1.812.614 2.82.626 1.57.02 2.583-1.212 2.613-2.16.037-1.14-.951-1.834-1.304-2.21-.052-.233-.324-.722-.604-.782-.487-.104-.963.09-.963.09s-.112-.03-.223-.088c.794-1.087.876-1.977.887-2.728.032-2.214-1.539-3.268-1.762-3.39.371-.38.53-.64.622-.847a27.5 27.5 0 00-.18 1.051l.369-.326v.379s.44-.45.504-.914c.039.377.021.914.021.914s.552-.561.737-1.92c.035.862.066 1.508.066 1.508s.29-1.837.192-3.75c-.048-.928-.144-2.02-.47-2.746.825-.647 1.245-.959 1.227-1.16 0-.004-.007-.009-.007-.012zM10.504.25s.961.233 1.406.578c.737.573 1.562 1.75 1.562 1.75s-.98-1.165-1.625-1.625C11.451.67 10.504.25 10.504.25zm-.516.031s-.72.785-1 1.235c-.42.672-1 2.156-1 2.156s.358-1.511.766-2.156C9.064 1.024 9.988.28 9.988.28zm3.914 2.56c.813.013 1.465.054 1.465.054s-2.596.027-3.867.253C9.48 3.508 6.77 4.686 6.77 4.686S9.273 3.36 11.478 2.96c.638-.116 1.611-.132 2.424-.12zm-3.031 3.07s-.122.283-.23.552c-.056-.01-.112-.027-.165-.026.19-.26.395-.527.395-.527zm1.293.01s-.513 1.13-.598 2.433c.17-.534.299-.83.299-.83s-.064.815.12 1.425c.131-1.169.774-1.955.774-1.955s-.209.81-.12 1.37c.107-.826.398-1.69.398-1.69s-.187 1.808.209 2.64c.385-.368.718-1.193.718-1.193s.074.246.045.508c.506-.967.452-2.574.452-2.574s.122.406.26.998c-.19.13-.315.271-.315.271s.136-.113.326-.215c.08.347.155.75.223 1.176-.005 0-.009-.004-.014-.004-.62-.006-.944.26-1.047.594-.288.935.01 1.18.453 1.18.836-.001.961-1.149.961-1.149s.119.33.094.57c-.01.104-.102.274-.102.274l.336-.242s-.092-.74-.394-1.069c.145-.587.24-1.068.285-1.42.05.023.097.052.143.085.063.634.133 1.446.117 2.066.171-.286.31-.586.31-.586l.01.52s-.344.88-.906 2.033c-1.185.85-1.903 1.111-3.17 1.181-.478.027-1.305-.175-1.305-.175s-.556-1.118-1.037-1.106c-.288.007-.719.22-.719.22s-.542-1.166-.232-4.009c.041-.38.674-.928.674-.928s-.258.515-.414 1.284a5.42 5.42 0 01-.116.14s.093-.094.112-.115a5.608 5.608 0 00-.102.713c.123-.289.29-.541.29-.541s-.064.57.12.84c.355-.915.75-1.59 1.063-2.053.013-.003.028-.012.04-.014.053-.009.122.01.183.016a5.67 5.67 0 00-.354 1.531c-.886.086-1.043 1.293-1.043 1.293l.324.309s-.104-.216-.105-.34c.273.375.461.746 1.025.74 1.124-.013.951-1.56.951-1.56l.07.023s-.376-.51-1.171-.477l-.006.002a8.303 8.303 0 01.851-1.384c.128.051.16.068.252.115-.154.395-.29.859-.27 1.312.138-.467.311-.861.48-1.209l.074.037-.069-.048a9.273 9.273 0 01.567-1.012zm-.344.013s-.222.368-.42.863c-.08-.057-.105-.075-.219-.139.362-.464.639-.724.639-.724zm3.767.375l.067.644a.698.698 0 00-.117-.055c.031-.307.05-.59.05-.59zm-4.539 2.105c.068.002.135.05.17.123.05.1.024.214-.056.252-.08.04-.184-.01-.233-.111-.049-.1-.023-.214.057-.252a.133.133 0 01.062-.012zm4.04.106c.058.002.121.05.16.123.053.1.042.211-.024.25-.065.038-.162-.01-.215-.11-.053-.1-.042-.211.024-.25a.098.098 0 01.054-.013zm-3.83.714s-.004.734-.805.793c-.556.042-.805-.445-.805-.445l.106.04s.077-.066.156-.028c.089.043.097.168.097.168s.06.075.399.066c.633-.016.851-.594.851-.594zm3.824.102s-.145.637-.72.648c-.606.012-.48-.527-.48-.527s.058.343.5.379c.428.035.7-.5.7-.5zm-2.578 1.5c-.239-.003-.704.125-.704.125l.157.453-.016-.36s.347-.075.523-.077c.158-.003.47.054.47.054l-.063.406.25-.476s-.408-.123-.618-.125zm-2.915.318c.529-.084 1.037 1.046 1.133 1.166.174.217.616.38.672.696.064.359-.46.992-.46.992s-.046-.26-.212-.352c.102.179.102.446.102.446s-.103.755-1.406.976c-1.3.22-1.557-.389-1.557-.389s-.117-.312-.035-.691c-.157.168-.088.541-.088.541s-.828-.338-.943-.717c-.09-.295.188-.58.265-.879.133-.514.024-1.275.602-1.486.329-.12.848.067 1.093.022.285-.054.413-.257.834-.325zm.008.244c-.29.017-.447.375-.492.72-.049.374.14 1.124.14 1.124s-.104-.738-.046-1.101c.052-.33.177-.575.414-.579.446-.006.625.344.68.72.05.342.023 1.038.023 1.038s.095-.72.055-1.078c-.044-.382-.3-.87-.774-.844zm-1.726.227c-.355.066-.533.52-.532.941.002.478.547 1.317.547 1.317s-.414-.843-.414-1.297c0-.474.111-.744.407-.813.3-.069.584.2.734.563.129.312.101 1.008.101 1.008s.062-.714-.023-1.055c-.1-.402-.474-.728-.82-.664zm7.156.32c1.36.023 2.372 1.958 2.365 2.94-.014 2.137-.426 2.398-1.25 3.58-1.214 1.742-4.018 3.821-5.699 3.9-1.064.05-1.817-.482-1.814-.912.006-1.313.996-1.441.996-1.441 2.425-.29 2.477-.52 4.117-2.2 1.587-1.625 1.808-3.87.273-4.552l-.379-.135s.227-1.2 1.39-1.18zm-1.022.098l.028.178s-.382.309-.477.867l-.783-.373.016-.315c.56-.016 1.216-.357 1.216-.357zm-3.046.314l.347.067-.021.187zm.486.051s.339.052.508.055c.22.003.68-.055.68-.055l.017.457-.271.592-.977-.713zm-1.846.297c-.104 0-.188.106-.187.236 0 .13.084.235.187.235.104 0 .19-.104.19-.235 0-.13-.085-.237-.19-.236zm.059.078c.05 0 .091.043.092.096 0 .054-.041.098-.092.098s-.093-.044-.092-.098c0-.053.042-.096.092-.096zm-1.307.164a.153.153 0 00-.056.012c-.098.037-.14.166-.092.287.046.121.162.19.26.152.098-.035.14-.165.093-.287-.036-.097-.12-.164-.205-.164zm.05.059c.041-.004.08.02.097.06.02.051-.003.108-.051.125-.048.02-.102-.007-.121-.058-.02-.05.003-.105.05-.123a.087.087 0 01.026-.004zm4.423.037l.732.768.254.199.22-.586s.868.283.89 1.662l-1.127-.623-.077.182 1.19.695c-.02.162-.054.325-.108.49-.02-.03-.04-.068-.06-.072-.562.133-1.453.492-1.453.492l.008 1.834c-.154.166-.308.337-.461.49l-.073-2.515.207-.586c-.042-.06-.314-.449-.377-.702-.029-.118.033-.245.004-.363-.035-.144-.186-.246-.209-.392-.017-.117-.014-.308.067-.348.044-.022.127.027.127.076 0 .088-.08.096-.082.154-.008.193.19.343.23.532.025.113-.022.235.006.347.049.197.222.479.28.569l.519-1.465-.695-.748-.352.375zm-5.953.053s-.26.496-.182.736c.124.381 1.002.84 1.002.84s-.59.247-.437 1.148c.12.714 1.125.633 1.125.633s.048.538-.004.932c-.055.407-.383.845-.383.845s.408-.413.474-.828c.027-.166.018-.5.018-.5s.093.16.125.246c.07.188.14.582.14.582s-.017-.385-.054-.574a6.832 6.832 0 00-.207-.687l.363-.047s.08.25.344.285c.161.022.488.004.488.004s.004.3-.05.457c-.095.27-.47.75-.47.75s-.255.372-.456.426c-.265.07-.682.025-.682.025s-.82.69-1.275.69c-.424 0-1.418-.674-1.508-.823-.105-.173.014-.723.01-1.084-.005-.327.597-.949.597-.949l.42-1.96s-.294-.213-.25-.348c.157-.478.645-.489.645-.489zm4.613.166l.758.254-.778-.16zm-.36.57l1.725 1.818-.178.43.075 2.584c-1.202 1.182-3.051 1.203-3.051 1.203s-.202-.063-.217-.183c-.013-.106.158-.235.158-.235-.171-.552-.21-.736-.224-1.34l-.155-.144s.312-.425.448-.701c.088-.18.075-.327.072-.598 0 0 .373-.11.535-.215.251-.162.201-.385.201-.385s.262.06.602-.015a.89.89 0 00.678-.856c.01-.811-.883-.953-.883-.953zm-1.285.035a.273.273 0 00-.103.526.273.273 0 01.209-.45c.035.001.07.008.101.022a.273.273 0 00-.207-.098zm-1.148.227a.273.273 0 00-.273.273c0 .11.067.208.168.25a.273.273 0 01.21-.447c.035 0 .07.008.102.021a.273.273 0 00-.207-.097zm2.135.263s.699.024.857.76c.081.377-.301.828-.767.881-.111.013-.409-.037-.409-.037s-.054-.114-.154-.115c-.227-.003-.342.138-.72.185-.3.038-.386-.011-.575.022-.214.037-.267.207-.267.207s-.325.063-.524.066c-.771.014-.883-.294-.941-.74-.083-.634.486-.797.486-.797s.025.108.35.258l-.23.068.06.264-.215.066.027.287.254-.027.045.26.266-.012-.051-.303.227-.05.025-.442c.218.039.474.063.826.033.345-.028.602-.147.82-.28a.671.671 0 00-.056.269c0 .38.316.687.707.687.39 0 .707-.308.707-.687a.697.697 0 00-.87-.668c.075-.089.122-.155.122-.155zm3 .188l1.1.615c0 .028-.009.058-.01.086l-1.13-.644zm-2.807.037c.05.002.1.02.137.06l.01.01a.198.198 0 01-.008.28l-.477.447a.198.198 0 01-.279-.01l-.008-.01a.195.195 0 01.008-.277l.094-.09.033.074a.15.15 0 10.185.147.15.15 0 00-.125-.147l.01-.17.28-.261a.195.195 0 01.14-.053zm-.006.059a.15.15 0 10.016.3.15.15 0 00-.016-.3zm.244.34c.05.001.1.02.137.06l.01.01a.198.198 0 01-.008.28l-.476.446a.198.198 0 01-.28-.01l-.008-.01a.195.195 0 01.008-.277l.477-.447a.195.195 0 01.14-.053zm-.007.058a.15.15 0 10.002 0zm-3.008.047l.136.232-.109.024zm1.537.35a.081.081 0 00-.049.04l-.062.118a.087.087 0 00.035.117.088.088 0 00.117-.037l.06-.117a.085.085 0 00-.035-.116.089.089 0 00-.066-.006zm1.014.025a.15.15 0 10.002 0zm-1.483.057a.088.088 0 00-.05.043l-.061.117a.085.085 0 00.035.115.085.085 0 00.115-.035l.063-.12a.085.085 0 00-.037-.115.084.084 0 00-.065-.005zm5.397.357c.024.008.04.04.054.07a4.003 4.003 0 01-.285.608l-.765.279s-.015-.196-.028-.293c-.02-.148-.281-.172-.281-.172s.896-.376 1.305-.492zm-4.43.045c.06-.002.11.016.14.064.154.25-.203.55-.722.637-.66.111-.89-.2-.73-.437.09-.137.498-.035.675-.059.17-.023.455-.2.637-.205zm3.125.54c.077.01.193.031.197.124.007.138.028.414.028.414l.74-.27a7.746 7.746 0 01-.576.78l-.065-.365v.449c-.11.132-.216.258-.332.385zm-2.006.032c-.324.004-.646.04-.973.108-.128.183-.072.564-.072.564l2.133-.105s-.014-.334-.106-.487a5.44 5.44 0 00-.982-.08zm-.076.09c.263-.007.551.008.992.05.068.152.067.34.067.34l-1.934.087s-.014-.204.035-.387c.339-.053.577-.083.84-.09zm1.066.59s-.11.838-.023 1.32c-.462.113-1.205.137-1.828.11-.056-.511-.086-1.22-.086-1.22s-.08.55-.008 1.36c.714-.004 1.466-.006 2.031-.14-.067-.742-.086-1.43-.086-1.43zm-1.222.246s-.03.34-.028.512c.002.188.04.562.04.562s.045-.37.042-.555c-.002-.174-.054-.519-.054-.519zm.484.098s-.043.285-.043.43c0 .147.047.44.047.44s.028-.29.027-.437c0-.145-.031-.433-.031-.433zm-2.465.539l.133.105c-.003.25.006.502.098.79-.258-.034-.617-.217-.782-.45.298-.122.346-.167.551-.445zm8.694.078c.25.139.459.393.457.611-.003.254-.264-.15-.557-.404.233.249.397.632.215.637-.238.006-.255-.311-.602-.514-.184-.107-.496-.152-.496-.152s.371-.174.983-.178zm-1.073.273c.741-.002.851.642 1.137.64.313-.003.172-.28.172-.28s.106.151.21.136c0 0 1.301 1.029 1.321 1.989.022 1.08-1.257 2.173-2.312 2.164-1.381-.02-2.043-.67-3.008-.68-.725.034-1.213.328-1.227.727-.006.193.174.558.174.558s-.306-.258-.322-.527c-.041-.738.523-1.603 1.402-1.666.953-.11 1.383.232 2.613.322.48.035 1.162-.482 1.149-.766-.038-.823-1.73-.808-1.73-.808l.343-.309c-.023-1.231-.637-1.086-.637-1.086l.36-.406s.21-.007.355-.008zm-.808.516c.161-.025.325 0 .453.187.138.203.176.746.176.746s-.448.437-.735.547c-.01-.128-.007-.244-.023-.402-.013-.123-.172-.28-.362-.309v-.171c.175-.192.341-.392.49-.598zm-.563.674v1.513c-.136.058-.245.115-.414.172-.255.087-.544-.005-.805.063-.285.073-.53.26-.804.367-.409.16-.95.176-.95.176s1.708-.936 2.973-2.291zm.072.16c.127.025.282.153.288.256.012.26.022.344.03.504.158-.017.427-.262.427-.262s.042.256-.055.406c-.081.125-.349.271-.69.42zm.85.549s1.6.075 1.602.71c0 .4-.705.64-1.125.641-.571.002-1.54-.344-1.54-.344s.913-.39 1.032-.574c.097-.151.031-.433.031-.433z" /></svg>
   }
}