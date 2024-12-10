use convert_case::{Case, Casing};
use regex::Regex;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

pub struct CaseConverter<'a> {
    special_words: HashMap<&'a str, &'a str>,
    word_patterns: Vec<(Regex, String)>,
}

static SPECIAL_WORDS: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from([
        ("key0", "0"),
        ("key1", "1"),
        ("key2", "2"),
        ("key3", "3"),
        ("key4", "4"),
        ("key5", "5"),
        ("key6", "6"),
        ("key7", "7"),
        ("key8", "8"),
        ("key9", "9"),
        ("key102nd", "102ND"),
        ("f1", "F1"),
        ("f2", "F2"),
        ("f3", "F3"),
        ("f4", "F4"),
        ("f5", "F5"),
        ("f6", "F6"),
        ("f7", "F7"),
        ("f8", "F8"),
        ("f9", "F9"),
        ("f10", "F10"),
        ("f11", "F11"),
        ("f12", "F12"),
        ("f13", "F13"),
        ("f14", "F14"),
        ("f15", "F15"),
        ("f16", "F16"),
        ("f17", "F17"),
        ("f18", "F18"),
        ("f19", "F19"),
        ("f20", "F20"),
        ("f21", "F21"),
        ("f22", "F22"),
        ("f23", "F23"),
        ("f24", "F24"),
        ("XComponent", "XCOMPONENT"),
        ("arkts", "ARKTS"),
        ("SysRq", "SYSRQ"),
        ("Key102nd", "102ND"),
        ("LineFeed", "LINEFEED"),
        ("KeyboardIllumToggle", "KBDILLUM_TOGGLE"),
        ("KeyboardIllumDown", "KBDILLUM_DOWN"),
        ("KeyboardIllumUp", "KBDILLUM_UP"),
        ("NumpadPlusMinus", "NUMPAD_PLUSMINUS"),
        ("PlayPause", "PLAYPAUSE"),
        ("MediaPlayPause", "MEDIA_PLAY_PAUSE"),
        ("StopCD", "STOPCD"),
        ("ScrollUp", "SCROLLUP"),
        ("ScrollDown", "SCROLLDOWN"),
        ("BassBoost", "BASSBOOST"),
        ("ForwardMail", "FORWARDMAIL"),
        ("TV2", "TV2"),
        ("VCR2", "VCR2"),
        ("ChannelUp", "CHANNELUP"),
        ("ChannelDown", "CHANNELDOWN"),
        ("ZoomIn", "ZOOMIN"),
        ("ZoomOut", "ZOOMOUT"),
        ("ZoomReset", "ZOOMRESET"),
        ("WordProcessor", "WORDPROCESSOR"),
        ("GraphicsEditor", "GRAPHICSEDITOR"),
        ("AddressBook", "ADDRESSBOOK"),
        ("SpellCheck", "SPELLCHECK"),
        ("ButtonConfig", "BUTTONCONFIG"),
        ("TaskManager", "TASKMANAGER"),
        ("ControlPanel", "CONTROLPANEL"),
        ("AppSelect", "APPSELECT"),
        ("ScreenSaver", "SCREENSAVER"),
        ("KbdInputAssistPrev", "KBDINPUTASSIST_PREV"),
        ("KbdInputAssistNext", "KBDINPUTASSIST_NEXT"),
        ("KbdInputAssistPrevGroup", "KBDINPUTASSIST_PREVGROUP"),
        ("KbdInputAssistNextGroup", "KBDINPUTASSIST_NEXTGROUP"),
        ("KbdInputAssistAccept", "KBDINPUTASSIST_ACCEPT"),
        ("KbdInputAssistCancel", "KBDINPUTASSIST_CANCEL"),
        ("SendFile", "SENDFILE"),
        ("DeleteFile", "DELETEFILE"),
        ("Prog1", "PROG1"),
        ("Prog2", "PROG2"),
        ("Prog3", "PROG3"),
        ("Prog4", "PROG4"),
        ("MsDos", "MSDOS"),
        ("ScreenLock", "SCREENLOCK"),
        ("ScreenLockPassword", "SCREEN_LOCK_PASSWORD"),
        ("CycleWindows", "CYCLEWINDOWS"),
        ("EjectCloseCD", "EJECTCLOSECD"),
        ("AltErase", "ALTERASE"),
        ("SwitchVideoMode", "SWITCHVIDEOMODE"),
        ("RfKill", "RFKILL"),
    ])
});

