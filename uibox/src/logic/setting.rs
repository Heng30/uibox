use crate::config;
use crate::slint_generatedAppWindow::{AppWindow, Logic, Store};
use crate::util;
use crate::util::translator::tr;
use log::warn;
use slint::{ComponentHandle, Weak};

pub fn init(ui: &AppWindow) {
    init_setting_dialog(ui.as_weak());

    let ui_handle = ui.as_weak();
    ui.global::<Logic>().on_setting_cancel(move || {
        init_setting_dialog(ui_handle.clone());
    });

    let ui_handle = ui.as_weak();
    ui.global::<Logic>().on_clean_image_cache(move || {
        let ui = ui_handle.unwrap();

        match util::fs::remove_dir_files(&config::cache_image_dir()) {
            Err(e) => ui.global::<Logic>().invoke_show_message(
                slint::format!("{}. {}: {:?}", tr("清除缓存失败"), tr("原因"), e),
                "warning".into(),
            ),
            _ => {
                let mut setting_dialog = ui.global::<Store>().get_setting_dialog();
                setting_dialog.chat.cache_size = "0M".into();
                ui.global::<Store>().set_setting_dialog(setting_dialog);
                ui.global::<Logic>()
                    .invoke_show_message(tr("清除缓存成功").into(), "success".into());
            }
        }
    });

    let ui_handle = ui.as_weak();
    ui.global::<Logic>().on_setting_ok(move |setting_config| {
        let ui = ui_handle.unwrap();
        let mut config = config::config();

        config.ui.font_size = setting_config
            .ui
            .font_size
            .to_string()
            .parse()
            .unwrap_or(20);
        config.ui.font_family = setting_config.ui.font_family.to_string();

        config.ui.language = setting_config.ui.language.to_string();

        config.chat.api_key = setting_config.chat.api_key.to_string();
        config.chat.api_base = setting_config.chat.api_base.to_string();
        config.chat.image_size = setting_config.chat.image_size.to_string();
        config.chat.image_count = setting_config.chat.image_count.to_string();

        config.socks5.openai = setting_config.proxy.openai;
        config.socks5.url = setting_config.proxy.url.to_string();
        config.socks5.port = setting_config
            .proxy
            .port
            .to_string()
            .parse()
            .unwrap_or(1080);

        match config::save(config) {
            Err(e) => {
                ui.global::<Logic>().invoke_show_message(
                    slint::format!("{}{:?}", tr("保存失败") + "！", e),
                    "warning".into(),
                );
            }
            _ => {
                init_setting_dialog(ui.as_weak());
                ui.global::<Logic>()
                    .invoke_show_message((tr("保存成功") + "!").into(), "success".into());
            }
        }
    });
}

fn init_setting_dialog(ui: Weak<AppWindow>) {
    let ui = ui.unwrap();
    let ui_config = config::ui();
    let chat_config = config::chat();
    let socks5_config = config::socks5();

    let mut setting_dialog = ui.global::<Store>().get_setting_dialog();
    setting_dialog.ui.font_size = slint::format!("{}", ui_config.font_size);
    setting_dialog.ui.font_family = ui_config.font_family.into();
    setting_dialog.ui.language = ui_config.language.into();

    setting_dialog.chat.api_key = chat_config.api_key.into();
    setting_dialog.chat.api_base = chat_config.api_base.into();
    setting_dialog.chat.image_size = chat_config.image_size.into();
    setting_dialog.chat.image_count = chat_config.image_count.into();

    setting_dialog.proxy.openai = socks5_config.openai;
    setting_dialog.proxy.url = socks5_config.url.into();
    setting_dialog.proxy.port = slint::format!("{}", socks5_config.port);

    match util::fs::dir_size(&config::cache_image_dir()) {
        Ok(size) => {
            setting_dialog.chat.cache_size = size.into();
        }
        Err(e) => warn!("get audio cache size failed. {:?}", e),
    }

    ui.global::<Store>().set_setting_dialog(setting_dialog);
}
