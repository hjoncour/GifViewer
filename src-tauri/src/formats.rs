

pub fn all_file_formats() -> Vec<&'static str> {
    let mut formats: Vec<&str> = pic_format();
    formats.extend(anim_format());
    return formats;
}

pub fn pic_format() -> Vec<&'static str> {
    return vec![
        "jpeg", 
        "jpg", 
        "pjpeg", 
        "jfif", 
        "pjp", 
        "png", 
        "svg",
        "webp"
        ];
}

pub fn anim_format() -> Vec<&'static  str> {
    return vec![
        "apng", 
        "avif", 
        "gif"
        ];
}
