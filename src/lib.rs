/*!
Crate for determining the file format of a [given file](`FileFormat::from_file`) or stream.

It provides a variety of functions for identifying a wide range of file formats, including
[ZIP](`FileFormat::Zip`), [Compound File Binary (CFB)](`FileFormat::CompoundFileBinary`),
[Extensible Markup Language (XML)](`FileFormat::ExtensibleMarkupLanguage`) and [more](`FileFormat`).

It checks the signature of the file to determine its format and intelligently employs specific
readers when available for accurate identification. If the signature is not recognized, the crate
falls back to the [default](`FileFormat::default`) file format, which is
[Arbitrary Binary Data (BIN)](`FileFormat::ArbitraryBinaryData`).

# Highlights

- **Extensive coverage** — supports 500+ file formats across 20+ categories.
- **Multi-layered detection** — combines magic-byte signature matching, format-specific deep
  readers, and text heuristics for accurate identification.
- **Detailed detection API** — [`from_reader_detailed`](`FileFormat::from_reader_detailed`) and
  friends return a [`Detection`] value carrying [`Confidence`] levels and the
  [`DetectionMethod`] used.
- **Safe and lightweight** — `#![forbid(unsafe_code)]` with zero dependencies by default.

# Examples

Determines from a file:

```no_run
use file_format::{FileFormat, Kind};

let fmt = FileFormat::from_file("fixtures/document/sample.pdf")?;
assert_eq!(fmt, FileFormat::PortableDocumentFormat);
assert_eq!(fmt.name(), "Portable Document Format");
assert_eq!(fmt.short_name(), Some("PDF"));
assert_eq!(fmt.media_type(), "application/pdf");
assert_eq!(fmt.extension(), "pdf");
assert_eq!(fmt.kind(), Kind::Document);
# Ok::<(), std::io::Error>(())
```

Determines from bytes:

```
use file_format::{FileFormat, Kind};

let fmt = FileFormat::from_bytes(&[0xFF, 0xD8, 0xFF]);
assert_eq!(fmt, FileFormat::JointPhotographicExpertsGroup);
assert_eq!(fmt.name(), "Joint Photographic Experts Group");
assert_eq!(fmt.short_name(), Some("JPEG"));
assert_eq!(fmt.media_type(), "image/jpeg");
assert_eq!(fmt.extension(), "jpg");
assert_eq!(fmt.kind(), Kind::Image);
```

Determines from a file with detailed detection information:

```no_run
use file_format::{Confidence, DetectionMethod, FileFormat};

let detection = FileFormat::from_file_detailed("fixtures/document/sample.pdf")?;
assert_eq!(detection.format(), FileFormat::PortableDocumentFormat);
assert_eq!(detection.confidence(), Confidence::High);
assert_eq!(detection.method(), DetectionMethod::Signature);
# Ok::<(), std::io::Error>(())
```

Retrieves file formats by extension, media type, kind, or name:

```
use file_format::{FileFormat, Kind};

let formats = FileFormat::from_extension("jpg");
assert!(formats.contains(&FileFormat::JointPhotographicExpertsGroup));

let formats = FileFormat::from_media_type("image/jpeg");
assert!(formats.contains(&FileFormat::JointPhotographicExpertsGroup));

let formats = FileFormat::from_kind(Kind::Image);
assert!(formats.contains(&FileFormat::JointPhotographicExpertsGroup));

let format = FileFormat::from_name("Joint Photographic Experts Group");
assert_eq!(format, Some(FileFormat::JointPhotographicExpertsGroup));
```

Parses from variant name:

```
use file_format::FileFormat;

let fmt: FileFormat = "Zip".parse().unwrap();
assert_eq!(fmt, FileFormat::Zip);
```

# Crate features

All features below are disabled by default.

## Core features

- `serde` - Enables `Serialize`/`Deserialize` on [`FileFormat`], [`Kind`], [`Confidence`],
  [`DetectionMethod`], and [`Detection`].

## Reader features

These features enable the detection of file formats that require a specific reader for
identification.

- `reader` - Enables all reader features.
- `reader-asf` - Enables [Advanced Systems Format (ASF)](`FileFormat::AdvancedSystemsFormat`) based
  file formats detection.
  * [Microsoft Digital Video Recording (DVR-MS)](`FileFormat::MicrosoftDigitalVideoRecording`)
  * [Windows Media Audio (WMA)](`FileFormat::WindowsMediaAudio`)
  * [Windows Media Video (WMV)](`FileFormat::WindowsMediaVideo`)
- `reader-cfb` - Enables [Compound File Binary (CFB)](`FileFormat::CompoundFileBinary`) based file
  formats detection.
  * [3D Studio Max (MAX)](`FileFormat::ThreeDimensionalStudioMax`)
  * [Autodesk Inventor Assembly (IAM)](`FileFormat::AutodeskInventorAssembly`)
  * [Autodesk Inventor Drawing (IDW)](`FileFormat::AutodeskInventorDrawing`)
  * [Autodesk Inventor Part (IPT)](`FileFormat::AutodeskInventorPart`)
  * [Autodesk Inventor Presentation (IPN)](`FileFormat::AutodeskInventorPresentation`)
  * [Corel Presentations 7 (SHW)](`FileFormat::CorelPresentations7`)
  * [Flash Project (FLA)](`FileFormat::FlashProject`)
  * [Microsoft Excel Spreadsheet (XLS)](`FileFormat::MicrosoftExcelSpreadsheet`)
  * [Microsoft PowerPoint Presentation (PPT)](`FileFormat::MicrosoftPowerpointPresentation`)
  * [Microsoft Project Plan (MPP)](`FileFormat::MicrosoftProjectPlan`)
  * [Microsoft Publisher Document (PUB)](`FileFormat::MicrosoftPublisherDocument`)
  * [Microsoft Software Installer (MSI)](`FileFormat::MicrosoftSoftwareInstaller`)
  * [Microsoft Visio Drawing (VSD)](`FileFormat::MicrosoftVisioDrawing`)
  * [Microsoft Word Document (DOC)](`FileFormat::MicrosoftWordDocument`)
  * [Microsoft Works 6 Spreadsheet (XLR)](`FileFormat::MicrosoftWorks6Spreadsheet`)
  * [Microsoft Works Database (WDB)](`FileFormat::MicrosoftWorksDatabase`)
  * [Microsoft Works Word Processor (WPS)](`FileFormat::MicrosoftWorksWordProcessor`)
  * [SolidWorks Assembly (SLDASM)](`FileFormat::SolidworksAssembly`)
  * [SolidWorks Drawing (SLDDRW)](`FileFormat::SolidworksDrawing`)
  * [SolidWorks Part (SLDPRT)](`FileFormat::SolidworksPart`)
  * [StarCalc (SDC)](`FileFormat::Starcalc`)
  * [StarChart (SDS)](`FileFormat::Starchart`)
  * [StarDraw (SDA)](`FileFormat::Stardraw`)
  * [StarImpress (SDD)](`FileFormat::Starimpress`)
  * [StarMath (SMF)](`FileFormat::Starmath`)
  * [StarWriter (SDW)](`FileFormat::Starwriter`)
  * [WordPerfect Document (WPD)](`FileFormat::WordperfectDocument`)
  * [WordPerfect Graphics (WPG)](`FileFormat::WordperfectGraphics`)
- `reader-ebml` - Enables [Extensible Binary Meta Language (EBML)](`FileFormat::ExtensibleBinaryMetaLanguage`)
  based file formats detection.
  * [Matroska 3D Video (MK3D)](`FileFormat::Matroska3dVideo`)
  * [Matroska Audio (MKA)](`FileFormat::MatroskaAudio`)
  * [Matroska Subtitles (MKS)](`FileFormat::MatroskaSubtitles`)
  * [Matroska Video (MKV)](`FileFormat::MatroskaVideo`)
  * [WebM](`FileFormat::Webm`)
- `reader-exe` - Enables [MS-DOS Executable (EXE)](`FileFormat::MsDosExecutable`) based file formats
  detection.
  * [Dynamic Link Library (DLL)](`FileFormat::DynamicLinkLibrary`)
  * [Linear Executable (LE)](`FileFormat::LinearExecutable`)
  * [New Executable (NE)](`FileFormat::NewExecutable`)
  * [Portable Executable (PE)](`FileFormat::PortableExecutable`)
- `reader-id3v2` - Enables [ID3v2 (ID3)](`FileFormat::Id3v2`) based file formats detection.
  * [Free Lossless Audio Codec (FLAC)](`FileFormat::FreeLosslessAudioCodec`)
  * [MPEG-1/2 Audio Layer 3 (MP3)](`FileFormat::Mpeg12AudioLayer3`)
- `reader-mp4` - Enables [MPEG-4 Part 14 (MP4)](`FileFormat::Mpeg4Part14`) based file formats
  detection.
  * [MPEG-4 Part 14 Audio (MP4)](`FileFormat::Mpeg4Part14Audio`)
  * [MPEG-4 Part 14 Subtitles (MP4)](`FileFormat::Mpeg4Part14Subtitles`)
  * [MPEG-4 Part 14 Video (MP4)](`FileFormat::Mpeg4Part14Video`)
- `reader-pdf` - Enables [Portable Document Format (PDF)](`FileFormat::PortableDocumentFormat`)
  based file formats detection.
  * [Adobe Illustrator Artwork (AI)](`FileFormat::AdobeIllustratorArtwork`)
- `reader-rm` - Enables [RealMedia (RM)](`FileFormat::Realmedia`) based file formats detection.
  * [RealAudio (RA)](`FileFormat::Realaudio`)
  * [RealVideo (RV)](`FileFormat::Realvideo`)
- `reader-sqlite3` - Enables [SQLite 3](`FileFormat::Sqlite3`) based file formats detection.
  * [Sketch](`FileFormat::Sketch`)
- `reader-txt` - Enables [Plain Text (TXT)](`FileFormat::PlainText`) detection when the file format
  is not recognized by its signature. Please note that this feature only detects files containing
  ASCII/UTF-8-encoded text.
- `reader-xml` - Enables [Extensible Markup Language (XML)](`FileFormat::ExtensibleMarkupLanguage`)
  based file formats detection. Please note that these file formats may be detected without the
  feature in certain cases.
  * [AbiWord (ABW)](`FileFormat::Abiword`)
  * [AbiWord Template (AWT)](`FileFormat::AbiwordTemplate`)
  * [Additive Manufacturing Format (AMF)](`FileFormat::AdditiveManufacturingFormat`)
  * [Advanced Stream Redirector (ASX)](`FileFormat::AdvancedStreamRedirector`)
  * [Atom](`FileFormat::Atom`)
  * [Collaborative Design Activity (COLLADA)](`FileFormat::CollaborativeDesignActivity`)
  * [Extensible 3D (X3D)](`FileFormat::Extensible3d`)
  * [Extensible Stylesheet Language Transformations (XSLT)](`FileFormat::ExtensibleStylesheetLanguageTransformations`)
  * [FictionBook (FB2)](`FileFormat::Fictionbook`)
  * [GPS Exchange Format (GPX)](`FileFormat::GpsExchangeFormat`)
  * [Geography Markup Language (GML)](`FileFormat::GeographyMarkupLanguage`)
  * [Keyhole Markup Language (KML)](`FileFormat::KeyholeMarkupLanguage`)
  * [MPEG-DASH MPD (MPD)](`FileFormat::MpegDashMpd`)
  * [Mathematical Markup Language (MathML)](`FileFormat::MathematicalMarkupLanguage`)
  * [MusicXML](`FileFormat::Musicxml`)
  * [Really Simple Syndication (RSS)](`FileFormat::ReallySimpleSyndication`)
  * [Scalable Vector Graphics (SVG)](`FileFormat::ScalableVectorGraphics`)
  * [Simple Object Access Protocol (SOAP)](`FileFormat::SimpleObjectAccessProtocol`)
  * [Tiled Map XML (TMX)](`FileFormat::TiledMapXml`)
  * [Tiled Tileset XML (TSX)](`FileFormat::TiledTilesetXml`)
  * [Timed Text Markup Language (TTML)](`FileFormat::TimedTextMarkupLanguage`)
  * [Training Center XML (TCX)](`FileFormat::TrainingCenterXml`)
  * [Uniform Office Format Presentation (UOP)](`FileFormat::UniformOfficeFormatPresentation`)
  * [Uniform Office Format Spreadsheet (UOS)](`FileFormat::UniformOfficeFormatSpreadsheet`)
  * [Uniform Office Format Text (UOT)](`FileFormat::UniformOfficeFormatText`)
  * [Universal Subtitle Format (USF)](`FileFormat::UniversalSubtitleFormat`)
  * [XML Localization Interchange File Format (XLIFF)](`FileFormat::XmlLocalizationInterchangeFileFormat`)
  * [XML Shareable Playlist Format (XSPF)](`FileFormat::XmlShareablePlaylistFormat`)
  * [draw.io (DRAWIO)](`FileFormat::Drawio`)
- `reader-zip` - Enables [ZIP](`FileFormat::Zip`)-based file formats detection.
  * [3D Manufacturing Format (3MF)](`FileFormat::ThreeDimensionalManufacturingFormat`)
  * [Adobe Integrated Runtime (AIR)](`FileFormat::AdobeIntegratedRuntime`)
  * [Android App Bundle (AAB)](`FileFormat::AndroidAppBundle`)
  * [Android Package (APK)](`FileFormat::AndroidPackage`)
  * [Autodesk 123D (123DX)](`FileFormat::Autodesk123d`)
  * [Circuit Diagram Document (CDDX)](`FileFormat::CircuitDiagramDocument`)
  * [Design Web Format XPS (DWFX)](`FileFormat::DesignWebFormatXps`)
  * [Electronic Publication (EPUB)](`FileFormat::ElectronicPublication`)
  * [Enterprise Application Archive (EAR)](`FileFormat::EnterpriseApplicationArchive`)
  * [FictionBook ZIP (FBZ)](`FileFormat::FictionbookZip`)
  * [Figma Design (FIG)](`FileFormat::FigmaDesign`)
  * [Flash CS5 Project (FLA)](`FileFormat::FlashCs5Project`)
  * [Fusion 360 (F3D)](`FileFormat::Fusion360`)
  * [InDesign Markup Language (IDML)](`FileFormat::IndesignMarkupLanguage`)
  * [Java Archive (JAR)](`FileFormat::JavaArchive`)
  * [Keyhole Markup Language ZIP (KMZ)](`FileFormat::KeyholeMarkupLanguageZip`)
  * [Microsoft Visual Studio Extension (VSIX)](`FileFormat::MicrosoftVisualStudioExtension`)
  * [MusicXML ZIP (MXL)](`FileFormat::MusicxmlZip`)
  * [Office Open XML Document (DOCX)](`FileFormat::OfficeOpenXmlDocument`)
  * [Office Open XML Drawing (VSDX)](`FileFormat::OfficeOpenXmlDrawing`)
  * [Office Open XML Presentation (PPTX)](`FileFormat::OfficeOpenXmlPresentation`)
  * [Office Open XML Spreadsheet (XLSX)](`FileFormat::OfficeOpenXmlSpreadsheet`)
  * [OpenDocument Database (ODB)](`FileFormat::OpendocumentDatabase`)
  * [OpenDocument Formula (ODF)](`FileFormat::OpendocumentFormula`)
  * [OpenDocument Formula Template (OTF)](`FileFormat::OpendocumentFormulaTemplate`)
  * [OpenDocument Graphics (ODG)](`FileFormat::OpendocumentGraphics`)
  * [OpenDocument Graphics Template (OTG)](`FileFormat::OpendocumentGraphicsTemplate`)
  * [OpenDocument Presentation (ODP)](`FileFormat::OpendocumentPresentation`)
  * [OpenDocument Presentation Template (OTP)](`FileFormat::OpendocumentPresentationTemplate`)
  * [OpenDocument Spreadsheet (ODS)](`FileFormat::OpendocumentSpreadsheet`)
  * [OpenDocument Spreadsheet Template (OTS)](`FileFormat::OpendocumentSpreadsheetTemplate`)
  * [OpenDocument Text (ODT)](`FileFormat::OpendocumentText`)
  * [OpenDocument Text Master (ODM)](`FileFormat::OpendocumentTextMaster`)
  * [OpenDocument Text Master Template (OTM)](`FileFormat::OpendocumentTextMasterTemplate`)
  * [OpenDocument Text Template (OTT)](`FileFormat::OpendocumentTextTemplate`)
  * [OpenRaster (ORA)](`FileFormat::Openraster`)
  * [OpenXPS (OXPS)](`FileFormat::Openxps`)
  * [Sketch 43](`FileFormat::Sketch43`)
  * [SpaceClaim Document (SCDOC)](`FileFormat::SpaceclaimDocument`)
  * [Sun XML Calc (SXC)](`FileFormat::SunXmlCalc`)
  * [Sun XML Calc Template (STC)](`FileFormat::SunXmlCalcTemplate`)
  * [Sun XML Draw (SXD)](`FileFormat::SunXmlDraw`)
  * [Sun XML Draw Template (STD)](`FileFormat::SunXmlDrawTemplate`)
  * [Sun XML Impress (SXI)](`FileFormat::SunXmlImpress`)
  * [Sun XML Impress Template (STI)](`FileFormat::SunXmlImpressTemplate`)
  * [Sun XML Math (SXM)](`FileFormat::SunXmlMath`)
  * [Sun XML Writer (SXW)](`FileFormat::SunXmlWriter`)
  * [Sun XML Writer Global (SGW)](`FileFormat::SunXmlWriterGlobal`)
  * [Sun XML Writer Template (STW)](`FileFormat::SunXmlWriterTemplate`)
  * [Universal Scene Description ZIP (USDZ)](`FileFormat::UniversalSceneDescriptionZip`)
  * [Web Application Archive (WAR)](`FileFormat::WebApplicationArchive`)
  * [Windows App Bundle (APPXBUNDLE)](`FileFormat::WindowsAppBundle`)
  * [Windows App Package (APPX)](`FileFormat::WindowsAppPackage`)
  * [XAP](`FileFormat::Xap`)
  * [XPInstall (XPI)](`FileFormat::Xpinstall`)
  * [iOS App Store Package (IPA)](`FileFormat::IosAppStorePackage`)
*/

