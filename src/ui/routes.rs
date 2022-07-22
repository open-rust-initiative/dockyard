use crate::ui::image::{del_image, get_image_count, get_image_list};
use crate::ui::library::{del_library, get_library_count, get_library_list};
use crate::ui::tag::{del_tag, get_tag_list};
use crate::ui::user::{user_login, user_register, user_update_info, user_update_passwd};
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(user_register);
    cfg.service(user_login);
    cfg.service(user_update_info);
    cfg.service(user_update_passwd);
    cfg.service(get_library_list);

    cfg.service(get_library_count);
    cfg.service(get_image_list);
    cfg.service(get_image_count);

    cfg.service(get_tag_list);
    cfg.service(del_library);
    cfg.service(del_image);
    cfg.service(del_tag);
}
