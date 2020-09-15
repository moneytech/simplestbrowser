#![windows_subsystem = "windows"]

use gtk::*;
use gtk::prelude::*;
use gio::prelude::*;
use std::env::args;
use webkit2gtk::{WebContext, WebView, WebViewExt};

fn main() {

//  Declare gtk::Application
    let application = gtk::Application::new(Some("org.skylinecc.SimplestBrowser"), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {

//      Load the compiled resource bundle
        let resources_bytes = include_bytes!("../data/resources.gresource");
        let resource_data = glib::Bytes::from(&resources_bytes[..]);
        let res = gio::Resource::new_from_data(&resource_data).unwrap();
        gio::resources_register(&res);

//      Load the window UI
        let builder = Builder::new_from_resource("/org/skylinecc/SimplestBrowser/window.ui");

//      Get a reference to the window
        let window: gtk::ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
        window.set_application(Some(app));

//      Reference boxes and buttons for the window
        let window_box: gtk::Box = builder.get_object("window_box").unwrap();
        let forward_button: gtk::Button = builder.get_object("forward_button").unwrap();
        let back_button: gtk::Button = builder.get_object("back_button").unwrap();
        let search_entry: gtk::Entry = builder.get_object("search_bar").unwrap();

//      Declare Webview and WebContext
    	let context = WebContext::get_default().unwrap();
		let webview = WebView::new_with_context(&context);

//      Set basic Webview settings
		webview.load_uri("https://duckduckgo.com/");
		window_box.pack_start(&webview, true, true, 0);

//      Clone webview for back & forward button actions :/
		let webview_back = webview.clone();
		let webview_forward = webview.clone();

//      Search Entry handler, set URL as text in search_entry when <enter> is pressed.
		search_entry.connect_activate(move |search_entry|{
			let url = search_entry.get_text().unwrap();
			webview.load_uri(&url);
		});

//      Go forwards and backwards pages when the buttons are clicked
		forward_button.connect_clicked(move |_| {
			webview_forward.go_forward();
		});

		back_button.connect_clicked(move |_| {
			webview_back.go_back();
		});



//      Show the UI and finish up
        window.show_all();
    });

    application.run(&args().collect::<Vec<_>>());
}