#![deny(missing_docs)]
#![forbid(unsafe_code)]

#[macro_use]
mod macros;

mod formats;
mod readers;
mod signatures;

use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{Cursor, Read, Result, Seek},
    path::Path,
};

pub use formats::FileFormat;

/// Error returned when parsing a [`FileFormat`] from a string fails.
///
/// See [`FileFormat::from_str`](std::str::FromStr::from_str).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseFileFormatError;

impl Display for ParseFileFormatError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("unknown file format variant")
    }
}

impl std::error::Error for ParseFileFormatError {}

/// How confident the detection result is.
///
/// The confidence level reflects how much evidence was available to identify the file format.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Confidence {
    /// The format could not be positively identified. The result is the default fallback
    /// ([`ArbitraryBinaryData`](FileFormat::ArbitraryBinaryData)). This typically means no
    /// signature matched and the content could not be classified — the file may genuinely be
    /// arbitrary binary data.
    Low,
    /// The format was identified heuristically — for example, the `reader-txt` feature detected
    /// that the content is valid ASCII/UTF-8 text. The result is likely correct but not
    /// guaranteed because heuristics can misidentify files.
    Medium,
    /// The format was identified by a magic-byte signature or further confirmed by a
    /// format-specific deep reader. The result is very likely correct.
    High,
}

