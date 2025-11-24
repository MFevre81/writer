use egui_code_editor::Syntax;

/// Determines the appropriate syntax highlighting based on file extension
pub fn get_syntax_for_file(filename: Option<&String>, highlighting_enabled: bool) -> Syntax {
    if !highlighting_enabled {
        return Syntax::default();
    }
    
    let Some(filename) = filename else {
        return Syntax::default();
    };
    
    match get_file_extension(filename) {
        Some("rs") => Syntax::rust(),
        Some("py") | Some("py3") | Some("pyw") => Syntax::python(),
        Some("lua") => Syntax::lua(),
        Some("sh") | Some("bash") | Some("zsh") => Syntax::shell(),
        Some("sql") => Syntax::sql(),
        Some("asm") | Some("s") => Syntax::asm(),
        Some("txt") | _ => Syntax::default(),
    }
}

/// Extracts the file extension from a filename
fn get_file_extension(filename: &str) -> Option<&str> {
    filename.rsplit('.').next()
}

/// Returns a human-readable name for the detected language
pub fn get_language_name(filename: Option<&String>) -> &'static str {
    let Some(filename) = filename else {
        return "Plain Text";
    };
    
    match get_file_extension(filename) {
        Some("rs") => "Rust",
        Some("py") | Some("py3") | Some("pyw") => "Python",
        Some("lua") => "Lua",
        Some("sh") | Some("bash") | Some("zsh") => "Shell",
        Some("sql") => "SQL",
        Some("asm") | Some("s") => "Assembly",
        _ => "Plain Text",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_extension() {
        let filename = "main.rs".to_string();
        assert_eq!(get_language_name(Some(&filename)), "Rust");
    }

    #[test]
    fn test_python_extensions() {
        let py = "script.py".to_string();
        let py3 = "script.py3".to_string();
        let pyw = "script.pyw".to_string();
        assert_eq!(get_language_name(Some(&py)), "Python");
        assert_eq!(get_language_name(Some(&py3)), "Python");
        assert_eq!(get_language_name(Some(&pyw)), "Python");
    }

    #[test]
    fn test_shell_extensions() {
        let sh = "script.sh".to_string();
        let bash = "script.bash".to_string();
        let zsh = "script.zsh".to_string();
        assert_eq!(get_language_name(Some(&sh)), "Shell");
        assert_eq!(get_language_name(Some(&bash)), "Shell");
        assert_eq!(get_language_name(Some(&zsh)), "Shell");
    }

    #[test]
    fn test_no_extension() {
        let readme = "README".to_string();
        assert_eq!(get_language_name(Some(&readme)), "Plain Text");
    }

    #[test]
    fn test_no_filename() {
        assert_eq!(get_language_name(None), "Plain Text");
    }

    #[test]
    fn test_unknown_extension() {
        let xyz = "file.xyz".to_string();
        assert_eq!(get_language_name(Some(&xyz)), "Plain Text");
    }
}
