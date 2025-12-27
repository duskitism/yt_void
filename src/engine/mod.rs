mod library;

use chrono::Datelike;
use std::{
    fmt::Display,
    io::{self, Write},
    sync::LazyLock,
};

#[allow(unused)]
#[repr(u16)]
enum Months {
    January = 1,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Display for Months {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::January => write!(f, "January"),
            Self::February => write!(f, "February"),
            Self::March => write!(f, "March"),
            Self::April => write!(f, "April"),
            Self::May => write!(f, "May"),
            Self::June => write!(f, "June"),
            Self::July => write!(f, "July"),
            Self::August => write!(f, "August"),
            Self::September => write!(f, "September"),
            Self::October => write!(f, "October"),
            Self::November => write!(f, "November"),
            Self::December => write!(f, "December"),
        }
    }
}

impl From<u16> for Months {
    fn from(value: u16) -> Self {
        if !(1..=12).contains(&value) {
            panic!(
                "Couldn't convert number to `Month`: value `{value}` is out of bounds. It should be within range of 1-12."
            )
        }
        // Safety: Shut up
        unsafe { std::mem::transmute(value) }
    }
}

#[allow(non_camel_case_types)]
enum DigitTypes {
    YMD(Option<u16>),
    YMD_Now,
    YYYY_MM_DD(Option<u16>),
    YYYY_MM(Option<u16>),
    MONTH_DD_MM,
    HMS, // Hour, minute, second
    X(u32),
    AZ,
}

static YMD_NOW: LazyLock<(u16, u16, u16)> = LazyLock::new(|| {
    let now = chrono::Utc::now().date_naive();
    (now.year() as u16, now.month() as u16, now.day() as u16)
});

fn get_rand_ymd(y_cap: Option<u16>) -> (u16, u16, u16) {
    const DEFAULT_YEAR_CAP: u16 = 2008;

    let year = rand::random_range(y_cap.unwrap_or(DEFAULT_YEAR_CAP)..=YMD_NOW.0); // current year
    let month = rand::random_range(1..=12u16);

    use Months as M;
    let day_cap = match Months::from(month) {
        M::January | M::March | M::May | M::July | M::August | M::October | M::December => 31,
        M::February => {
            // February mumbo-jumbo, check if it's a leap year
            if (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => 30,
    };

    let day = rand::random_range(1..=day_cap);

    (year, month, day)
}

impl Display for DigitTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::YMD(cap) => {
                let (y, m, d) = get_rand_ymd(*cap);
                write!(f, "{y}{m:0>2}{d:0>2}")?;
            }
            Self::YMD_Now => {
                let (y, m, d) = *YMD_NOW;
                write!(f, "{y}{m:0>2}{d:0>2}")?;
            }
            Self::YYYY_MM_DD(cap) => {
                let (y, m, d) = get_rand_ymd(*cap);
                write!(f, "{y} {m:0>2} {d:0>2}")?;
            }
            Self::YYYY_MM(cap) => {
                let (y, m, ..) = get_rand_ymd(*cap);
                write!(f, "{y} {m:0>2}")?;
            }
            Self::MONTH_DD_MM => {
                let (y, m, d) = get_rand_ymd(None);
                write!(f, "{} {d:0>2}, {y}", Months::from(m))?;
            }
            Self::HMS => write!(
                f,
                "{:0>2}{:0>2}{:0>2}",
                rand::random_range(0..24),
                rand::random_range(0..60),
                rand::random_range(0..60)
            )?,
            Self::X(digits) => {
                let zeros = *digits as usize;
                write!(f, "{:0>zeros$}", rand::random_range(0..10u32.pow(*digits)))?
            }
            Self::AZ => {
                let rand_letter = rand::random_range(0..26u8);

                write!(f, "{}", (65 /* (a) */+ rand_letter) as char)?;
            }
        };

        Ok(())
    }
}

enum FilterType {
    Playlist,
    UploadDate,
}

impl Display for FilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Playlist => write!(f, /* Tokens: */ "&sp=EgIQAw%253D%253D"),
            Self::UploadDate => write!(f, "&sp=CAI%253D"),
        }
    }
}

struct YtString {
    components: &'static [&'static dyn Display],
}

impl Display for YtString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let link = format!(
            "https://www.youtube.com/results?search_query={}",
            self.components
                .iter()
                .map(|item| item.to_string())
                .collect::<String>()
                .replace(" ", "+")
        );
        write!(f, "{link}")?;

        Ok(())
    }
}

macro_rules! input {
    ($($items:expr), + $(,)?) => {
        {
            let mut buf = String::new();

            print!($($items), +);

            io::stdout()
                .flush()
                .and_then(|_| io::stdin().read_line(&mut buf))
                .expect("Failed to read stdin for some reason");

            buf.truncate(buf.trim_end().len());
            buf
        }
    };
    () => {
        compile_error!("Requires at least one argument");
    };
}

pub fn init() {
    loop {
        let amt = loop {
            match input!("How many do you want: ").trim().parse::<u32>() {
                Ok(0) => {
                    break {
                        println!("Whatever man");
                        0
                    };
                }
                Ok(n) => break n,
                _ => println!("broh"),
            }
        };

        for i in 0..amt {
            println!(
                "{}: [ {} ]",
                i + 1,
                library::LIBRARY[rand::random_range(0..library::LIBRARY.len())]
            );
        }

        loop {
            // Supports "nah", "yes", "no", "ye", etch. Crazy stuff.
            match input!("Do you want more (y/n): ")
                .to_lowercase()
                .trim()
                .get(..1)
            {
                Some("y") => break,
                Some("n") => return,
                _ => println!("broh"),
            }
        }
    }
}