/// The method that was used to identify the file format.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DetectionMethod {
    /// The file was identified by matching a magic-byte signature only.
    Signature,
    /// The file was first matched by signature and then refined by a format-specific deep reader
    /// (e.g. the ZIP reader that inspects archive entries).
    Reader,
    /// No signature matched and the file was identified as plain text by the generic text reader
    /// (requires the `reader-txt` feature).
    Text,
    /// No detection method succeeded. The result is the default fallback format.
    Default,
}

/// Result of a detailed file format detection.
///
/// Returned by [`FileFormat::from_reader_detailed`], [`FileFormat::from_file_detailed`], and
/// [`FileFormat::from_bytes_detailed`].
///
/// In addition to the detected [`FileFormat`] it carries:
///
/// - [`confidence`](Detection::confidence) — how certain the identification is.
/// - [`method`](Detection::method) — which detection strategy produced the result.
/// - [`reader_error`](Detection::reader_error) — any I/O error that occurred in a
///   format-specific reader (the detection then fell back to a less precise strategy).
///
/// # Examples
///
/// ```no_run
/// use file_format::{Confidence, DetectionMethod, FileFormat};
///
/// let det = FileFormat::from_file_detailed("fixtures/document/sample.pdf")?;
/// assert_eq!(det.format(), FileFormat::PortableDocumentFormat);
/// assert_eq!(det.confidence(), Confidence::High);
/// assert!(matches!(det.method(), DetectionMethod::Signature | DetectionMethod::Reader));
/// assert!(det.reader_error().is_none());
/// # Ok::<(), std::io::Error>(())
/// ```
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Detection {
    format: FileFormat,
    confidence: Confidence,
    method: DetectionMethod,
    #[cfg_attr(feature = "serde", serde(skip))]
    reader_error: Option<std::io::Error>,
}

