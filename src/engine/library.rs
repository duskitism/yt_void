use super::*;

use DigitTypes as DT;

macro_rules! yt_string {
    ($($item:expr), + $(,)?) => {
        YtString { components: &[$(&$item), +] }
    };
}

macro_rules! pref_x {
    ($pref:expr) => {
        yt_string!($pref, DT::X(4))
    };
    ($pref:expr, $digits:expr) => {
        yt_string!($pref, DT::X($digits))
    };
}

macro_rules! pref_ymd {
    ($pref:expr) => {
        yt_string!($pref, DT::YMD(None))
    };
    ($pref:expr, $y_cap:expr) => {
        yt_string!($pref, DT::YMD(Some($y_cap)))
    };
}

// now ymd
macro_rules! pref_nymd {
    ($pref:expr) => {
        yt_string!($pref, DT::YMD_Now, FilterType::UploadDate)
    };
}

pub const LIBRARY: &[YtString] = &[
    pref_x!("IMG "),
    pref_x!("MVI "),
    pref_x!("MOV "),
    pref_x!("100 "),
    pref_x!("SAM "),
    pref_x!("DSC "),
    pref_x!("SDV "),
    pref_x!("DJI "),
    // ----
    pref_x!("DSCF"),
    pref_x!("DSCN"),
    pref_x!("PICT"),
    pref_x!("MAQ0"),
    pref_x!("FILE"),
    pref_x!("GOPR"),
    pref_x!("GP01"),
    pref_x!("GX01"),
    pref_x!("GH01"),
    pref_x!("M2U0"),
    pref_x!("MAH0"),
    pref_x!("CIMG"),
    pref_x!("IMGP"),
    pref_x!("Video"),
    pref_x!("MOV0"),
    // ---
    pref_x!("WA0", 3),
    pref_x!("MOL0", 2),
    pref_x!("VTS 01 ", 3),
    pref_x!("AVSEQ", 2),
    // ----
    pref_ymd!("WIN ", 2013),
    pref_ymd!("VID "),
    pref_ymd!("Capture "),
    pref_ymd!("InShot ", 2016),
    pref_ymd!("PXL ", 2020),
    pref_ymd!("AUD-", 2017),
    pref_ymd!("WP ", 2011),
    pref_ymd!("GMT"),
    // ---
    yt_string!("WhatsApp Video ", DT::YYYY_MM_DD(Some(2015))),
    yt_string!("Desktop ", DT::YYYY_MM_DD(None)),
    // ---
    yt_string!(DT::HMS),
    yt_string!(DT::YMD(None)),
    yt_string!(DT::YMD_Now, FilterType::UploadDate),
    yt_string!("\"", DT::MONTH_DD_MM, "\""),
    yt_string!("MOL0", DT::AZ, DT::X(2)),
    yt_string!("VTS ", DT::X(2), " ", DT::X(1)),
    yt_string!("VTS ", DT::X(3), " 1"),
    yt_string!("\"My slideshow ", DT::X(2), "\""),
    yt_string!("\"My Stupeflix Video ", DT::X(3), "\""),
    yt_string!("\"Video ", DT::YMD(Some(2012)), "\""),
    yt_string!("KakaoTalk Video ", DT::YYYY_MM(Some(2012))),
    yt_string!("AVSEQ", DT::X(2), ".DAT"),
    //--
    yt_string!(DT::YMD_Now, FilterType::UploadDate),
    pref_nymd!("Capture "),
    pref_nymd!("WIN "),
    pref_nymd!("VID "),
    pref_nymd!("InShot "),
    pref_nymd!("YouCut "),
    yt_string!("\"Video ", DT::YMD_Now, "\""),
    //--
    yt_string!("240p 400k", FilterType::Playlist),
    yt_string!("480p 600k", FilterType::Playlist),
    yt_string!("720p 1500k", FilterType::Playlist),
    yt_string!("720p 4000k", FilterType::Playlist),
];
