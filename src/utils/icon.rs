use dioxus_desktop::tao::window::Icon;
use log::{error, info};

// конвертируем иконку приложения из файла png
pub fn get_app_icon() -> Option<Icon> {
    info!("get_app_icon()");

    // ловим ошибку открытия файла
    let rgba: Option<Vec<u8>> = match image::open("public/icon.png") {
        Ok(image) => Some(image.to_rgba8().into_raw()),
        Err(err) => {
            error!("get_app_icon() >> image::open: {}", err);
            None
        }
    };

    // ловим ошибку генерации иконки
    let icon = match rgba {
        Some(rgba_vec) => match Icon::from_rgba(rgba_vec, 128, 128) {
            Ok(icon) => Some(icon),
            Err(err) => {
                error!("get_app_icon() >> from_rgba: {}", err);
                None
            }
        },
        None => None,
    };
    icon
}