impl Detection {
    /// Returns the detected file format.
    #[inline]
    pub fn format(&self) -> FileFormat {
        self.format
    }

    /// Returns the confidence level of the detection.
    #[inline]
    pub fn confidence(&self) -> Confidence {
        self.confidence
    }

    /// Returns the method used to detect the file format.
    #[inline]
    pub fn method(&self) -> DetectionMethod {
        self.method
    }

    /// Returns the error that occurred in the format-specific reader, if any.
    ///
    /// A `Some` value means the signature matched a format but the deep reader failed, so the
    /// result was obtained via a fallback strategy and may be less precise.
    #[inline]
    pub fn reader_error(&self) -> Option<&std::io::Error> {
        self.reader_error.as_ref()
    }
}

impl FileFormat {
    /// Determines the file format from bytes.
    ///
    /// # Examples
    ///
    /// Detects from the first bytes of a
    /// [Portable Network Graphics (PNG)](`FileFormat::PortableNetworkGraphics`) file:
    ///
    /// ```
    /// use file_format::FileFormat;
    ///
    /// let fmt = FileFormat::from_bytes(b"\x89\x50\x4E\x47\x0D\x0A\x1A\x0A");
    /// assert_eq!(fmt, FileFormat::PortableNetworkGraphics);
    ///```
    ///
    /// Detects from a zeroed buffer:
    ///
    /// ```
    /// use file_format::FileFormat;
    ///
    /// let fmt = FileFormat::from_bytes(&[0; 1000]);
    /// assert_eq!(fmt, FileFormat::ArbitraryBinaryData);
    ///```
    #[inline]
    pub fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> Self {
        Self::from(bytes.as_ref())
    }

