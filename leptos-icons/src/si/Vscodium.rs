#[cfg(feature = "SiVscodium")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiVscodium")]
/// *This icon requires the feature* `SiVscodium` *to be enabled*.
#[component]
pub fn Vscodium(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M15.29501.4152c.15768-.24864.43968-.414.73584-.4152.41496-.00096.81.23208 1.05024.564.22632.31728.29616.71832.29664 1.10136-.0036.53856-.12768 1.06775-.27552 1.58303-.36936 1.21608-.8712 2.38703-1.24608 3.60167-.4128 1.37832-.6108 2.91431-.05616 4.28279.28992-.80928.57672-1.62024.86568-2.43047.08208-.21408.19008-.45744.41952-.54528.31943-.11376.64511.2196.63575.52824-.08736.44951-.2568.87791-.39023 1.3152-1.08072 3.24862-2.29104 6.45237-3.45647 9.67172 1.54991-1.4172 3.10463-2.82911 4.65646-4.24367.44664-.40656.93096-.8496 1.06776-1.464.13656-.60311-.11736-1.20167-.40512-1.71959-.3324-.618-.75672-1.20575-.91272-1.90127-.07848-.36024-.05472-.77472.18816-1.06992.1752-.22224.49416-.33696.7632-.22848.26568.10152.426.35664.55152.5976a7.75846 7.75846 0 0 1 .7752 2.71175c.288-.58104.57935-1.15967.86783-1.74023.17424-.342.32808-.70896.6108-.9792.17064-.16848.41928-.27456.66-.21984.30192.06672.51384.34128.59496.6264.09192.30288-.0053.61824-.10296.906-.29088.80471-.74784 1.53287-1.18824 2.26175-.61392.9648-1.266 1.90631-1.97783 2.802.4068-.27169.8352-.55753 1.33343-.6144.47136-.06313.9924.21287 1.1484.67295.1044.3132-.06456.66768-.34824.82056-.41616.23208-.90864.22896-1.37112.2496-1.67831.04608-3.3139.8724-4.35526 2.18783-.82176 1.04904-1.24152 2.35247-2.07552 3.39287-.52607.63408-1.26935 1.43015-2.08703 1.25423-1.08887-.234-1.08887-1.93583-1.36031-2.77943-.13992-.42527-.40248-.81383-.7668-1.07903-.31848-.2352-.69984-.37008-1.08528-.444-.88488-.16992-1.7976-.08016-2.68031-.26256-.44184-.09-.87696-.26712-1.2144-.57264-.3216-.28224-.54383-.66-.69791-1.05527-.33264-.86112-.40032-1.79544-.696-2.66736-.42792-1.2552-1.27464-2.36495-2.37743-3.10199-.23256-.15696-.48528-.31007-.63576-.55487-.1224-.19128-.1116-.4632.04872-.63.27024-.29424.75864-.31848 1.07087-.07512.63984.45384.73224 1.3356 1.32864 1.82903.00096-.81455-.00312-1.6284-.00456-2.44247.00192-.2352-.02424-.4884.09336-.70224.10704-.20256.4032-.25752.57528-.1056.23256.19488.30336.51288.33648.80064.11568 1.122.22752 2.24447.34368 3.36719.58463-.52512.8532-1.37736.66407-2.14175-.05736-.24408-.15024-.49512-.09768-.74832.04296-.2124.26832-.36192.48072-.324.2388.0324.4092.23712.49872.44736.14256.34847.14304.73655.11424 1.10663-.08472.84288-.44136 1.6212-.68736 2.42328-.23976.708-.40464 1.49471-.1644 2.22599.21408.68376.75672 1.22423 1.39344 1.53071.786.38616 1.67135.50616 2.53751.54024-.52656-.9216-1.0992-1.89671-1.08936-2.99303.01032-1.10376.71208-2.03351.90168-3.09887.32304-1.76735-.47448-3.68855-1.95911-4.70254-.65952-.45312-1.45128-.65472-2.1288-1.07496-.61295-.37512-1.16351-1.0428-1.06727-1.80311.4656-.18024.98543-.06504 1.40567.17688.82128.43824 1.34376 1.26911 2.16912 1.70063a1.77383 1.77383 0 0 0 .95255.21624c.1788-.42864 0-.90864-.28848-1.24248-.54263-.65375-1.45847-.8124-2.02583-1.43735-.2844-.32616-.44304-.79224-.32352-1.21872.0972-.33696.44592-.5664.7932-.54096.51432.03192.95712.354 1.30872.70848 1.16327 1.1628 1.83263 2.7444 2.09519 4.35167.31008 1.89575.06912 3.82967-.31272 5.69638-.07128.3804-.20472.7704-.10608 1.15872.04248.17975.17208.36527.37128.38135.27384.01752.50016-.16896.6996-.3324.62376-.50951 1.1196-1.19543 1.32407-1.98095.15984-.57264.1644-1.17215.15312-1.76183.0031-.29496-.01872-.62064.15768-.876.126-.19728.3612-.35232.60312-.29496.1896.036.324.2004.38184.37656.09312.29016.07896.60336.0228.89928-.17832.81671-.3396 1.644-.66264 2.41943-.228.55656-.53592 1.08576-.95304 1.52184-.45983.48743-1.02791.87815-1.39847 1.44623-.25272.38808-.41616.87024-.30648 1.33392.08016.36983.36168.67175.70128.82583.49464.22584 1.09511.21432 1.57727-.03984.38256-.20688.65448-.56447.87096-.93191.38136-.66096.61248-1.39488.80424-2.12976.29496-1.18655.46872-2.4031.53688-3.62399.078-1.30391-.15864-2.65223-.816-3.79126-.34944-.60984-.8436-1.1292-1.4208-1.52664-.51984-.38664-1.09151-.72144-1.5132-1.22231-.39431-.46416-.66767-1.09056-.52415-1.70664.26976-.16944.61032-.19896.90912-.09288.56375.19176 1.00535.6444 1.29 1.15656.38663.6036.70919 1.32191 1.3903 1.64975-.55415-1.09007-1.12559-2.17079-1.68647-3.25703-.1824-.36696-.36216-.768-.31464-1.18775.03456-.31992.33168-.58224.65256-.5808.28488.00096.552.16128.72168.38448.19368.25128.28632.56088.37368.8616.44688 1.55567.89351 3.11062 1.34207 4.66582.56376-1.074.80208-2.3184.64176-3.52367-.07008-.63888-.25776-1.25591-.40656-1.8792-.10128-.40775-.0708-.86519.16176-1.22423z" /></svg>
   }
}