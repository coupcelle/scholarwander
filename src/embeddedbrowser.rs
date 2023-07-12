use wry::{
    application::{
      event::{Event, StartCause, WindowEvent},
      event_loop::{ControlFlow, EventLoop},
      platform::run_return::EventLoopExtRunReturn,
      window::{WindowBuilder,Window},
    },
    webview::WebViewBuilder,
  };
pub use wry::application::event_loop::EventLoopProxy;

pub struct EmbeddedBrowser {


}

impl EmbeddedBrowser {

	pub fn show_page(window_title: Option<String>, url: String, title_callback: Option<impl Fn(&Window,EventLoopProxy<()>, String) + 'static> ) {
		
		let mut event_loop = EventLoop::new();
		let proxy = event_loop.create_proxy();
		let mut window_builder = WindowBuilder::new();
		if let Some(title) = window_title {
			window_builder = window_builder.with_title(title);
		}
		let window = window_builder.build(&event_loop).unwrap();
		
		let mut _webview_builder = WebViewBuilder::new(window).unwrap()
			.with_url(url.as_str()).unwrap();

		if let Some(callback) = title_callback {
			_webview_builder = _webview_builder.with_document_title_changed_handler(move | window: &Window, title: String | { callback(window, proxy, title) });
		}
			
		_webview_builder.build().unwrap();

		event_loop.run_return(move |event, _, control_flow| {
			*control_flow = ControlFlow::Wait;

			match event {
			Event::NewEvents(StartCause::Init) => println!("Opening browser window for SSO..."),
			Event::UserEvent(event) => *control_flow = ControlFlow::Exit,
			Event::WindowEvent {
				event: WindowEvent::CloseRequested,
				..
			} => *control_flow = ControlFlow::Exit,
			_ => (),
			}
		});


	}

}