    /// Determines the file format from a file.
    ///
    /// This method opens the file at the given path and reads its entire contents into memory
    /// for detection.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use file_format::FileFormat;
    ///
    /// let fmt = FileFormat::from_file("fixtures/video/sample.avi")?;
    /// assert_eq!(fmt, FileFormat::AudioVideoInterleave);
    /// # Ok::<(), std::io::Error>(())
    ///```
    #[inline]
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        Self::from_reader(File::open(path)?)
    }

    /// Determines the file format from a reader.
    ///
    /// The reader must implement both [`Read`] and [`Seek`] so the detection logic can read
    /// the initial bytes and then rewind for format-specific deep readers.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use file_format::FileFormat;
    ///
    /// let fmt = FileFormat::from_reader(std::io::empty())?;
    /// assert_eq!(fmt, FileFormat::Empty);
    /// # Ok::<(), std::io::Error>(())
    ///```
    pub fn from_reader<R: Read + Seek>(reader: R) -> Result<Self> {
        Self::from_reader_detailed(reader).map(|d| d.format)
    }

    /// Determines the file format from bytes, returning a [`Detection`] with confidence, method,
    /// and any reader error.
    ///
    /// # Examples
    ///
    /// ```
    /// use file_format::{Confidence, DetectionMethod, FileFormat};
    ///
    /// let det = FileFormat::from_bytes_detailed(b"\x89\x50\x4E\x47\x0D\x0A\x1A\x0A");
    /// assert_eq!(det.format(), FileFormat::PortableNetworkGraphics);
    /// assert_eq!(det.confidence(), Confidence::High);
    /// assert_eq!(det.method(), DetectionMethod::Signature);
    ///```
    #[inline]
    pub fn from_bytes_detailed<B: AsRef<[u8]>>(bytes: B) -> Detection {
        Self::from_reader_detailed(Cursor::new(bytes.as_ref())).unwrap_or_else(|_| Detection {
            format: Self::default(),
            confidence: Confidence::Low,
            method: DetectionMethod::Default,
            reader_error: None,
        })
    }

    /// Determines the file format from a file, returning a [`Detection`] with confidence, method,
    /// and any reader error.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use file_format::{Confidence, FileFormat};
    ///
    /// let det = FileFormat::from_file_detailed("fixtures/video/sample.avi")?;
    /// assert_eq!(det.format(), FileFormat::AudioVideoInterleave);
    /// assert_eq!(det.confidence(), Confidence::High);
    /// # Ok::<(), std::io::Error>(())
    ///```
    #[inline]
    pub fn from_file_detailed<P: AsRef<Path>>(path: P) -> Result<Detection> {
        Self::from_reader_detailed(File::open(path)?)
    }

    /// Determines the file format from a reader, returning a [`Detection`] with confidence, method,
    /// and any reader error.
    ///
    /// When a signature matches but the format-specific reader fails, the error is captured in
    /// [`Detection::reader_error`] and the format falls back to a generic reader.
    ///
    /// # Examples
    ///
    /// ```
    /// use file_format::{Confidence, DetectionMethod, FileFormat};
    ///
    /// let det = FileFormat::from_reader_detailed(std::io::empty())?;
    /// assert_eq!(det.format(), FileFormat::Empty);
    /// assert_eq!(det.confidence(), Confidence::High);
    /// assert_eq!(det.method(), DetectionMethod::Signature);
    /// # Ok::<(), std::io::Error>(())
    ///```
    pub fn from_reader_detailed<R: Read + Seek>(mut reader: R) -> Result<Detection> {
        let mut buf = vec![0u8; Self::SIGNATURE_MAX_LEN];
        let mut nread = 0;
        while nread < buf.len() {
            match reader.read(&mut buf[nread..])? {
                0 => break,
                n => nread += n,
            }
        }

        Ok(if nread == 0 {
            Detection {
                format: Self::Empty,
                confidence: Confidence::High,
                method: DetectionMethod::Signature,
                reader_error: None,
            }
        } else if let Some(sig_fmt) = Self::from_signature(&buf[..nread]) {
            match Self::from_fmt_reader(sig_fmt, &mut reader) {
                Ok(format) => {
                    let refined = format != sig_fmt;
                    Detection {
                        format,
                        confidence: Confidence::High,
                        method: if refined {
                            DetectionMethod::Reader
                        } else {
                            DetectionMethod::Signature
                        },
                        reader_error: None,
                    }
                }
                Err(err) => {
                    let (format, method, confidence) = Self::from_generic_reader(&mut reader);
                    Detection {
                        format,
                        confidence,
                        method,
                        reader_error: Some(err),
                    }
                }
            }
        } else {
            let (format, method, confidence) = Self::from_generic_reader(&mut reader);
            Detection {
                format,
                confidence,
                method,
                reader_error: None,
            }
        })
    }
}