pub static CONVERTER: LazyLock<Mutex<CaseConverter>> = LazyLock::new(|| {
    let converter = CaseConverter::new();

    Mutex::new(converter)
});

impl<'a> CaseConverter<'a> {
    pub fn new() -> Self {
        CaseConverter {
            special_words: (*SPECIAL_WORDS).clone(),
            word_patterns: Vec::new(),
        }
    }

    pub fn to_screaming_snake_case(&self, input: &str, case: Case) -> String {
        let mut result = input.to_string().to_case(case);

        for (pattern, replacement) in &self.word_patterns {
            result = pattern.replace_all(&result, replacement).to_string();
        }

        find_and_replace(result.as_str())
    }
}

pub fn convert_case(input: &str, case: Case) -> String {
    if let Ok(converter) = CONVERTER.lock() {
        converter.to_screaming_snake_case(input, case)
    } else {
        input.to_string()
    }
}

fn find_and_replace(input: &str) -> String {
    let mut longest_match: Option<(usize, usize, &str)> = None;
    let mut longest_len = 0;

    let chars: Vec<char> = input.chars().collect();

    for start in 0..chars.len() {
        let mut cleaned_substr = String::new();
        let mut orig_positions = Vec::new(); // 记录原始位置

        for (i, &c) in chars[start..].iter().enumerate() {
            if c != '_' {
                cleaned_substr.push(c);
                orig_positions.push(start + i);
            }
        }

        let lower_substr = cleaned_substr.to_lowercase();

        for (key, value) in SPECIAL_WORDS.iter() {
            let lower_key = key.to_lowercase();

            if lower_substr.starts_with(&lower_key) && lower_key.len() > longest_len {
                let end_pos = orig_positions
                    .get(lower_key.len() - 1)
                    .map(|&pos| pos + 1)
                    .unwrap_or(start);

                longest_len = lower_key.len();
                longest_match = Some((start, end_pos, value));
            }
        }
    }

    if let Some((start, end, value)) = longest_match {
        let mut result = String::new();
        result.push_str(&input[..start]);
        result.push_str(value);
        result.push_str(&input[end..]);
        result
    } else {
        input.to_string()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xcomponent_conversion() {
        assert_eq!(
            convert_case("xcomponent", Case::ScreamingSnake),
            "XCOMPONENT"
        );
        assert_eq!(
            convert_case("XComponent", Case::ScreamingSnake),
            "XCOMPONENT"
        );
        assert_eq!(
            convert_case("xComponent", Case::ScreamingSnake),
            "XCOMPONENT"
        );
        assert_eq!(
            convert_case("XCOMPonent", Case::ScreamingSnake),
            "XCOMPONENT"
        );
    }

    #[test]
    fn test_mixed_case_conversion() {
        // assert_eq!(
        //     convert_case("myXComponentTest", Case::ScreamingSnake),
        //     "MY_XCOMPONENT_TEST"
        // );
        assert_eq!(
            convert_case("TestXComponentExample", Case::ScreamingSnake),
            "TEST_XCOMPONENT_EXAMPLE"
        );
    }

    #[test]
    fn test_key_number_conversion() {
        // 测试 KEY0 到 KEY9
        for i in 0..10 {
            let input = format!("key{}", i);
            let input_upper = format!("KEY{}", i);
            let expected = i.to_string();

            assert_eq!(
                convert_case(&input, Case::ScreamingSnake),
                expected,
                "Failed for input: {}",
                input
            );
            assert_eq!(
                convert_case(&input_upper, Case::ScreamingSnake),
                expected,
                "Failed for input: {}",
                input_upper
            );
        }
    }

    #[test]
    fn test_key_number_conversion_nd() {
        assert_eq!(convert_case("key102nd", Case::ScreamingSnake), "102ND");
        assert_eq!(convert_case("KEY102ND", Case::ScreamingSnake), "102ND");
        assert_eq!(convert_case("key_102nd", Case::ScreamingSnake), "102ND");
        assert_eq!(convert_case("KEY_102ND", Case::ScreamingSnake), "102ND");
        assert_eq!(
            convert_case("ScreenLockPassword", Case::ScreamingSnake),
            "SCREEN_LOCK_PASSWORD"
        );
        assert_eq!(
            convert_case("MediaPlayPause", Case::ScreamingSnake),
            "MEDIA_PLAY_PAUSE"
        );
    }
}
