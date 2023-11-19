use yew::prelude::*;

#[function_component(KeTCindy)]
pub fn ketcindy() -> Html {
    html! {
        <>
            <div class="container">
                <main>
                    <section class="row">
                        <div class="col">
                            <p>{"最終更新: 2023-11-19"}</p>
                            <h1>{"KeTCindy"}</h1>
                            <hr/>
                            <p>{"KeTCindyは，動的幾何学ソフトCinderellaを用いて図形を描き，TeXの図版ファイルを作るためのシステムである．"}</p>
                            <p>
                                {"私がKeTCindyを初めて触ったのは2022年7月頃で，同年8月に "}
                                <a href="https://qiita.com/NXVZBGBFBEN/items/c18a0c89ce7a9e2ae700" target="_blank" rel="noopener noreferrer">{"Qiitaの記事"}</a>
                                {" を執筆した．\
                                その後，2023年2月から継続的に，\
                                私の通学先である木更津高専でKeTCindyの開発に参画なさっている 山下 哲 教授 のセミナを受講し，技術を学ぶとともに\
                                インストール手順や使い方をまとめた "}
                                <a href="https://github.com/NXVZBGBFBEN/KeTCindyBook" target="_blank" rel="noopener noreferrer">{"KeTCindyスタートブック"}</a>
                                {" の執筆を共同で進めている．"}
                            </p>
                        </div>
                    </section>
                    <section class="row">
                        <div class="col">
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
                        </div>
                    </section>
                    <section class="row">
                        <div class="col">
                            <h2>{"KeTCindyスタートブック"}</h2>
                            <p>
                                {"ここでは，KeTCindyスタートブック(インストール手順の説明のみ)と，その補助となる外部リンクを紹介する．\
                                質問等は nxvzbgbfben(at)gmail.com で受け付けている．((at)は@に置換すること．)"}</p>
                            <p>
                                <a href="/assets/documents/KeTCindyBook.pdf" target="_blank" rel="noopener noreferrer">{"KeTCindyスタートブック"}</a>{" (PDF 376KB)"}
                            </p>
                            <table class="table">
                                <tbody>
                                    <tr>
                                        <th>{"R"}</th>
                                        <td><a href="https://cran.r-project.org/" target="_blank" rel="noopener noreferrer">{"https://cran.r-project.org/"}</a></td>
                                    </tr>
                                    <tr>
                                        <th>{"Cinderella"}</th>
                                        <td><a href="https://cinderella.de/" target="_blank" rel="noopener noreferrer">{"https://cinderella.de/"}</a></td>
                                    </tr>
                                    <tr>
                                        <th>{"Maxima"}</th>
                                        <td><a href="https://maxima.sourceforge.io/" target="_blank" rel="noopener noreferrer">{"https://maxima.sourceforge.io/"}</a></td>
                                    </tr>
                                    <tr>
                                        <th>{"MinGW-w64"}</th>
                                        <td><a href="https://www.mingw-w64.org/" target="_blank" rel="noopener noreferrer">{"https://www.mingw-w64.org/"}</a></td>
                                    </tr>
                                    <tr>
                                        <th>{"SumatraPDF"}</th>
                                        <td><a href="https://www.sumatrapdfreader.org/free-pdf-reader" target="_blank" rel="noopener noreferrer">{"https://www.sumatrapdfreader.org/free-pdf-reader"}</a></td>
                                    </tr>
                                    <tr>
                                        <th>{"emath"}</th>
                                        <td><a href="http://emath.s40.xrea.com/" target="_blank" rel="noopener noreferrer">{"http://emath.s40.xrea.com/"}</a></td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                    </section>
                    <section class="row">
                        <div class="col">
                            <h2>{"リンク集"}</h2>
                            <ul>
                                <li><a href="https://s-takato.github.io/ketcindyorg/indexj.html" target="_blank" rel="noopener noreferrer">{"KeTCindy Home"}</a></li>
                                <li><a href="https://sites.google.com/site/ketcindy/home" target="_blank" rel="noopener noreferrer">{"KeTCindy (Google Sites)"}</a></li>
                                <li><a href="https://www.youtube.com/@ketcindy" target="_blank" rel="noopener noreferrer">{"Project KeTCindy (YouTube)"}</a></li>
                            </ul>
                        </div>
                    </section>
                </main>
            </div>
        </>
    }
}
