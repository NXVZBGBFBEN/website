use horrorshow::helper::doctype;
use horrorshow::html;
use std::fs::File;
use std::io::Write;

pub fn main() {
    /* generate `index.html` */
    let html = html! {
        : doctype::HTML;
        html(lang="ja", data-theme="dim") {
            head {
                meta(charset="utf-8");
                meta(name="viewport", content="width=device-width, initial-scale=1");
                title : "NXVZBGBFBEN";
                link(data-trunk, rel="copy-dir", href="assets");
                link(data-trunk, rel="css", href="./tailwind.css");
            }
            body(class="flex flex-col min-h-screen");
        }
    };
    let mut index_html = File::create("index.html").expect("Could not create `index.html`.");
    writeln!(index_html, "{html}").expect("Could not write to `index.html`.");

    /* generate `tailwind.config.js` */
    let tailwind_config =
    r#"
    module.exports = {
        content: [
            "./src/**/*.rs",
            "./index.html",
        ],
        theme: {
            extend: {},
        },
        plugins: [
            require("@tailwindcss/typography"),
            require("daisyui"),
        ],
        daisyui: {
            themes: ["dim"],
        },
    }
    "#;
    let mut tailwind_config_js = File::create("tailwind.config.js").expect("Could not create `tailwind.config.js`.");
    writeln!(tailwind_config_js, "{tailwind_config}").expect("Could not write to `tailwind.config.js`.");

    /* generate `input.css` */
    let input =
    r#"
    @tailwind base;
    @tailwind components;
    @tailwind utilities;
    "#;
    let mut input_css = File::create("input.css").expect("Could not create `input.css`.");
    writeln!(input_css, "{input}").expect("Could not write to `input.css`.");
}