impl FileFormat {
    /// Returns `true` if the file format is an [`Archive`](Kind::Archive).
    #[inline]
    pub const fn is_archive(&self) -> bool {
        matches!(self.kind(), Kind::Archive)
    }

    /// Returns `true` if the file format is [`Audio`](Kind::Audio).
    #[inline]
    pub const fn is_audio(&self) -> bool {
        matches!(self.kind(), Kind::Audio)
    }

    /// Returns `true` if the file format is [`Compressed`](Kind::Compressed).
    #[inline]
    pub const fn is_compressed(&self) -> bool {
        matches!(self.kind(), Kind::Compressed)
    }

    /// Returns `true` if the file format is a [`Database`](Kind::Database).
    #[inline]
    pub const fn is_database(&self) -> bool {
        matches!(self.kind(), Kind::Database)
    }

    /// Returns `true` if the file format is a [`Diagram`](Kind::Diagram).
    #[inline]
    pub const fn is_diagram(&self) -> bool {
        matches!(self.kind(), Kind::Diagram)
    }

    /// Returns `true` if the file format is a [`Disk`](Kind::Disk).
    #[inline]
    pub const fn is_disk(&self) -> bool {
        matches!(self.kind(), Kind::Disk)
    }

    /// Returns `true` if the file format is a [`Document`](Kind::Document).
    #[inline]
    pub const fn is_document(&self) -> bool {
        matches!(self.kind(), Kind::Document)
    }

