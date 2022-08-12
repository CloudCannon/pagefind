#[cfg(feature = "arabic")]
pub mod arabic;
#[cfg(feature = "armenian")]
pub mod armenian;
#[cfg(feature = "basque")]
pub mod basque;
#[cfg(feature = "catalan")]
pub mod catalan;
#[cfg(feature = "danish")]
pub mod danish;
#[cfg(feature = "dutch")]
pub mod dutch;
#[cfg(feature = "english")]
pub mod english;
#[cfg(feature = "finnish")]
pub mod finnish;
#[cfg(feature = "french")]
pub mod french;
#[cfg(feature = "german")]
pub mod german;
#[cfg(feature = "german2")]
pub mod german2;
#[cfg(feature = "greek")]
pub mod greek;
#[cfg(feature = "hindi")]
pub mod hindi;
#[cfg(feature = "hungarian")]
pub mod hungarian;
#[cfg(feature = "indonesian")]
pub mod indonesian;
#[cfg(feature = "irish")]
pub mod irish;
#[cfg(feature = "italian")]
pub mod italian;
#[cfg(feature = "kraaij_pohlmann")]
pub mod kraaij_pohlmann;
#[cfg(feature = "lithuanian")]
pub mod lithuanian;
#[cfg(feature = "lovins")]
pub mod lovins;
#[cfg(feature = "nepali")]
pub mod nepali;
#[cfg(feature = "norwegian")]
pub mod norwegian;
#[cfg(feature = "porter")]
pub mod porter;
#[cfg(feature = "portuguese")]
pub mod portuguese;
#[cfg(feature = "romanian")]
pub mod romanian;
#[cfg(feature = "russian")]
pub mod russian;
#[cfg(feature = "serbian")]
pub mod serbian;
#[cfg(feature = "spanish")]
pub mod spanish;
#[cfg(feature = "swedish")]
pub mod swedish;
#[cfg(feature = "tamil")]
pub mod tamil;
#[cfg(feature = "turkish")]
pub mod turkish;
#[cfg(feature = "yiddish")]
pub mod yiddish;

pub enum Algorithm {
    #[cfg(feature = "arabic")]
    Arabic,
    #[cfg(feature = "armenian")]
    Armenian,
    #[cfg(feature = "basque")]
    Basque,
    #[cfg(feature = "catalan")]
    Catalan,
    #[cfg(feature = "danish")]
    Danish,
    #[cfg(feature = "dutch")]
    Dutch,
    #[cfg(feature = "english")]
    English,
    #[cfg(feature = "finnish")]
    Finnish,
    #[cfg(feature = "french")]
    French,
    #[cfg(feature = "german")]
    German,
    #[cfg(feature = "german2")]
    German2,
    #[cfg(feature = "greek")]
    Greek,
    #[cfg(feature = "hindi")]
    Hindi,
    #[cfg(feature = "hungarian")]
    Hungarian,
    #[cfg(feature = "indonesian")]
    Indonesian,
    #[cfg(feature = "irish")]
    Irish,
    #[cfg(feature = "italian")]
    Italian,
    #[cfg(feature = "kraaij_pohlmann")]
    Kraaij_pohlmann,
    #[cfg(feature = "lithuanian")]
    Lithuanian,
    #[cfg(feature = "lovins")]
    Lovins,
    #[cfg(feature = "nepali")]
    Nepali,
    #[cfg(feature = "norwegian")]
    Norwegian,
    #[cfg(feature = "porter")]
    Porter,
    #[cfg(feature = "portuguese")]
    Portuguese,
    #[cfg(feature = "romanian")]
    Romanian,
    #[cfg(feature = "russian")]
    Russian,
    #[cfg(feature = "serbian")]
    Serbian,
    #[cfg(feature = "spanish")]
    Spanish,
    #[cfg(feature = "swedish")]
    Swedish,
    #[cfg(feature = "tamil")]
    Tamil,
    #[cfg(feature = "turkish")]
    Turkish,
    #[cfg(feature = "yiddish")]
    Yiddish,
}

