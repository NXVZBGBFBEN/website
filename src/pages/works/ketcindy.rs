use crate::components::target_blank_link::*;
use yew::prelude::*;

#[function_component(KeTCindy)]
pub fn ketcindy() -> Html {
    html! {
        <>
            <div class="container mx-auto px-5 flex-grow">
                <article class="prose max-w-none">
                    <p>{"最終更新: 2023-11-19"}</p>
                    <h1>{"KeTCindy"}</h1>
                    <p>{"KeTCindyは，動的幾何学ソフトCinderellaを用いて図形を描き，TeXの図版ファイルを作るためのシステムである．"}</p>
                    <p>
                        {"私がKeTCindyを初めて触ったのは2022年7月頃で，同年8月に "}
                        <TargetBlankLink text={"Qiitaの記事"} href={"https://qiita.com/NXVZBGBFBEN/items/c18a0c89ce7a9e2ae700"} />
                        {" を執筆した．\
                        その後，2023年2月から継続的に，\
                        私の通学先である木更津高専でKeTCindyの開発に参画なさっている 山下 哲 教授 のセミナを受講し，技術を学ぶとともに\
                        インストール手順や使い方をまとめた "}
                        <TargetBlankLink text={"KeTCindyスタートブック"} href={"https://github.com/NXVZBGBFBEN/KeTCindyBook"} />
                        {" の執筆を共同で進めている．"}
                        <h2>{"KeTCindyの利点"}</h2>
                        <p>
                            {"KeTCindyを用いるメリットは，簡単に(La)TeX文書で用いることのできる図版を作成できる点にあると思う．\
                            図版を作成する場合，ドローイングソフトやTikZパッケージを用いる方法もあるが，正確な図を描くことができなかったり，\
                            複雑なコマンドを駆使する必要がある．一方KeTCindyは，Cinderellaという作図に特化したソフトを用いることができ，\
                            また，数式処理システムMaximaやgccなどの周辺ソフトウェアと連携することで，3D描画などの高度な機能も利用できる．"}
                        </p>
                        <p>
                            {"さらに，アニメーションや変数の値を入力できる動的な図をHTMLファイルに書き出すことで，\
                            オンライン教材の作成にも役立てることができる．"}
                        </p>
                        <h2>{"KeTCindyスタートブック"}</h2>
                        <p>
                            {"ここでは，KeTCindyスタートブック(インストール手順の説明のみ)と，その補助となる外部リンクを紹介する．\
                            質問等は nxvzbgbfben(at)gmail.com で受け付けている．((at)は@に置換すること．)"}</p>
                        <TargetBlankLink text={"KeTCindyスタートブック (PDF 376KB)"} href={"/assets/documents/KeTCindyBook.pdf"} />
                        <div class="overflow-x-auto">
                            <table class="table">
                                <tbody>
                                    <tr>
                                        <th>{"R"}</th>
                                        <td><TargetBlankLink text={"https://cran.r-project.org/"} href={"https://cran.r-project.org/"} /></td>
                                    </tr>
                                    <tr>
                                        <th>{"Cinderella"}</th>
                                        <td><TargetBlankLink text={"https://cran.r-project.org/"} href={"https://cinderella.de/"} /></td>
                                    </tr>
                                    <tr>
                                        <th>{"Maxima"}</th>
                                        <td><TargetBlankLink text={"https://maxima.sourceforge.io/"} href={"https://maxima.sourceforge.io/"} /></td>
                                    </tr>
                                    <tr>
                                        <th>{"MinGW-w64"}</th>
                                        <td><TargetBlankLink text={"https://www.mingw-w64.org/"} href={"https://www.mingw-w64.org/"} /></td>
                                    </tr>
                                    <tr>
                                        <th>{"SumatraPDF"}</th>
                                        <td><TargetBlankLink text={"https://www.sumatrapdfreader.org/free-pdf-reader"} href={"https://www.sumatrapdfreader.org/free-pdf-reader"} /></td>
                                    </tr>
                                    <tr>
                                        <th>{"emath"}</th>
                                        <td><TargetBlankLink text={"http://emath.s40.xrea.com/"} href={"http://emath.s40.xrea.com/"} /></td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                        <h2>{"リンク集"}</h2>
                        <ul class="list-disc">
                            <li><TargetBlankLink text={"KeTCindy Home"} href={"https://s-takato.github.io/ketcindyorg/indexj.html"} /></li>
                            <li><TargetBlankLink text={"KeTCindy (Google Sites)"} href={"https://sites.google.com/site/ketcindy/home"} /></li>
                            <li><TargetBlankLink text={"Project KeTCindy (YouTube)"} href={"https://www.youtube.com/@ketcindy"} /></li>
                        </ul>
                    </p>
                </article>
            </div>
        </>
    }
}
