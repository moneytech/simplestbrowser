use gtk::*;
use gtk::prelude::*;
use gio::prelude::*;
use std::env::args;
use gtk::License::Gpl30;
use webkit2gtk::{WebContext, WebView, WebViewExt, LoadEvent};
use gdk_pixbuf::Pixbuf;

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
        let quit_button: gtk::MenuItem = builder.get_object("quit_button").unwrap();
        let about_button: gtk::MenuItem = builder.get_object("about_button").unwrap();

//      Declare Webview and WebContext
    	let context = WebContext::get_default().unwrap();
		let webview = WebView::new_with_context(&context);

//      Set basic Webview settings
		webview.load_uri("https://duckduckgo.com/");
		window_box.pack_start(&webview, true, true, 0);

//      Clone webview for back & forward button actions :/
		let webview_back = webview.clone();
		let webview_forward = webview.clone();
		let webview_entry = webview.clone();

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

        quit_button.connect_activate(move |_| {
            window.destroy();
        });

        about_button.connect_activate(move |_| {
            about();
        });

        webview_entry.connect_load_changed(move |webview_entry, event| if event == LoadEvent::Finished {
            let url = format!("{}", webview_entry.get_uri().unwrap());
            search_entry.set_text(&url);
            println!("Loading {}", url);
        });

    });

    application.run(&args().collect::<Vec<_>>());
}

fn about() {

	let about_window = AboutDialog::new();
	about_window.set_website_label(Some("Simplest Browser"));
	about_window.set_website(Some("https://github.com/skylinecc/finance"));
	about_window.set_website_label(Some("Simplest Browser Project Page"));
	about_window.set_comments(Some("A super simple webkitGTK browser written in Rust."));
	about_window.set_copyright(Some("Copyright © 2020 Skyline Coding Club"));
	about_window.set_license_type(Gpl30);
	about_window.set_wrap_license(false);
	about_window.set_logo(Some(&Pixbuf::new_from_resource("/org/skylinecc/SimplestBrowser/org.skylinecc.SimplestBrowser.svg").unwrap()));
	about_window.set_title("About SimplestBrowser");
	about_window.set_authors(&["Grant Handy"]);
    about_window.add_credit_section(&"Club Members", &[
        "Nicholas Zhang",
		"Ethan Suresh",
		"Alex Ikeda",
		"Evan Ikeda",
		"Corrine Wang",
		"Miguel Oyarzun",
		"Grant Handy",
		"Michael Donnely",
		"Ayush Ranjan",
		"Alex Rose",
	]);
    about_window.show_all();

}
