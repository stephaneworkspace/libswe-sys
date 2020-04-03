/*
 * Traditional astrology for rust
 * ==============================
 *
 * Rust library by Stéphane (s.bressani@bluewin.ch)
 *
 * Using swissephem c library by Astrodienst AG
 * by Dieter Koch and Alois Treindl (https://www.astro.com/ftp/swisseph/)
 *
 * The source code is released under an CC License, which allows it to be used
 * also on commercial projects. This software uses the swiss ephemeris which is
 * licensed GPL.
 *
 * Therefore, if you want to use astro_compute_swisseph in your commercial
 * projects, you must adhere to the GPL license or buy a Swiss Ephemeris
 * commercial license.
 */
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate strum;
use crate::swerust::handler_swe17::{split_deg, SplitDegResult};
use serde::{Deserialize, Serialize};
use strum::AsStaticRef;

/// Language available
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    English = 0,
    French = 1,
}

/// Zodiac
#[derive(Debug, Clone, Display, EnumIter)]
pub enum Signs {
    Aries = 1,
    Taurus = 2,
    Gemini = 3,
    Cancer = 4,
    Leo = 5,
    Virgo = 6,
    Libra = 7,
    Scorpio = 8,
    Sagittarius = 9,
    Capricorn = 10,
    Aquarius = 11,
    Pisces = 12,
}

impl Signs {
    /// Text for translation
    pub fn text(self, lang: Language) -> String {
        match lang {
            Language::English => match self {
                Signs::Aries => "Aries".to_string(),
                Signs::Taurus => "Taurus".to_string(),
                Signs::Gemini => "Gemini".to_string(),
                Signs::Cancer => "Cancer".to_string(),
                Signs::Leo => "Leo".to_string(),
                Signs::Virgo => "Virgo".to_string(),
                Signs::Libra => "Libra".to_string(),
                Signs::Scorpio => "Scorpio".to_string(),
                Signs::Sagittarius => "Sagittarius".to_string(),
                Signs::Capricorn => "Capricorn".to_string(),
                Signs::Aquarius => "Aquarius".to_string(),
                Signs::Pisces => "Pisces".to_string(),
            },
            Language::French => match self {
                Signs::Aries => "Belier".to_string(),
                Signs::Taurus => "Taureau".to_string(),
                Signs::Gemini => "Gemaux".to_string(),
                Signs::Cancer => "Cancer".to_string(),
                Signs::Leo => "Lio".to_string(),
                Signs::Virgo => "Vierge".to_string(),
                Signs::Libra => "Balance".to_string(),
                Signs::Scorpio => "Scorpion".to_string(),
                Signs::Sagittarius => "Sagittaire".to_string(),
                Signs::Capricorn => "Capricorne".to_string(),
                Signs::Aquarius => "Verseau".to_string(),
                Signs::Pisces => "Poisson".to_string(),
            },
        }
    }
}

/// Element
#[derive(Debug, Clone, Copy, PartialEq, AsStaticStr)]
pub enum Element {
    Fire = 0xFF0000,  // Red
    Earth = 0xFFC200, // Orange/Yellow
    Wind = 0x00C42A,  // Green
    Water = 0x0B34FF, // Blue
}

pub const COLOR_BODIE_SUN: i32 = 0xFFA300; // Orange
pub const COLOR_BODIE_MOON: i32 = 0xB5B510; // Yellow
pub const COLOR_BODIE_MERCURY: i32 = 0x6900FF; // Indigo
pub const COLOR_BODIE_VENUS: i32 = 0xFF009E; // Pink
pub const COLOR_BODIE_MARS: i32 = 0xFF1212; // Red small ligth
pub const COLOR_BODIE_JUPITER: i32 = 0x12A5FF; // Blue ligth
pub const COLOR_BODIE_SATURN: i32 = 0xCC0000; // Red CC
pub const COLOR_BODIE_URANUS: i32 = 0xA89402; // Brown
pub const COLOR_BODIE_NEPTUNE: i32 = 0x00B526; // Green small ligth
pub const COLOR_BODIE_PLUTO: i32 = 0xBF3A3A; // Red special
pub const COLOR_BODIE_OTHER: i32 = 0x6B6B6B; // Gray

