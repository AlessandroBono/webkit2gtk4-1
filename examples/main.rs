/*
 * Copyright (c) 2016 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

use gtk::{glib::ToVariant, prelude::*, ApplicationWindow};
use webkit2gtk::{prelude::*, WebContext, WebView};

fn main() {
    gtk::init().unwrap();
    let app = gtk::Application::new(None, Default::default());
    app.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        let context = WebContext::default().unwrap();
        context.set_web_extensions_initialization_user_data(&"webkit".to_variant());
        context.set_web_extensions_directory("../webkit2gtk-webextension-rs/example/target/debug/");
        let webview = WebView::with_context(&context);
        webview.load_uri("https://crates.io/");
        window.set_child(Some(&webview));

        let settings = WebViewExt::settings(&webview).unwrap();
        settings.set_enable_developer_extras(true);

        /*let inspector = webview.get_inspector().unwrap();
        inspector.show();*/

        window.show();

        webview.run_javascript("alert('Hello');", gtk::gio::Cancellable::NONE, |_result| {});
        webview.run_javascript("42", gtk::gio::Cancellable::NONE, |result| match result {
            Ok(result) => {
                let value = result.js_value().unwrap();
                println!("is_boolean: {}", value.is_boolean());
                println!("is_number: {}", value.is_number());
                println!("{:?}", value.to_boolean());
            }
            Err(error) => println!("{}", error),
        });
    });
    app.run();
}
