use tauri::{WebviewUrl, WebviewWindowBuilder};

const STORE_MANAGER_URL: &str = "https://cflex-store-manager.swigglesmac9.chatgpt.site";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External(STORE_MANAGER_URL.parse().expect("valid store URL")),
            )
            .title("C.Flex Store Manager")
            .inner_size(1280.0, 820.0)
            .min_inner_size(360.0, 640.0)
            .resizable(true)
            .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running C.Flex Store Manager");
}
