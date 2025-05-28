use web_view::*;

fn main() {
    web_view::builder()
        .title("Ghayb Browser")
        .content(Content::Url("https://github.com/mehedyk"))
        .size(1024, 768)
        .resizable(true)
        .debug(false)
        .user_data(())
        .invoke_handler(|_webview, arg| {
            if arg == "exit" {
                std::process::exit(0);
            }
            Ok(())
        })
        .run()
        .unwrap();
}
