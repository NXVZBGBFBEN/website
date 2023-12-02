use horrorshow::helper::doctype;
use horrorshow::html;
use std::fs::File;
use std::io::Write;

pub fn main() {
    let html = html! {
        : doctype::HTML;
        html(lang="ja", data-bs-theme="dark") {
            head {
                meta(charset="utf-8");
                meta(name="viewport", content="width=device-width, initial-scale=1");
                title : "NXVZBGBFBEN";
                link(
                    href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css",
                    rel="stylesheet",
                    integrity="sha384-9ndCyUaIbzAi2FUVXJi0CjmCapSmO7SnpJef0486qhLnuZ2cdeRhO02iuK6FUUVM",
                    crossorigin="anonymous"
                );
                script(
                    src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/js/bootstrap.bundle.min.js",
                    integrity="sha384-geWF76RCwLtnZ8qwWowPQNguL3RmwHVBC9FhGdlKrxdiJJigb/j/68SIy3Te4Bkz",
                    crossorigin="anonymous"
                );
                link(
                    rel="stylesheet",
                    href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.10.5/font/bootstrap-icons.css"
                );
                link(data-trunk, rel="copy-dir", href="assets");
            }
        }
    };
    let mut file = File::create("index.html").expect("Could not create `index.html`.");
    writeln!(file, "{html}").expect("Could not write to `index.html`.")
}
