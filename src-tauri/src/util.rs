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


pub const VERSION: &str = git_version::git_version!(
    args = ["--abbrev=8", "--always", "--dirty=~"],
    prefix = concat!(env!("CARGO_PKG_VERSION"), "-"),
    suffix = "",
    fallback = env!("CARGO_PKG_VERSION")
);