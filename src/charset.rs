
pub mod charset {
    pub fn normalize(character_set: &str) -> String {
        let cs = character_set.to_ascii_uppercase();
        return match cs.as_str() {
            "UTF8" | "UT-8" | "UTR-8" | "UFT-8" | "UTF8-WITHOUT-BOM" | "UTF8_GENERAL_CI" => String::from("UTF-8"),

            // override Japanese
            // CP943: IBM OS/2 Japanese, superset of Cp932 and Shift-JIS
            "CP943" | "CP943C" | "SIFT_JIS" | "SHIFT-JIS" => String::from("SHIFT_JIS"),

            // override Korean
            "EUC-KR" | "MS949" | "KSC5601" | "WINDOWS-949" | "KS_C_5601-1987" | "KSC_5601" => String::from("UHC"),

            // override Thai
            //case "TIS-620", "WINDOWS-874":
            //	return "ISO-8859-11"

            // override latin-2
            "LATIN2_HUNGARIAN_CI" | "LATIN2" => String::from("LATIN-2"),

            // override cyrillic
            "WIN1251" | "WIN-1251" | "WINDOWS-1251" => String::from("CP1251"),

            // override Hebrew
            "WINDOWS-1255" => String::from("ISO-8859-8"),

            // override Turkish
            //case "WINDOWS-1254":
            //	return "ISO-8859-9"
            // override the parsing of ISO-8859-1 to behave as Windows-1252 (CP1252):
            // in ISO-8859-1, everything from 128-255 in the ASCII table are ctrl characters,
            // whilst in CP1252 they're symbols

            // override Baltic
            "WINDOWS-1257" => String::from("ISO-8859-13"),

            "ANSI" | "LATIN-1" | "ISO" | "RFC" | "MACINTOSH" | "8859-1" | "8859-15" | "ISO8859-1" | "ISO8859-15" | "ISO-8559-1" | "ISO-8859-1" | "ISO-8859-15" => String::from("CP1252"),

            _ => String::from(character_set).to_ascii_uppercase()
        };
    }

    #[cfg(test)]
    mod tests {
        use std::string::String;

        // Note this useful idiom: importing names from outer (for mod tests) scope.
        use super::*;

        #[test]
        fn test_normalize() {
            assert_eq!(normalize("dummy"), "DUMMY");
            assert_eq!(normalize("UTF8"), "UTF-8");
            assert_eq!(normalize("utf-8"), "UTF-8");
            assert_eq!(normalize("MACINTOSH"), "CP1252");
            assert_eq!(normalize("WINDOWS-1255"), "ISO-8859-8");
        }
    }
}
