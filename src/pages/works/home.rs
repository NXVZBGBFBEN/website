use crate::app::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Works)]
pub fn works() -> Html {
    html! {
        <>
            <div class="container">
                <main>
                    <section class="row">
                        <div class="col">
                            <h1>{"Works"}</h1>
                            <hr/>
                            <p>{"作ったものや関わっているプロジェクトなどのまとめです．"}</p>
                        </div>
                    </section>
                    <h2>{"個人開発"}</h2>
                    <section class="row mb-3 g-2">
                        <div class="col-md-4">
                            <div class="card">
                                <div class="card-body">
                                    <h5 class="card-title">{"DigiTatt"}</h5>
                                    <h6 class="card-subtitle mb-2 text-muted">{"第1回 Kloud Discord Bot Contest 参加"}</h6>
                                    <p class="card-text">{"削除されたメッセージを検知して復活させるDiscord用bot"}</p>
                                    <a href="https://github.com/NXVZBGBFBEN/DigiTatt" target="_blank" rel="noopener noreferrer">{"GitHub"}</a>
                                </div>
                            </div>
                        </div>
                        <div class="col-md-4">
                            <div class="card">
                                <div class="card-body">
                                    <h5 class="card-title">{"HSP3-IDEA "}<span class="badge rounded-pill bg-secondary">{"wip"}</span></h5>
                                    <p class="card-text">{"プログラミング言語HSP3のIntellij系IDE用プラグイン"}</p>
                                    <a href="https://github.com/NXVZBGBFBEN/HSP3-IDEA" target="_blank" rel="noopener noreferrer">{"GitHub"}</a>
                                </div>
                            </div>
                        </div>
                        <div class="col-md-4">
                            <div class="card">
                                <div class="card-body">
                                    <h5 class="card-title">{"MCServer-AutoBackup"}</h5>
                                    <p class="card-text">{"Minecraftサーバ用自動バックアップスクリプト"}</p>
                                    <a href="https://github.com/NXVZBGBFBEN/MCServer-AutoBackup" target="_blank" rel="noopener noreferrer">{"GitHub"}</a>
                                </div>
                            </div>
                        </div>
                        <div class="col-md-4">
                            <div class="card">
                                <div class="card-body">
                                    <h5 class="card-title">{"Medley"}</h5>
                                    <p class="card-text">{"LaTeXの構文を用いたCLI電卓"}</p>
                                    <a href="https://github.com/NXVZBGBFBEN/Medley" target="_blank" rel="noopener noreferrer">{"GitHub"}</a>
                                </div>
                            </div>
                        </div>
                    </section>
                    <h2>{"共同開発"}</h2>
                    <section class="row g-2">
                        <div class="col-md-4">
                            <div class="card">
                                <div class="card-body">
                                    <h5 class="card-title">{"food-terror"}</h5>
                                    <h6 class="card-subtitle mb-2 text-muted">{"Kloudハッカソン #2 アイデア賞"}</h6>
                                    <p class="card-text">{"飯テロ画像を指定した日時に送信できるDiscord用bot"}</p>
                                    <a href="https://github.com/NXVZBGBFBEN/food-terror" target="_blank" rel="noopener noreferrer">{"GitHub"}</a>
                                </div>
                            </div>
                        </div>
                        <div class="col-md-4">
                            <div class="card">
                                <div class="card-body">
                                    <h5 class="card-title">{"KeTCindy"}</h5>
                                    <p class="card-text">{"動的幾何学ソフトCinderellaを用いてTeXの図版ファイルを作る"}</p>
                                    <Link<Route> to={Route::KeTCindy}>{"Project page"}</Link<Route>>
                                </div>
                            </div>
                        </div>
                        <div class="col-md-4">
                            <div class="card">
                                <div class="card-body">
                                    <h5 class="card-title">{"木更津高専統合管理BOT"}</h5>
                                    <p class="card-text">{"木更津高専生向け多機能Discord用bot"}</p>
                                    <a href="https://github.com/NITKC-DEV/Kisarazu-Multi-Manager" target="_blank" rel="noopener noreferrer">{"GitHub"}</a>
                                </div>
                            </div>
                        </div>
                        <div class="col-md-4">
                            <div class="card">
                                <div class="card-body">
                                    <h5 class="card-title">{"たすくりあ"}</h5>
                                    <h6 class="card-subtitle mb-2 text-muted">{"Kloudハッカソン #3 参加"}</h6>
                                    <p class="card-text">{"ボイスチャットに参加している時間を勉強時間とみなし，時間を記録するDiscord用bot"}</p>
                                    <a href="https://github.com/starkoka/Tasclear" target="_blank" rel="noopener noreferrer">{"GitHub"}</a>
                                </div>
                            </div>
                        </div>
                        <div class="col-md-4">
                            <div class="card">
                                <div class="card-body">
                                    <h5 class="card-title">{"木更津高専 祇園祭 ホームページ"}</h5>
                                    <p class="card-text">{"木更津高専の学園祭，「祇園祭」の公式ホームページ"}</p>
                                    <a href="https://gionsai.jp" target="_blank" rel="noopener noreferrer">{"https://gionsai.jp"}</a>
                                </div>
                            </div>
                        </div>
                    </section>
                </main>
            </div>
        </>
    }
}
