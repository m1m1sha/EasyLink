use tauri::AppHandle;

#[tauri::command]
pub async fn autostart(_app: AppHandle, enabled: bool) -> Result<(), String> {
    #[cfg(windows)]
    {
        use crate::utils::task_autostart;
        use std::{sync::mpsc, thread};

        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let _ = tx.send(task_autostart(enabled).map_err(|e| e.to_string()));
        });
        Ok(rx.recv().map_err(|e| e.to_string())??)
    }

    #[cfg(not(windows))]
    {
        use tauri_plugin_autostart::ManagerExt;
        let manager = _app.autolaunch();
        if enabled {
            manager.enable().map_err(|e| e.to_string())
        } else {
            manager.disable().map_err(|e| e.to_string())
        }
    }
}

#[tauri::command]
pub async fn autostart_status(_app: AppHandle) -> Result<bool, String> {
    #[cfg(windows)]
    {
        use crate::utils::task_autostart_status;
        task_autostart_status(None).map_err(|e| e.to_string())
    }

    #[cfg(not(windows))]
    {
        let manager = _app.autolaunch();
        manager.is_enabled().map_err(|e| e.to_string())
    }
}
