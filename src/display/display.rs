use std::mem;
use windows::{
    core::{PCWSTR},
    Win32::Graphics::Gdi::*,
};
use thiserror::Error;

/// 显示模式信息
#[derive(Debug, Clone)]
pub struct DisplayMode {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
    pub bits_per_pixel: u32,
}

/// 错误类型
#[derive(Error, Debug)]
pub enum DisplayError {
    #[error("无法枚举显示设置")]
    EnumFailed,
    #[error("更改显示设置失败: {0}")]
    ChangeFailed(String),
    #[error("找不到匹配的显示模式")]
    ModeNotFound,
}

/// 获取当前显示设置
pub fn get_current_display_mode() -> Result<DisplayMode, DisplayError> {
    unsafe {
        let mut devmode: DEVMODEW = mem::zeroed();
        devmode.dmSize = mem::size_of::<DEVMODEW>() as u16;

        // ENUM_CURRENT_SETTINGS = -1
        let result = EnumDisplaySettingsW(
            PCWSTR::null(),
            ENUM_CURRENT_SETTINGS,
            &mut devmode,
        );

        if result.as_bool() {
            Ok(DisplayMode {
                width: devmode.dmPelsWidth,
                height: devmode.dmPelsHeight,
                refresh_rate: devmode.dmDisplayFrequency,
                bits_per_pixel: devmode.dmBitsPerPel,
            })
        } else {
            Err(DisplayError::EnumFailed)
        }
    }
}

/// 枚举所有可用的显示模式
pub fn enumerate_display_modes() -> Result<Vec<DisplayMode>, DisplayError> {
    let mut modes = Vec::new();

    unsafe {
        let mut devmode: DEVMODEW = mem::zeroed();
        devmode.dmSize = mem::size_of::<DEVMODEW>() as u16;

        let mut index = 0u32;
        loop {
            let result = EnumDisplaySettingsW(
                PCWSTR::null(),
                ENUM_DISPLAY_SETTINGS_MODE(index),
                &mut devmode,
            );

            if !result.as_bool() {
                break;
            }

            let mode = DisplayMode {
                width: devmode.dmPelsWidth,
                height: devmode.dmPelsHeight,
                refresh_rate: devmode.dmDisplayFrequency,
                bits_per_pixel: devmode.dmBitsPerPel,
            };

            // 去重
            if !modes.iter().any(|m: &DisplayMode| {
                m.width == mode.width
                    && m.height == mode.height
                    && m.refresh_rate == mode.refresh_rate
                    && m.bits_per_pixel == mode.bits_per_pixel
            }) {
                modes.push(mode);
            }

            index += 1;
        }
    }

    if modes.is_empty() {
        Err(DisplayError::EnumFailed)
    } else {
        // 按分辨率和刷新率排序
        modes.sort_by(|a, b| {
            (b.width, b.height, b.refresh_rate)
                .cmp(&(a.width, a.height, a.refresh_rate))
        });
        Ok(modes)
    }
}

/// 更改显示设置
pub fn change_display_mode(
    width: u32,
    height: u32,
    refresh_rate: u32,
    permanent: bool,
) -> Result<(), DisplayError> {
    unsafe {
        let mut devmode: DEVMODEW = mem::zeroed();
        devmode.dmSize = mem::size_of::<DEVMODEW>() as u16;
        devmode.dmPelsWidth = width;
        devmode.dmPelsHeight = height;
        devmode.dmDisplayFrequency = refresh_rate;
        devmode.dmBitsPerPel = 32;
        devmode.dmFields = DM_PELSWIDTH | DM_PELSHEIGHT | DM_DISPLAYFREQUENCY | DM_BITSPERPEL;

        // 先测试设置是否有效
        let test_result = ChangeDisplaySettingsW(
            Some(&devmode),
            CDS_TEST,
        );

        if test_result != DISP_CHANGE_SUCCESSFUL {
            return Err(DisplayError::ChangeFailed(
                format!("测试失败: {:?}", test_result)
            ));
        }

        // 应用设置
        let flags = if permanent {
            CDS_UPDATEREGISTRY // 永久更改
        } else {
            CDS_TYPE(0) // 临时更改
        };

        let result = ChangeDisplaySettingsW(Some(&devmode), flags);

        match result {
            DISP_CHANGE_SUCCESSFUL => Ok(()),
            DISP_CHANGE_RESTART => {
                Err(DisplayError::ChangeFailed("需要重启计算机".to_string()))
            }
            DISP_CHANGE_BADMODE => {
                Err(DisplayError::ChangeFailed("不支持的显示模式".to_string()))
            }
            DISP_CHANGE_FAILED => {
                Err(DisplayError::ChangeFailed("显示驱动程序失败".to_string()))
            }
            _ => {
                Err(DisplayError::ChangeFailed(format!("未知错误: {:?}", result)))
            }
        }
    }
}

