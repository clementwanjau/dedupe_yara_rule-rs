use regex::Regex;
use std::path::Path;

pub fn collect_yar_files(path: &dyn AsRef<Path>) -> Vec<String> {
    if path.as_ref().is_dir() {
        path.as_ref()
            .read_dir()
            .unwrap()
            .flatten()
            .map(|c| collect_yar_files(&c.path()))
            .flatten()
            .collect::<Vec<_>>()
    } else {
        match path.as_ref().extension() {
            Some(k) if k.eq_ignore_ascii_case("yar") || k.eq_ignore_ascii_case("yara") => {
                vec![path.as_ref().to_str().unwrap().into()]
            }
            _ => vec![],
        }
    }
}

pub fn remove_comments(st: String) -> String {
    let comments = Regex::new(r#"(/\*([^*]|[\r\n]|(\*+([^*/]|[\r\n])))*\*+/|^//.*?$)"#).unwrap();

    let st = comments.replace_all(&st, "");
    st.into()
}