/// Bodies
#[derive(Debug, Clone, Copy, PartialEq, Display, EnumIter, AsStaticStr)]
pub enum Bodies {
    EclNut = -1,
    Sun = 0,
    Moon = 1,
    Mercury = 2,
    Venus = 3,
    Mars = 4,
    Jupiter = 5,
    Saturn = 6,
    Uranus = 7,
    Neptune = 8,
    Pluto = 9,
    MeanNode = 10,
    TrueNode = 11,
    MeanApog = 12,
    OscuApog = 13,
    Earth = 14,
    Chiron = 15,
    Pholus = 16,
    Ceres = 17,
    Pallas = 18,
    Juno = 19,
    Vesta = 20,
    IntpApog = 21,
    IntpPerg = 22,
    NPlanets = 23,
    SouthNode = 24,
    FortunaPart = 25,
    // SE_FICT_OFFSET = 40,
    // SE_NFICT_ELEM = 15,
    // SE_AST_OFFSET = 10000,
    /* Hamburger or Uranian "planets" */
    Cupido = 40,
    Hades = 41,
    Zeus = 42,
    Kronos = 43,
    Apollon = 44,
    Admetos = 45,
    Vulkanus = 46,
    Poseidon = 47,
    /* other fictitious bodies */
    Isis = 48,
    Nibiru = 49,
    Harrington = 50,
    NeptuneLeverrier = 51,
    NeptuneAdams = 52,
    PlutoLowell = 53,
    PlutoPickering = 54,
    /* Asteroid */
    AsteroidAstera = 10000 + 5,
    AsteroidHebe = 10000 + 6,
    AsteroidIris = 10000 + 7,
    AsteroidFlora = 10000 + 8,
    AsteroidMetis = 10000 + 9,
    AsteroidHygiea = 10000 + 10,
    AsteroidUrania = 10000 + 30,
    AsteroidIsis = 10000 + 42,
    AsteroidHilda = 10000 + 153,
    AsteroidPhilosophia = 10000 + 227,
    AsteroidSophia = 10000 + 251,
    AsteroidAletheia = 10000 + 259,
    AsteroidSapientia = 10000 + 275,
    AsteroidThule = 10000 + 279,
    AsteroidUrsula = 10000 + 375,
    AsteroidEros = 10000 + 433,
    AsteroidCupido = 10000 + 763,
    AsteroidHidalgo = 10000 + 944,
    AsteroidLilith = 10000 + 1181,
    AsteroidAmor = 10000 + 1221,
    AsteroidKama = 10000 + 1387,
    AsteroidAphrodite = 10000 + 1388,
    AsteroidApollo = 10000 + 1862,
    AsteroidDamocles = 10000 + 3553,
    AsteroidCruithne = 10000 + 3753,
    AsteroidPoseidon = 10000 + 4341,
    AsteroidVulcano = 10000 + 4464,
    AsteroidZeus = 10000 + 5731,
    AsteroidNessus = 10000 + 7066,
}

/// Object type
#[derive(Debug, Clone, PartialEq, Display, EnumIter)]
pub enum ObjectType {
    Unknown,
    PlanetOrStar,
    Earth,
    Fiction,
    Asteroid,
}