/// 恢复默认显示设置
pub fn restore_default_settings() -> Result<(), DisplayError> {
    unsafe {
        let result = ChangeDisplaySettingsW(None, CDS_TYPE(0));

        if result == DISP_CHANGE_SUCCESSFUL {
            Ok(())
        } else {
            Err(DisplayError::ChangeFailed(format!("恢复失败: {:?}", result)))
        }
    }
}

/// 针对特定显示器更改设置
pub fn change_display_mode_for_monitor(
    device_name: &str,
    width: u32,
    height: u32,
    refresh_rate: u32,
) -> Result<(), DisplayError> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    let device_name_wide: Vec<u16> = OsStr::new(device_name)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        let mut devmode: DEVMODEW = mem::zeroed();
        devmode.dmSize = mem::size_of::<DEVMODEW>() as u16;
        devmode.dmPelsWidth = width;
        devmode.dmPelsHeight = height;
        devmode.dmDisplayFrequency = refresh_rate;
        devmode.dmBitsPerPel = 32;
        devmode.dmFields = DM_PELSWIDTH | DM_PELSHEIGHT | DM_DISPLAYFREQUENCY | DM_BITSPERPEL;

        let result = ChangeDisplaySettingsExW(
            PCWSTR::from_raw(device_name_wide.as_ptr()),
            Some(&devmode),
            None,
            CDS_TYPE(0),
            None,
        );

        if result == DISP_CHANGE_SUCCESSFUL {
            Ok(())
        } else {
            Err(DisplayError::ChangeFailed(format!("错误代码: {:?}", result)))
        }
    }
}

/// 枚举所有显示器
pub fn enumerate_monitors() -> Vec<String> {
    let mut monitors = Vec::new();

    unsafe {
        let mut device: DISPLAY_DEVICEW = mem::zeroed();
        device.cb = mem::size_of::<DISPLAY_DEVICEW>() as u32;

        let mut index = 0u32;
        loop {
            let result = EnumDisplayDevicesW(
                PCWSTR::null(),
                index,
                &mut device,
                0,
            );

            if !result.as_bool() {
                break;
            }

            // 检查是否是活动的显示器
            if (device.StateFlags & DISPLAY_DEVICE_ACTIVE) != 0 {
                let name = String::from_utf16_lossy(
                    &device.DeviceName[..device.DeviceName.iter()
                        .position(|&c| c == 0)
                        .unwrap_or(device.DeviceName.len())]
                );
                monitors.push(name);
            }

            index += 1;
        }
    }

    monitors
}

/*
fn main() {
    println!("=== Windows 显示设置管理器 ===\n");

    // 1. 获取当前显示设置
    match get_current_display_mode() {
        Ok(mode) => {
            println!("📺 当前显示设置:");
            println!("   分辨率: {}x{}", mode.width, mode.height);
            println!("   刷新率: {} Hz", mode.refresh_rate);
            println!("   色深: {} bit", mode.bits_per_pixel);
        }
        Err(e) => println!("获取当前设置失败: {}", e),
    }

    println!();

    // 2. 枚举所有显示器
    println!("🖥️  可用显示器:");
    for (i, monitor) in enumerate_monitors().iter().enumerate() {
        println!("   {}. {}", i + 1, monitor);
    }

    println!();

    // 3. 列出可用的显示模式
    match enumerate_display_modes() {
        Ok(modes) => {
            println!("📋 可用的显示模式 (前10个):");
            for (i, mode) in modes.iter().take(10).enumerate() {
                println!(
                    "   {}. {}x{} @ {} Hz ({} bit)",
                    i + 1,
                    mode.width,
                    mode.height,
                    mode.refresh_rate,
                    mode.bits_per_pixel
                );
            }

            println!("\n   ... 共 {} 种模式", modes.len());
        }
        Err(e) => println!("枚举显示���式失败: {}", e),
    }

    println!();

    // 4. 示例: 更改分辨率（注释掉以避免实际更改）
    println!("💡 示例代码 (已注释，取消注释后可执行):");
    println!("   // 临时更改为 1920x1080 @ 60Hz");
    println!("   // change_display_mode(1920, 1080, 60, false)?;");
    println!();
    println!("   // 永久更改设置");
    println!("   // change_display_mode(1920, 1080, 144, true)?;");
    println!();
    println!("   // 恢复默认设置");
    println!("   // restore_default_settings()?;");

    // 取消下面的注释来实际测试更改分辨率
    
    println!("\n🔄 正在更改分辨率...");
    match change_display_mode(1920, 1080, 60, false) {
        Ok(()) => println!("✅ 分辨率更改成功!"),
        Err(e) => println!("❌ 更改失败: {}", e),
    }

    // 等待5秒后恢复
    println!("⏳ 5秒后恢复原设置...");
    std::thread::sleep(std::time::Duration::from_secs(5));

    match restore_default_settings() {
        Ok(()) => println!("✅ 已恢复默认设置"),
        Err(e) => println!("❌ 恢复失败: {}", e),
    }
    
}
*/