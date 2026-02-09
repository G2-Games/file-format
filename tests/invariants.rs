use file_format::{FileFormat, Kind};
use std::collections::HashSet;

#[test]
fn all_is_not_empty() {
    assert!(!FileFormat::all().is_empty());
}

#[test]
fn all_contains_no_duplicates() {
    let all = FileFormat::all();
    let unique: HashSet<&FileFormat> = all.iter().collect();
    assert_eq!(all.len(), unique.len());
}

#[test]
fn extension_is_not_empty() {
    for fmt in FileFormat::all() {
        assert!(
            !fmt.extension().is_empty(),
            "{:?} has an empty extension",
            fmt
        );
    }
}

#[test]
fn extension_has_no_leading_dot() {
    for fmt in FileFormat::all() {
        assert!(
            !fmt.extension().starts_with('.'),
            "{:?} extension {:?} has a leading dot",
            fmt,
            fmt.extension()
        );
    }
}

#[test]
fn media_type_is_lowercase() {
    for fmt in FileFormat::all() {
        let mt = fmt.media_type();
        assert_eq!(
            mt,
            mt.to_ascii_lowercase(),
            "{:?} media_type {:?} is not lowercase",
            fmt,
            mt
        );
    }
}

#[test]
fn media_type_contains_slash() {
    for fmt in FileFormat::all() {
        let mt = fmt.media_type();
        assert!(
            mt.contains('/'),
            "{:?} media_type {:?} does not contain '/'",
            fmt,
            mt
        );
    }
}

#[test]
fn media_type_has_no_whitespace() {
    for fmt in FileFormat::all() {
        let mt = fmt.media_type();
        assert!(
            !mt.chars().any(|c| c.is_whitespace()),
            "{:?} media_type {:?} contains whitespace",
            fmt,
            mt
        );
    }
}

#[test]
fn name_is_not_empty() {
    for fmt in FileFormat::all() {
        assert!(!fmt.name().is_empty(), "{:?} has an empty name", fmt);
    }
}

#[test]
fn name_is_unique() {
    let mut seen = HashSet::new();
    for fmt in FileFormat::all() {
        assert!(
            seen.insert(fmt.name()),
            "{:?} has a duplicate name {:?}",
            fmt,
            fmt.name()
        );
    }
}

#[test]
fn from_name_roundtrip() {
    for fmt in FileFormat::all() {
        assert_eq!(
            FileFormat::from_name(fmt.name()),
            Some(*fmt),
            "{:?} name {:?} does not roundtrip via from_name",
            fmt,
            fmt.name()
        );
    }
}

#[test]
fn from_str_roundtrip() {
    for fmt in FileFormat::all() {
        let variant_name = format!("{:?}", fmt);
        let parsed: FileFormat = variant_name.parse().unwrap_or_else(|_| {
            panic!(
                "{:?} variant name {:?} does not roundtrip via FromStr",
                fmt, variant_name
            )
        });
        assert_eq!(parsed, *fmt);
    }
}

#[test]
fn from_extension_roundtrip() {
    for fmt in FileFormat::all() {
        let results = FileFormat::from_extension(fmt.extension());
        assert!(
            results.contains(fmt),
            "{:?} not found via from_extension({:?})",
            fmt,
            fmt.extension()
        );
    }
}

#[test]
fn from_extension_case_insensitive() {
    for fmt in FileFormat::all() {
        let ext = fmt.extension();
        let upper = ext.to_ascii_uppercase();
        let lower = ext.to_ascii_lowercase();
        assert_eq!(
            FileFormat::from_extension(&upper),
            FileFormat::from_extension(&lower),
            "{:?} extension {:?} is not case-insensitive",
            fmt,
            ext
        );
    }
}

#[test]
fn from_extension_strips_leading_dot() {
    for fmt in FileFormat::all() {
        let with_dot = format!(".{}", fmt.extension());
        let without_dot = FileFormat::from_extension(fmt.extension());
        assert_eq!(
            FileFormat::from_extension(&with_dot),
            without_dot,
            "{:?} from_extension does not strip leading dot for {:?}",
            fmt,
            fmt.extension()
        );
    }
}

#[test]
fn from_media_type_roundtrip() {
    for fmt in FileFormat::all() {
        let results = FileFormat::from_media_type(fmt.media_type());
        assert!(
            results.contains(fmt),
            "{:?} not found via from_media_type({:?})",
            fmt,
            fmt.media_type()
        );
    }
}