impl Bodies {
    /// Object type of Bodies
    pub fn object_type(self) -> ObjectType {
        match self {
            Bodies::EclNut => ObjectType::Unknown,
            Bodies::Sun => ObjectType::PlanetOrStar,
            Bodies::Moon => ObjectType::PlanetOrStar,
            Bodies::Mercury => ObjectType::PlanetOrStar,
            Bodies::Venus => ObjectType::PlanetOrStar,
            Bodies::Mars => ObjectType::PlanetOrStar,
            Bodies::Jupiter => ObjectType::PlanetOrStar,
            Bodies::Saturn => ObjectType::PlanetOrStar,
            Bodies::Uranus => ObjectType::PlanetOrStar,
            Bodies::Neptune => ObjectType::PlanetOrStar,
            Bodies::Pluto => ObjectType::PlanetOrStar,
            Bodies::MeanNode => ObjectType::PlanetOrStar,
            Bodies::TrueNode => ObjectType::PlanetOrStar,
            Bodies::MeanApog => ObjectType::PlanetOrStar,
            Bodies::OscuApog => ObjectType::PlanetOrStar,
            Bodies::Earth => ObjectType::Earth,
            Bodies::Chiron => ObjectType::Fiction,
            Bodies::Pholus => ObjectType::Fiction,
            Bodies::Ceres => ObjectType::Fiction,
            Bodies::Pallas => ObjectType::Fiction,
            Bodies::Juno => ObjectType::Fiction,
            Bodies::Vesta => ObjectType::Fiction,
            Bodies::IntpApog => ObjectType::Fiction,
            Bodies::IntpPerg => ObjectType::Fiction,
            Bodies::NPlanets => ObjectType::Fiction,
            Bodies::SouthNode => ObjectType::Fiction,
            Bodies::FortunaPart => ObjectType::Fiction,
            Bodies::Cupido => ObjectType::Fiction,
            Bodies::Hades => ObjectType::Fiction,
            Bodies::Zeus => ObjectType::Fiction,
            Bodies::Kronos => ObjectType::Fiction,
            Bodies::Apollon => ObjectType::Fiction,
            Bodies::Admetos => ObjectType::Fiction,
            Bodies::Vulkanus => ObjectType::Fiction,
            Bodies::Poseidon => ObjectType::Fiction,
            Bodies::Isis => ObjectType::Fiction,
            Bodies::Nibiru => ObjectType::Fiction,
            Bodies::Harrington => ObjectType::Fiction,
            Bodies::NeptuneLeverrier => ObjectType::Fiction,
            Bodies::NeptuneAdams => ObjectType::Fiction,
            Bodies::PlutoLowell => ObjectType::Fiction,
            Bodies::PlutoPickering => ObjectType::Fiction,
            Bodies::AsteroidAstera => ObjectType::Asteroid,
            Bodies::AsteroidHebe => ObjectType::Asteroid,
            Bodies::AsteroidIris => ObjectType::Asteroid,
            Bodies::AsteroidFlora => ObjectType::Asteroid,
            Bodies::AsteroidMetis => ObjectType::Asteroid,
            Bodies::AsteroidHygiea => ObjectType::Asteroid,
            Bodies::AsteroidUrania => ObjectType::Asteroid,
            Bodies::AsteroidIsis => ObjectType::Asteroid,
            Bodies::AsteroidHilda => ObjectType::Asteroid,
            Bodies::AsteroidPhilosophia => ObjectType::Asteroid,
            Bodies::AsteroidSophia => ObjectType::Asteroid,
            Bodies::AsteroidAletheia => ObjectType::Asteroid,
            Bodies::AsteroidSapientia => ObjectType::Asteroid,
            Bodies::AsteroidThule => ObjectType::Asteroid,
            Bodies::AsteroidUrsula => ObjectType::Asteroid,
            Bodies::AsteroidEros => ObjectType::Asteroid,
            Bodies::AsteroidCupido => ObjectType::Asteroid,
            Bodies::AsteroidHidalgo => ObjectType::Asteroid,
            Bodies::AsteroidLilith => ObjectType::Asteroid,
            Bodies::AsteroidAmor => ObjectType::Asteroid,
            Bodies::AsteroidKama => ObjectType::Asteroid,
            Bodies::AsteroidAphrodite => ObjectType::Asteroid,
            Bodies::AsteroidApollo => ObjectType::Asteroid,
            Bodies::AsteroidDamocles => ObjectType::Asteroid,
            Bodies::AsteroidCruithne => ObjectType::Asteroid,
            Bodies::AsteroidPoseidon => ObjectType::Asteroid,
            Bodies::AsteroidVulcano => ObjectType::Asteroid,
            Bodies::AsteroidZeus => ObjectType::Asteroid,
            Bodies::AsteroidNessus => ObjectType::Asteroid,
        }
    }

