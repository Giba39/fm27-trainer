//! Nederlands / Engels / Português do Brasil.
//! Teksten staan naast elkaar in de code: `t!("Nederlands", "English", "Português")`.
//! FM-termen (sessienamen, factors) blijven Engels of volgen de taal van FM.

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Lang {
    Nl,
    En,
    PtBr,
}

static LANG: AtomicU8 = AtomicU8::new(1);

pub fn set(l: Lang) {
    let v = match l {
        Lang::Nl => 0,
        Lang::En => 1,
        Lang::PtBr => 2,
    };
    LANG.store(v, Ordering::Relaxed);
}

pub fn get() -> Lang {
    match LANG.load(Ordering::Relaxed) {
        0 => Lang::Nl,
        2 => Lang::PtBr,
        _ => Lang::En,
    }
}

pub fn is_nl() -> bool {
    get() == Lang::Nl
}

pub fn is_pt() -> bool {
    get() == Lang::PtBr
}

/// Systeemtaal: Nederlands als het met "nl" begint, Portugees als het met "pt" begint, anders Engels.
pub fn detect() -> Lang {
    match sys_locale::get_locale() {
        Some(l) if l.to_lowercase().starts_with("nl") => Lang::Nl,
        Some(l) if l.to_lowercase().starts_with("pt") => Lang::PtBr,
        _ => Lang::En,
    }
}

/// Kiest de variant op basis van de actieve taal.
/// `t!(nl, en, pt)` kiest de juiste taal.
/// `t!(nl, en)` valt bij Portugees terug op Engels als er geen 3e argument is.
#[macro_export]
macro_rules! t {
    ($nl:expr, $en:expr, $pt:expr $(,)?) => {
        match $crate::i18n::get() {
            $crate::i18n::Lang::Nl => $nl,
            $crate::i18n::Lang::PtBr => $pt,
            $crate::i18n::Lang::En => $en,
        }
    };
    ($nl:expr, $en:expr $(,)?) => {
        match $crate::i18n::get() {
            $crate::i18n::Lang::Nl => $nl,
            $crate::i18n::Lang::PtBr => $en,
            $crate::i18n::Lang::En => $en,
        }
    };
}

/// Maandnamen voor datums in de UI.
pub fn month_short(m: u32) -> &'static str {
    const NL: [&str; 12] = ["jan", "feb", "mrt", "apr", "mei", "jun", "jul", "aug", "sep", "okt", "nov", "dec"];
    const EN: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    const PT: [&str; 12] = ["Jan", "Fev", "Mar", "Abr", "Mai", "Jun", "Jul", "Ago", "Set", "Out", "Nov", "Dez"];
    let i = (m.clamp(1, 12) - 1) as usize;
    match get() {
        Lang::Nl => NL[i],
        Lang::PtBr => PT[i],
        Lang::En => EN[i],
    }
}

pub fn weekday_short(i: usize) -> &'static str {
    const NL: [&str; 7] = ["ma", "di", "wo", "do", "vr", "za", "zo"];
    const EN: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    const PT: [&str; 7] = ["Seg", "Ter", "Qua", "Qui", "Sex", "Sáb", "Dom"];
    match get() {
        Lang::Nl => NL[i % 7],
        Lang::PtBr => PT[i % 7],
        Lang::En => EN[i % 7],
    }
}

pub fn date(d: chrono::NaiveDate) -> String {
    use chrono::Datelike;
    match get() {
        Lang::Nl | Lang::PtBr => format!("{} {} {}", d.day(), month_short(d.month()), d.year()),
        Lang::En => format!("{} {} {}", month_short(d.month()), d.day(), d.year()),
    }
}

pub fn date_short(d: chrono::NaiveDate) -> String {
    use chrono::Datelike;
    match get() {
        Lang::Nl | Lang::PtBr => format!("{} {}", d.day(), month_short(d.month())),
        Lang::En => format!("{} {}", month_short(d.month()), d.day()),
    }
}