#[test]
fn from_media_type_case_insensitive() {
    for fmt in FileFormat::all() {
        let mt = fmt.media_type();
        let upper = mt.to_ascii_uppercase();
        assert_eq!(
            FileFormat::from_media_type(&upper),
            FileFormat::from_media_type(mt),
            "{:?} media_type {:?} is not case-insensitive",
            fmt,
            mt
        );
    }
}

#[test]
fn from_kind_roundtrip() {
    for fmt in FileFormat::all() {
        let results = FileFormat::from_kind(fmt.kind());
        assert!(
            results.contains(fmt),
            "{:?} not found via from_kind({:?})",
            fmt,
            fmt.kind()
        );
    }
}

#[test]
fn from_kind_covers_all_kinds() {
    let all_kinds = [
        Kind::Archive,
        Kind::Audio,
        Kind::Compressed,
        Kind::Database,
        Kind::Diagram,
        Kind::Disk,
        Kind::Document,
        Kind::Ebook,
        Kind::Executable,
        Kind::Font,
        Kind::Formula,
        Kind::Geospatial,
        Kind::Image,
        Kind::Metadata,
        Kind::Model,
        Kind::Other,
        Kind::Package,
        Kind::Playlist,
        Kind::Presentation,
        Kind::Rom,
        Kind::Spreadsheet,
        Kind::Subtitle,
        Kind::Video,
    ];
    for kind in all_kinds {
        assert!(
            !FileFormat::from_kind(kind).is_empty(),
            "from_kind({:?}) returned no formats",
            kind
        );
    }
}

#[test]
fn default_is_arbitrary_binary_data() {
    assert_eq!(FileFormat::default(), FileFormat::ArbitraryBinaryData);
}

#[test]
fn display_equals_name() {
    for fmt in FileFormat::all() {
        assert_eq!(fmt.to_string(), fmt.name());
    }
}

#[test]
fn short_name_is_not_empty_when_present() {
    for fmt in FileFormat::all() {
        if let Some(sn) = fmt.short_name() {
            assert!(!sn.is_empty(), "{:?} has an empty short_name", fmt);
        }
    }
}

#[test]
fn is_kind_helpers_consistent() {
    for fmt in FileFormat::all() {
        let kind = fmt.kind();
        assert_eq!(fmt.is_archive(), kind == Kind::Archive, "{:?}", fmt);
        assert_eq!(fmt.is_audio(), kind == Kind::Audio, "{:?}", fmt);
        assert_eq!(fmt.is_compressed(), kind == Kind::Compressed, "{:?}", fmt);
        assert_eq!(fmt.is_database(), kind == Kind::Database, "{:?}", fmt);
        assert_eq!(fmt.is_diagram(), kind == Kind::Diagram, "{:?}", fmt);
        assert_eq!(fmt.is_disk(), kind == Kind::Disk, "{:?}", fmt);
        assert_eq!(fmt.is_document(), kind == Kind::Document, "{:?}", fmt);
        assert_eq!(fmt.is_ebook(), kind == Kind::Ebook, "{:?}", fmt);
        assert_eq!(fmt.is_executable(), kind == Kind::Executable, "{:?}", fmt);
        assert_eq!(fmt.is_font(), kind == Kind::Font, "{:?}", fmt);
        assert_eq!(fmt.is_formula(), kind == Kind::Formula, "{:?}", fmt);
        assert_eq!(fmt.is_geospatial(), kind == Kind::Geospatial, "{:?}", fmt);
        assert_eq!(fmt.is_image(), kind == Kind::Image, "{:?}", fmt);
        assert_eq!(fmt.is_metadata(), kind == Kind::Metadata, "{:?}", fmt);
        assert_eq!(fmt.is_model(), kind == Kind::Model, "{:?}", fmt);
        assert_eq!(fmt.is_other(), kind == Kind::Other, "{:?}", fmt);
        assert_eq!(fmt.is_package(), kind == Kind::Package, "{:?}", fmt);
        assert_eq!(fmt.is_playlist(), kind == Kind::Playlist, "{:?}", fmt);
        assert_eq!(
            fmt.is_presentation(),
            kind == Kind::Presentation,
            "{:?}",
            fmt
        );
        assert_eq!(fmt.is_rom(), kind == Kind::Rom, "{:?}", fmt);
        assert_eq!(fmt.is_spreadsheet(), kind == Kind::Spreadsheet, "{:?}", fmt);
        assert_eq!(fmt.is_subtitle(), kind == Kind::Subtitle, "{:?}", fmt);
        assert_eq!(fmt.is_video(), kind == Kind::Video, "{:?}", fmt);
    }
}