    /// Object color
    pub fn object_color(self) -> i32 {
        match self {
            Bodies::EclNut => COLOR_BODIE_OTHER,
            Bodies::Sun => COLOR_BODIE_SUN,
            Bodies::Moon => COLOR_BODIE_MOON,
            Bodies::Mercury => COLOR_BODIE_MERCURY,
            Bodies::Venus => COLOR_BODIE_VENUS,
            Bodies::Mars => COLOR_BODIE_MARS,
            Bodies::Jupiter => COLOR_BODIE_JUPITER,
            Bodies::Saturn => COLOR_BODIE_SATURN,
            Bodies::Uranus => COLOR_BODIE_URANUS,
            Bodies::Neptune => COLOR_BODIE_NEPTUNE,
            Bodies::Pluto => COLOR_BODIE_PLUTO,
            Bodies::MeanNode => COLOR_BODIE_OTHER,
            Bodies::TrueNode => COLOR_BODIE_OTHER,
            Bodies::MeanApog => COLOR_BODIE_OTHER,
            Bodies::OscuApog => COLOR_BODIE_OTHER,
            Bodies::Earth => COLOR_BODIE_OTHER,
            Bodies::Chiron => COLOR_BODIE_OTHER,
            Bodies::Pholus => COLOR_BODIE_OTHER,
            Bodies::Ceres => COLOR_BODIE_OTHER,
            Bodies::Pallas => COLOR_BODIE_OTHER,
            Bodies::Juno => COLOR_BODIE_OTHER,
            Bodies::Vesta => COLOR_BODIE_OTHER,
            Bodies::IntpApog => COLOR_BODIE_OTHER,
            Bodies::IntpPerg => COLOR_BODIE_OTHER,
            Bodies::NPlanets => COLOR_BODIE_OTHER,
            Bodies::SouthNode => COLOR_BODIE_OTHER,
            Bodies::FortunaPart => COLOR_BODIE_OTHER,
            Bodies::Cupido => COLOR_BODIE_OTHER,
            Bodies::Hades => COLOR_BODIE_OTHER,
            Bodies::Zeus => COLOR_BODIE_OTHER,
            Bodies::Kronos => COLOR_BODIE_OTHER,
            Bodies::Apollon => COLOR_BODIE_OTHER,
            Bodies::Admetos => COLOR_BODIE_OTHER,
            Bodies::Vulkanus => COLOR_BODIE_OTHER,
            Bodies::Poseidon => COLOR_BODIE_OTHER,
            Bodies::Isis => COLOR_BODIE_OTHER,
            Bodies::Nibiru => COLOR_BODIE_OTHER,
            Bodies::Harrington => COLOR_BODIE_OTHER,
            Bodies::NeptuneLeverrier => COLOR_BODIE_OTHER,
            Bodies::NeptuneAdams => COLOR_BODIE_OTHER,
            Bodies::PlutoLowell => COLOR_BODIE_OTHER,
            Bodies::PlutoPickering => COLOR_BODIE_OTHER,
            Bodies::AsteroidAstera => COLOR_BODIE_OTHER,
            Bodies::AsteroidHebe => COLOR_BODIE_OTHER,
            Bodies::AsteroidIris => COLOR_BODIE_OTHER,
            Bodies::AsteroidFlora => COLOR_BODIE_OTHER,
            Bodies::AsteroidMetis => COLOR_BODIE_OTHER,
            Bodies::AsteroidHygiea => COLOR_BODIE_OTHER,
            Bodies::AsteroidUrania => COLOR_BODIE_OTHER,
            Bodies::AsteroidIsis => COLOR_BODIE_OTHER,
            Bodies::AsteroidHilda => COLOR_BODIE_OTHER,
            Bodies::AsteroidPhilosophia => COLOR_BODIE_OTHER,
            Bodies::AsteroidSophia => COLOR_BODIE_OTHER,
            Bodies::AsteroidAletheia => COLOR_BODIE_OTHER,
            Bodies::AsteroidSapientia => COLOR_BODIE_OTHER,
            Bodies::AsteroidThule => COLOR_BODIE_OTHER,
            Bodies::AsteroidUrsula => COLOR_BODIE_OTHER,
            Bodies::AsteroidEros => COLOR_BODIE_OTHER,
            Bodies::AsteroidCupido => COLOR_BODIE_OTHER,
            Bodies::AsteroidHidalgo => COLOR_BODIE_OTHER,
            Bodies::AsteroidLilith => COLOR_BODIE_OTHER,
            Bodies::AsteroidAmor => COLOR_BODIE_OTHER,
            Bodies::AsteroidKama => COLOR_BODIE_OTHER,
            Bodies::AsteroidAphrodite => COLOR_BODIE_OTHER,
            Bodies::AsteroidApollo => COLOR_BODIE_OTHER,
            Bodies::AsteroidDamocles => COLOR_BODIE_OTHER,
            Bodies::AsteroidCruithne => COLOR_BODIE_OTHER,
            Bodies::AsteroidPoseidon => COLOR_BODIE_OTHER,
            Bodies::AsteroidVulcano => COLOR_BODIE_OTHER,
            Bodies::AsteroidZeus => COLOR_BODIE_OTHER,
            Bodies::AsteroidNessus => COLOR_BODIE_OTHER,
        }
    }