/// Retrieves the primary algorithm that this crate was compiled with.
/// Allows consumer crates to change algorithm based only on feature flag usage,
/// without changes to the consuming code.
/// Assumes that the crate was compiled with only one feature flag enabled.
/// If multiple flags are enabled, no guarantee is given on which algorithm is returned
/// (though it will likely be alphabetical).
pub fn get_algorithm() -> Option<fn(&mut super::SnowballEnv) -> bool> {
    #[cfg(feature = "arabic")]
    return Some(arabic::stem);
    #[cfg(feature = "armenian")]
    return Some(armenian::stem);
    #[cfg(feature = "basque")]
    return Some(basque::stem);
    #[cfg(feature = "catalan")]
    return Some(catalan::stem);
    #[cfg(feature = "danish")]
    return Some(danish::stem);
    #[cfg(feature = "dutch")]
    return Some(dutch::stem);
    #[cfg(feature = "english")]
    return Some(english::stem);
    #[cfg(feature = "finnish")]
    return Some(finnish::stem);
    #[cfg(feature = "french")]
    return Some(french::stem);
    #[cfg(feature = "german")]
    return Some(german::stem);
    #[cfg(feature = "german2")]
    return Some(german2::stem);
    #[cfg(feature = "greek")]
    return Some(greek::stem);
    #[cfg(feature = "hindi")]
    return Some(hindi::stem);
    #[cfg(feature = "hungarian")]
    return Some(hungarian::stem);
    #[cfg(feature = "indonesian")]
    return Some(indonesian::stem);
    #[cfg(feature = "irish")]
    return Some(irish::stem);
    #[cfg(feature = "italian")]
    return Some(italian::stem);
    #[cfg(feature = "kraaij_pohlmann")]
    return Some(kraaij_pohlmann::stem);
    #[cfg(feature = "lithuanian")]
    return Some(lithuanian::stem);
    #[cfg(feature = "lovins")]
    return Some(lovins::stem);
    #[cfg(feature = "nepali")]
    return Some(nepali::stem);
    #[cfg(feature = "norwegian")]
    return Some(norwegian::stem);
    #[cfg(feature = "porter")]
    return Some(porter::stem);
    #[cfg(feature = "portuguese")]
    return Some(portuguese::stem);
    #[cfg(feature = "romanian")]
    return Some(romanian::stem);
    #[cfg(feature = "russian")]
    return Some(russian::stem);
    #[cfg(feature = "serbian")]
    return Some(serbian::stem);
    #[cfg(feature = "spanish")]
    return Some(spanish::stem);
    #[cfg(feature = "swedish")]
    return Some(swedish::stem);
    #[cfg(feature = "tamil")]
    return Some(tamil::stem);
    #[cfg(feature = "turkish")]
    return Some(turkish::stem);
    #[cfg(feature = "yiddish")]
    return Some(yiddish::stem);
    None
}

impl From<Algorithm> for fn(&mut super::SnowballEnv) -> bool {
    fn from(lang: Algorithm) -> Self {
        match lang {
            #[cfg(feature = "arabic")]
            Algorithm::Arabic => arabic::stem,
            #[cfg(feature = "armenian")]
            Algorithm::Armenian => armenian::stem,
            #[cfg(feature = "basque")]
            Algorithm::Basque => basque::stem,
            #[cfg(feature = "catalan")]
            Algorithm::Catalan => catalan::stem,
            #[cfg(feature = "danish")]
            Algorithm::Danish => danish::stem,
            #[cfg(feature = "dutch")]
            Algorithm::Dutch => dutch::stem,
            #[cfg(feature = "english")]
            Algorithm::English => english::stem,
            #[cfg(feature = "finnish")]
            Algorithm::Finnish => finnish::stem,
            #[cfg(feature = "french")]
            Algorithm::French => french::stem,
            #[cfg(feature = "german")]
            Algorithm::German => german::stem,
            #[cfg(feature = "german2")]
            Algorithm::German2 => german2::stem,
            #[cfg(feature = "greek")]
            Algorithm::Greek => greek::stem,
            #[cfg(feature = "hindi")]
            Algorithm::Hindi => hindi::stem,
            #[cfg(feature = "hungarian")]
            Algorithm::Hungarian => hungarian::stem,
            #[cfg(feature = "indonesian")]
            Algorithm::Indonesian => indonesian::stem,
            #[cfg(feature = "irish")]
            Algorithm::Irish => irish::stem,
            #[cfg(feature = "italian")]
            Algorithm::Italian => italian::stem,
            #[cfg(feature = "kraaij_pohlmann")]
            Algorithm::Kraaij_pohlmann => kraaij_pohlmann::stem,
            #[cfg(feature = "lithuanian")]
            Algorithm::Lithuanian => lithuanian::stem,
            #[cfg(feature = "lovins")]
            Algorithm::Lovins => lovins::stem,
            #[cfg(feature = "nepali")]
            Algorithm::Nepali => nepali::stem,
            #[cfg(feature = "norwegian")]
            Algorithm::Norwegian => norwegian::stem,
            #[cfg(feature = "porter")]
            Algorithm::Porter => porter::stem,
            #[cfg(feature = "portuguese")]
            Algorithm::Portuguese => portuguese::stem,
            #[cfg(feature = "romanian")]
            Algorithm::Romanian => romanian::stem,
            #[cfg(feature = "russian")]
            Algorithm::Russian => russian::stem,
            #[cfg(feature = "serbian")]
            Algorithm::Serbian => serbian::stem,
            #[cfg(feature = "spanish")]
            Algorithm::Spanish => spanish::stem,
            #[cfg(feature = "swedish")]
            Algorithm::Swedish => swedish::stem,
            #[cfg(feature = "tamil")]
            Algorithm::Tamil => tamil::stem,
            #[cfg(feature = "turkish")]
            Algorithm::Turkish => turkish::stem,
            #[cfg(feature = "yiddish")]
            Algorithm::Yiddish => yiddish::stem,
        }
    }
}
