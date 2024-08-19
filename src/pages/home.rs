use crate::components::target_blank_link::*;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <div class="container mx-auto px-5 flex-grow">
                <div class="hero">
                    <div class="hero-content flex-col lg:flex-row">
                        <img class="avatar w-32 rounded mx-auto" src="/assets/images/USERICON2_2-1920x1920.png" />
                        <div class="divider lg:divider-horizontal"></div>
                        <div class="prose">
                            <h3>{"Profile"}</h3>
                            <p>
                                {"情報工学科所属の高専生です．"}<br/>
                                {"最近はサイバーセキュリティ，言語・数式処理系，教材作成などに興味があります．"}
                            </p>
                            <p>
                                {"所有資格・受賞歴等は"}
                                <TargetBlankLink text={"LinkedInアカウント"} href={"https://www.linkedin.com/in/akimoto-sora/"} />
                                {"に掲載しています．"}
                            </p>
                            <p>
                                {"各種サービスで使用しているユーザアイコン及びヘッダ画像は"}
                                <TargetBlankLink text={"友人"} href={"https://twitter.com/Nimono_blend"} />
                                {"が制作したものです．"}
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </>
    }
}