    /// Text translate
    pub fn text(self, lang: Language) -> String {
        match lang {
            Language::English => match self {
                Bodies::TrueNode => "North node".to_string(),
                Bodies::SouthNode => "South node".to_string(),
                Bodies::FortunaPart => "Fortuna part".to_string(),
                _ => self.as_static().to_string(),
            },
            Language::French => match self {
                Bodies::Sun => "Soleil".to_string(),
                Bodies::Moon => "Lune".to_string(),
                Bodies::Mercury => "Mercure".to_string(),
                Bodies::Venus => "Venus".to_string(),
                Bodies::Mars => "Mars".to_string(),
                Bodies::Jupiter => "Jupiter".to_string(),
                Bodies::Saturn => "Saturne".to_string(),
                Bodies::Uranus => "Uranus".to_string(),
                Bodies::Neptune => "Neptune".to_string(),
                Bodies::Pluto => "Pluton".to_string(),
                Bodies::TrueNode => "Noeud nord".to_string(),
                Bodies::Chiron => "Chiron".to_string(),
                Bodies::Ceres => "Ceres".to_string(),
                Bodies::SouthNode => "Noeud sud".to_string(),
                Bodies::FortunaPart => "Part de fortune".to_string(),
                _ => self.as_static().to_string(),
            },
        }
    }
}

/// Object position (direction)
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectPos {
    Stationary,
    Direct,
    Retrograde,
}

/// Object
#[derive(Debug, Clone)]
pub struct Object {
    pub object_enum: Bodies,
    pub object_name: String,
    pub object_type: ObjectType,
    pub longitude: f64,
    pub latitude: f64,
    pub speed_longitude: f64,
    pub object_pos: ObjectPos,
    pub split: SplitDegResult,
}

impl Object {
    /// Constructor
    pub fn new(
        object_enum: Bodies,
        object_name: &str,
        object_type: ObjectType,
        longitude: f64,
        latitude: f64,
        speed_longitude: f64,
    ) -> Object {
        let object_pos;
        if f64::abs(speed_longitude) < 0.0003 {
            object_pos = ObjectPos::Stationary;
        } else if speed_longitude > 0.0 {
            object_pos = ObjectPos::Direct;
        } else {
            object_pos = ObjectPos::Retrograde;
        }
        Object {
            object_enum: object_enum,
            object_name: object_name.to_string(),
            object_type: object_type,
            longitude: longitude,
            latitude: latitude,
            speed_longitude: speed_longitude,
            object_pos: object_pos,
            split: split_deg(longitude, 0),
        }
    }
}

/// House
#[derive(Debug, Clone)]
pub struct House {
    pub object_id: i32,
    pub longitude: f64,
    pub split: SplitDegResult,
    pub angle: Angle,
}

impl House {
    /// Constructor
    pub fn new(object_id: i32, longitude: f64, angle: Angle) -> House {
        House {
            object_id: object_id,
            longitude: longitude,
            split: split_deg(longitude, 0),
            angle: angle,
        }
    }
}

