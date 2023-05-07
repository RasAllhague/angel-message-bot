use serenity::model::user::User;

pub fn get_icon_url(user: &User) -> String {
    if let Some(url) = user.avatar_url() {
        return url;
    }

    user.default_avatar_url()
}
