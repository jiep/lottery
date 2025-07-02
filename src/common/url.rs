pub fn make_check_url(url: &str, draw_id: u32) -> String {
    format!("{url}{draw_id}")
}

pub fn make_find_url(url: &str, draw_id: u32) -> String {
    url.replace("{}", draw_id.to_string().as_str())
}