/// Angle
#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
pub enum Angle {
    Nothing = 0,
    Asc = 1,
    Fc = 2,
    Desc = 3,
    Mc = 4,
}

/// Type of calandar
pub enum Calandar {
    Julian = 0,
    Gregorian = 1,
}

/// Optional flag swissephem
pub enum OptionalFlag {
    JplEph = 1,
    SwissEph = 2,
    Moshier = 4,
    Heliocentric = 8,
    TruePosition = 16,
    J2000Equinox = 32,
    NoNutation = 64,
    Speed3 = 128,
    Speed = 256,
    NoGravitanionalDeflection = 512,
    NoAnnualAberration = 1024,
    AstronomicPosition = 1024 | 512,
    // AstronomicPosition = OptionalFlag::NoAnnualAberration
    //     | OptionalFlag::NoGravitanionalDeflection,
    EquatorialPosition = 2 * 1024,
    XYZCartesianNotPolarCoordinate = 4 * 1024,
    Radians = 8 * 1024,
    BarycentricPosition = 16 * 1024,
    TopocentricPosition = 32 * 1024,
    SideralPosition = 64 * 1024,
    ICRS = 128 * 1024,
    Dpsideps1980 = 256 * 1024,
    JplHorApprox = 512 * 1024,
}

/// House system
/// I have put in enum only the most important houses methods
/// To do
pub enum HouseSystem {
    Campanus,
    Equal,
    Koch,
    Placidus,
    Porphyrius,
    Regiomontanus,
    WholeSign,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Display,
    EnumIter,
    AsStaticStr,
    Serialize,
    Deserialize,
)]
pub enum Aspects {
    Conjunction = 0,
    Opposition = 1,
    Trine = 2,
    Square = 3,
    Sextile = 4,
    Inconjunction = 5,
    Sesquisquare = 6,
    Semisquare = 7,
    Semisextile = 8,
}

impl Aspects {
    /// Aspect propriety -> (Aspects, Orbes)
    pub fn angle(self) -> (u16, u16) {
        match self {
            Aspects::Conjunction => (0, 10),
            Aspects::Opposition => (180, 8),
            Aspects::Trine => (120, 7),
            Aspects::Square => (90, 6),
            Aspects::Sextile => (60, 5),
            Aspects::Inconjunction => (150, 2),
            Aspects::Sesquisquare => (135, 1),
            Aspects::Semisquare => (45, 1),
            Aspects::Semisextile => (30, 1),
        }
    }

    /// Major aspect -> bool
    pub fn maj(self) -> bool {
        match self {
            Aspects::Conjunction => true,
            Aspects::Opposition => true,
            Aspects::Trine => true,
            Aspects::Square => true,
            Aspects::Sextile => true,
            _ => false,
        }
    }

    pub fn text(self, lang: Language) -> String {
        match lang {
            Language::English => match self {
                Aspects::Conjunction => "Conjunction".to_string(),
                Aspects::Opposition => "Opposition".to_string(),
                Aspects::Trine => "Trine".to_string(),
                Aspects::Square => "Square".to_string(),
                Aspects::Sextile => "Sextile".to_string(),
                Aspects::Inconjunction => "Inconjunction".to_string(),
                Aspects::Sesquisquare => "Sesquisquare".to_string(),
                Aspects::Semisquare => "Semisquare".to_string(),
                Aspects::Semisextile => "Semisextile".to_string(),
            },
            Language::French => match self {
                Aspects::Conjunction => "Conjonction".to_string(),
                Aspects::Opposition => "Opposition".to_string(),
                Aspects::Trine => "Trigone".to_string(),
                Aspects::Square => "Quadrature".to_string(),
                Aspects::Sextile => "Sextile".to_string(),
                Aspects::Inconjunction => "Quinconce".to_string(),
                Aspects::Sesquisquare => "Sesqui-carré".to_string(),
                Aspects::Semisquare => "Demi-carré".to_string(),
                Aspects::Semisextile => "Demi-sextile".to_string(),
            },
        }
    }
}