    /// Returns `true` if the file format is an [`Ebook`](Kind::Ebook).
    #[inline]
    pub const fn is_ebook(&self) -> bool {
        matches!(self.kind(), Kind::Ebook)
    }

    /// Returns `true` if the file format is an [`Executable`](Kind::Executable).
    #[inline]
    pub const fn is_executable(&self) -> bool {
        matches!(self.kind(), Kind::Executable)
    }

    /// Returns `true` if the file format is a [`Font`](Kind::Font).
    #[inline]
    pub const fn is_font(&self) -> bool {
        matches!(self.kind(), Kind::Font)
    }

    /// Returns `true` if the file format is a [`Formula`](Kind::Formula).
    #[inline]
    pub const fn is_formula(&self) -> bool {
        matches!(self.kind(), Kind::Formula)
    }

    /// Returns `true` if the file format is [`Geospatial`](Kind::Geospatial).
    #[inline]
    pub const fn is_geospatial(&self) -> bool {
        matches!(self.kind(), Kind::Geospatial)
    }

    /// Returns `true` if the file format is an [`Image`](Kind::Image).
    #[inline]
    pub const fn is_image(&self) -> bool {
        matches!(self.kind(), Kind::Image)
    }

    /// Returns `true` if the file format is [`Metadata`](Kind::Metadata).
    #[inline]
    pub const fn is_metadata(&self) -> bool {
        matches!(self.kind(), Kind::Metadata)
    }

