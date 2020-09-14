use gtk::*;
use gtk::prelude::*;
use gio::prelude::*;
use std::env::args;
use webkit2gtk::{SettingsExt, WebContext, WebContextExt, WebView, WebViewExt};

fn main() {
    let application = gtk::Application::new(Some("org.skylinecc.SimplestBrowser"), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {

        // Load the compiled resource bundle
        let resources_bytes = include_bytes!("../data/resources.gresource");
        let resource_data = glib::Bytes::from(&resources_bytes[..]);
        let res = gio::Resource::new_from_data(&resource_data).unwrap();
        gio::resources_register(&res);

        // Load the window UI
        let builder = Builder::new_from_resource("/org/skylinecc/SimplestBrowser/window.ui");

        // Get a reference to the window
        let window: ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
        window.set_application(Some(app));

        let window_box: gtk::Box = builder.get_object("window_box").unwrap();
//        let forward_button: gtk::Button = builder.get_object("forward_button").unwrap();
        let back_button: gtk::Button = builder.get_object("back_button").unwrap();
        let search_entry: gtk::Entry = builder.get_object("search_bar").unwrap();

    	let context = WebContext::get_default().unwrap();
		let webview = WebView::new_with_context(&context);

		webview.load_uri("https://duckduckgo.com/");
		window_box.pack_start(&webview, true, true, 0);

		let webview_clone1 = webview.clone();
		let webview_clone2 = webview.clone();

		search_entry.connect_activate(move |search_entry|{
			let url = search_entry.get_text().unwrap();
			webview.load_uri(&url);
		});

//		forward_button.connect_activate(move |forward_button| {
//			let webview = webview.clone();
//			webview.go_forward();
//		});

		back_button.connect_activate(move |_| {
			let webview_clone1 = webview.clone();
			webview.go_back();
		});



        // Show the UI
        window.show_all();
    });

    application.run(&args().collect::<Vec<_>>());
}




