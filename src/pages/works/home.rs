use crate::app::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Works)]
pub fn works() -> Html {
    html! {
        <>
            <div class="container mx-auto px-5 flex-grow">
                <article class="prose max-w-none">
                    <h1>{"Works"}</h1>
                    <p>{"作ったものや関わっているプロジェクトなどのまとめです．"}</p>
                    <h2>{"個人開発"}</h2>
                    <div class="grid grid-cols-1 lg:grid-cols-3 gap-4">
                        <div class="card bg-base-200 shadow-xl not-prose">
                            <div class="card-body">
                                <h3 class="card-title text-2xl">{"DigiTatt"}</h3>
                                <div class="badge badge-outline h-auto">{"第1回 Kloud Discord Bot Contest 参加"}</div>
                                <p>{"削除されたメッセージを検知して復活させるDiscord用bot"}</p>
                                <div class="card-actions justify-end">
                                    <a role="button" class="btn btn-primary" href="https://github.com/NXVZBGBFBEN/DigiTatt" target="_blank" rel="noopener noreferrer">{"GitHub"}</a>
                                </div>
                            </div>
                        </div>
                        <div class="card bg-base-200 shadow-xl not-prose">
                            <div class="card-body">
                                <h3 class="card-title text-2xl">
                                    {"HSP3-IDEA"}
                                    <div class="badge badge-secondary">{"wip"}</div>
                                </h3>
                                <p>{"プログラミング言語HSP3のIntellij系IDE用プラグイン"}</p>
                                <div class="card-actions justify-end">
                                    <a role="button" class="btn btn-primary" href="https://github.com/NXVZBGBFBEN/HSP3-IDEA" target="_blank" rel="noopener noreferrer">{"GitHub"}</a>
                                </div>
                            </div>
                        </div>
                        <div class="card bg-base-200 shadow-xl not-prose">
                            <div class="card-body">
                                <h3 class="card-title text-2xl">{"MCServer-AutoBackup"}</h3>
                                <p>{"Minecraftサーバ用自動バックアップスクリプト"}</p>
                                <div class="card-actions justify-end">
                                    <a role="button" class="btn btn-primary" href="https://github.com/NXVZBGBFBEN/MCServer-AutoBackup" target="_blank" rel="noopener noreferrer">{"GitHub"}</a>
                                </div>
                            </div>
                        </div>
                        <div class="card bg-base-200 shadow-xl not-prose">
                            <div class="card-body">
                                <h3 class="card-title text-2xl">{"Medley"}</h3>
                                <p>{"LaTeXの構文を用いたCLI電卓"}</p>
                                <div class="card-actions justify-end">
                                    <a role="button" class="btn btn-primary" href="https://github.com/NXVZBGBFBEN/Medley" target="_blank" rel="noopener noreferrer">{"GitHub"}</a>
                                </div>
                            </div>
                        </div>
                    </div>
                    <h2>{"共同開発"}</h2>
                    <div class="grid grid-cols-1 lg:grid-cols-3 gap-4">
                        <div class="card bg-base-200 shadow-xl not-prose">
                            <div class="card-body">
                                <h3 class="card-title text-2xl">{"food-terror"}</h3>
                                <div class="badge badge-outline h-auto">{"Kloudハッカソン #2 アイデア賞"}</div>
                                <p>{"飯テロ画像を指定した日時に送信できるDiscord用bot"}</p>
                                <div class="card-actions justify-end">
                                    <a role="button" class="btn btn-primary" href="https://github.com/NXVZBGBFBEN/food-terror" target="_blank" rel="noopener noreferrer">{"GitHub"}</a>
                                </div>
                            </div>
                        </div>
                        <div class="card bg-base-200 shadow-xl not-prose">
                            <div class="card-body">
                                <h3 class="card-title text-2xl">{"KeTCindy"}</h3>
                                <p>{"動的幾何学ソフトCinderellaを用いてTeXの図版ファイルを作る"}</p>
                                <div class="card-actions justify-end">
                                    <Link<Route> classes={classes!("btn", "btn-primary")} to={Route::KeTCindy}>{"Project page"}</Link<Route>>
                                </div>
                            </div>
                        </div>
                        <div class="card bg-base-200 shadow-xl not-prose">
                            <div class="card-body">
                                <h3 class="card-title text-2xl">{"木更津高専統合管理BOT"}</h3>
                                <p>{"木更津高専生向け多機能Discord用bot"}</p>
                                <div class="card-actions justify-end">
                                    <a role="button" class="btn btn-primary" href="https://github.com/NITKC-DEV/Kisarazu-Multi-Manager" target="_blank" rel="noopener noreferrer">{"GitHub"}</a>
                                </div>
                            </div>
                        </div>
                        <div class="card bg-base-200 shadow-xl not-prose">
                            <div class="card-body">
                                <h3 class="card-title text-2xl">{"たすくりあ"}</h3>
                                <div class="badge badge-outline h-auto">{"Kloudハッカソン #3 参加"}</div>
                                <p>{"ボイスチャットに参加している時間を勉強時間とみなし，時間を記録するDiscord用bot"}</p>
                                <div class="card-actions justify-end">
                                    <a role="button" class="btn btn-primary" href="https://github.com/starkoka/Tasclear" target="_blank" rel="noopener noreferrer">{"GitHub"}</a>
                                </div>
                            </div>
                        </div>
                        <div class="card bg-base-200 shadow-xl not-prose">
                            <div class="card-body">
                                <h3 class="card-title text-2xl">{"木更津高専 祇園祭 ホームページ"}</h3>
                                <p>{"木更津高専の学園祭，「祇園祭」の公式ホームページ"}</p>
                                <div class="card-actions justify-end">
                                    <a role="button" class="btn btn-primary" href="https://gionsai.jp" target="_blank" rel="noopener noreferrer">{"Website"}</a>
                                </div>
                            </div>
                        </div>
                    </div>
                </article>
            </div>
        </>
    }
}
