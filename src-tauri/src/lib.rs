// Shared entry point for desktop and mobile. On mobile, Tauri builds this as a
// library and the Android/iOS host calls `run()` via the generated entry point.

#[cfg(desktop)]
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    Manager, WindowEvent,
};

#[cfg(desktop)]
fn show_main(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

// Open a URL in the user's real web browser. On desktop the web app calls this
// for links that leave the messenger so they open in the browser while the
// messenger window stays put. On mobile the whole site lives in one fullscreen
// webview and in-app navigation is the right behaviour, so the web app
// navigates in place instead of calling this (see openOffMessenger), making
// this a harmless no-op there.
#[tauri::command]
fn open_external(url: String) {
    #[cfg(desktop)]
    {
        let _ = opener::open(url);
    }
    #[cfg(mobile)]
    {
        let _ = url;
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_external]);

    // ── Desktop-only: tray, single-instance, close-to-tray. None of these
    //    apply on mobile, where the OS owns the window lifecycle. ──────────
    #[cfg(desktop)]
    let builder = builder
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            show_main(app);
        }))
        .setup(|app| {
            let open_item =
                MenuItem::with_id(app, "open", "Open IHL Messenger", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&open_item, &quit_item])?;

            TrayIconBuilder::with_id("main-tray")
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("IHL Messenger")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "open" => show_main(app),
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        ..
                    } = event
                    {
                        show_main(tray.app_handle());
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        });

    builder
        .run(tauri::generate_context!())
        .expect("error while running IHL Messenger");
}
