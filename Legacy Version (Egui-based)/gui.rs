use druid::{widget::{Button, Label}, AppLauncher, Color, Data, Lens, Widget, WidgetExt, WindowDesc};
use crate::network::check_url;
use crate::security::setup_secure_client;
use crate::proxy::setup_proxy;
use tokio::runtime::Runtime;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub url: String,
}

pub fn ui_builder() -> impl Widget<AppState> {
    let label = Label::new("Enter URL to check:")
        .padding(10.0)
        .background(Color::grey(0.8));
    
    let button = Button::new("Check URL")
        .on_click(|_ctx, data: &mut AppState, _env| {
            let url = data.url.clone();
            let result = check_url(&url);
            println!("{}", result);
        })
        .padding(10.0);

    label.padding(10.0).center().padding(10.0)
        .with_child(button)
}

pub fn check_url(url: &str) -> String {
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(check_url(url));
    
    match result {
        Ok(status) => status,
        Err(err) => format!("Error: {}", err),
    }
}