    /// Returns `true` if the file format is a [`Model`](Kind::Model).
    #[inline]
    pub const fn is_model(&self) -> bool {
        matches!(self.kind(), Kind::Model)
    }

    /// Returns `true` if the file format is [`Other`](Kind::Other).
    #[inline]
    pub const fn is_other(&self) -> bool {
        matches!(self.kind(), Kind::Other)
    }

    /// Returns `true` if the file format is a [`Package`](Kind::Package).
    #[inline]
    pub const fn is_package(&self) -> bool {
        matches!(self.kind(), Kind::Package)
    }

    /// Returns `true` if the file format is a [`Playlist`](Kind::Playlist).
    #[inline]
    pub const fn is_playlist(&self) -> bool {
        matches!(self.kind(), Kind::Playlist)
    }

    /// Returns `true` if the file format is a [`Presentation`](Kind::Presentation).
    #[inline]
    pub const fn is_presentation(&self) -> bool {
        matches!(self.kind(), Kind::Presentation)
    }

    /// Returns `true` if the file format is a [`Rom`](Kind::Rom).
    #[inline]
    pub const fn is_rom(&self) -> bool {
        matches!(self.kind(), Kind::Rom)
    }

    /// Returns `true` if the file format is a [`Spreadsheet`](Kind::Spreadsheet).
    #[inline]
    pub const fn is_spreadsheet(&self) -> bool {
        matches!(self.kind(), Kind::Spreadsheet)
    }

    /// Returns `true` if the file format is a [`Subtitle`](Kind::Subtitle).
    #[inline]
    pub const fn is_subtitle(&self) -> bool {
        matches!(self.kind(), Kind::Subtitle)
    }

    /// Returns `true` if the file format is [`Video`](Kind::Video).
    #[inline]
    pub const fn is_video(&self) -> bool {
        matches!(self.kind(), Kind::Video)
    }
}

impl Default for FileFormat {
    /// Returns the default file format which is
    /// [Arbitrary Binary Data (BIN)](`FileFormat::ArbitraryBinaryData`).
    #[inline]
    fn default() -> Self {
        Self::ArbitraryBinaryData
    }
}

impl Display for FileFormat {
    #[inline]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.name())
    }
}

impl From<&[u8]> for FileFormat {
    #[inline]
    fn from(value: &[u8]) -> Self {
        Self::from_reader(Cursor::new(value)).unwrap_or_default()
    }
}

/// The broad category a [`FileFormat`] belongs to.
///
/// Every file format is classified into exactly one kind, such as [`Image`](Kind::Image),
/// [`Audio`](Kind::Audio), [`Document`](Kind::Document), and so on. Use
/// [`FileFormat::kind`] to retrieve it, or [`FileFormat::from_kind`] to list all formats
/// in a given category.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum Kind {
    /// Files and directories stored in a single, possibly compressed, archive.
    Archive,
    /// Musics, sound effects, and spoken audio recordings.
    Audio,
    /// Compressed single files or streams.
    Compressed,
    /// Organized collections of data.
    Database,
    /// Visual information using graphics and spatial relationships.
    Diagram,
    /// Floppy disk images, optical disc images and virtual machine disks.
    Disk,
    /// Word processing and desktop publishing documents.
    Document,
    /// Electronic books.
    Ebook,
    /// Machine-executable code, virtual machine code and shared libraries.
    Executable,
    /// Typefaces used for displaying text on screen or in print.
    Font,
    /// Mathematical formulas.
    Formula,
    /// Collections of geospatial features, GPS tracks and other location-related files.
    Geospatial,
    /// Animated images, icons, cursors, raster graphics and vector graphics.
    Image,
    /// Data that provides information about other data.
    Metadata,
    /// 3D models, CAD drawings, and other types of files used for creating or displaying 3D images.
    Model,
    /// Data which do not fit in any of the other kinds.
    Other,
    /// Collections of files bundled together for software distribution.
    Package,
    /// Lists of audio or video files, organized in a specific order for sequential playback.
    Playlist,
    /// Slide shows.
    Presentation,
    /// Copies of a read-only memory chip of computers, cartridges, or other electronic devices.
    Rom,
    /// Data in tabular form.
    Spreadsheet,
    /// Subtitles and captions.
    Subtitle,
    /// Moving images, possibly with color and coordinated sound.
    Video,
}
