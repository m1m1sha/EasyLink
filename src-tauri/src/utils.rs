#[cfg(target_os = "windows")]
pub fn convert_to_string(s: impl AsRef<[u8]>) -> String {
    let s = s.as_ref();
    if let Ok(utf8) = String::from_utf8(s.to_vec()) {
        utf8
    } else {
        let (gbk, _encode, error) = encoding_rs::GBK.decode(s);
        if !error {
            gbk.to_string()
        } else {
            String::from_utf8_lossy(s).to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "windows")]
    fn test_convert_to_string_utf8() {
        let input = b"hello";
        let expected = "hello".to_string();
        assert_eq!(convert_to_string(input), expected);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_convert_to_string_gbk() {
        let input = vec![0xc4, 0xe3, 0xba, 0xc3]; // "你好" in GBK
        let expected = "你好".to_string();
        assert_eq!(convert_to_string(&input), expected);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_convert_to_string_invalid_utf8() {
        let input = vec![0xff, 0xfe, 0xfd];
        let expected = String::from_utf8_lossy(&input).to_string();
        assert_eq!(convert_to_string(&input), expected);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_convert_to_string_empty() {
        let input = b"";
        let expected = "".to_string();
        assert_eq!(convert_to_string(input), expected);
    }
}

#[cfg(target_os = "windows")]
pub fn task_autostart(enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
    use planif::{
        enums::TaskCreationFlags,
        schedule::TaskScheduler,
        schedule_builder::{Action, ScheduleBuilder},
        settings::{Duration, LogonType, PrincipalSettings, RunLevel},
    };
    use tauri::utils::platform::current_exe;

    use crate::constants;

    if enabled {
        let ts = TaskScheduler::new()?;
        let com = ts.get_com();
        let sb = ScheduleBuilder::new(&com)?;

        let exe = current_exe()?;
        let exe = exe.to_str().unwrap();

        let settings = PrincipalSettings {
            display_name: "".to_string(),
            group_id: None,
            id: "".to_string(),
            logon_type: LogonType::InteractiveToken,
            run_level: RunLevel::Highest,
            user_id: Some(whoami::username()),
        };

        sb.create_logon()
            .author("m1m1sha")?
            .trigger("trigger", enabled)?
            .action(Action::new(
                "auto start",
                exe,
                "",
                constants::TASK_AUTOSTART,
            ))?
            .in_folder(constants::TASK_AUTOSTART_FOLDER)?
            .principal(settings)?
            .delay(Duration {
                seconds: Some(6),
                ..Default::default()
            })?
            .build()?
            .register(
                constants::TASK_AUTOSTART_NAME,
                TaskCreationFlags::CreateOrUpdate as i32,
            )?;
    } else {
        task_autostart_status(Some(true))?;
    }

    Ok(())
}

#[cfg(target_os = "windows")]
pub fn task_autostart_status(delete: Option<bool>) -> Result<bool, Box<dyn std::error::Error>> {
    use windows::{
        core::BSTR,
        Win32::{
            Foundation::VARIANT_BOOL,
            System::{Com::*, TaskScheduler::*, Variant::VARIANT},
        },
    };

    use crate::constants;

    let delete = delete.unwrap_or(false);

    unsafe {
        let task_service: ITaskService = CoCreateInstance(&TaskScheduler, None, CLSCTX_ALL)?;
        task_service.Connect(
            &VARIANT::default(),
            &VARIANT::default(),
            &VARIANT::default(),
            &VARIANT::default(),
        )?;
        let folder_path = BSTR::from(&format!("\\{}", constants::TASK_AUTOSTART_FOLDER));
        let root = BSTR::from("\\");
        let task_name = BSTR::from(constants::TASK_AUTOSTART_NAME);
        let mut penabled = VARIANT_BOOL::from(false);
        let bool_ptr: *mut VARIANT_BOOL = &mut penabled;

        let task_folder: ITaskFolder = task_service.GetFolder(&folder_path)?;

        let task = task_folder.GetTask(&task_name)?;
        task.Definition()?
            .Triggers()?
            .get_Item(1)?
            .Enabled(bool_ptr)?;

        if delete {
            task_folder.DeleteFolder(&folder_path, 0)?;
            let root_folder: ITaskFolder = task_service.GetFolder(&root)?;
            root_folder.DeleteFolder(&folder_path, 0)?;
        }

        CoUninitialize();

        Ok(penabled.as_bool() && delete)
    }
}
