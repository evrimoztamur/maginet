//! Stable campaign metadata shared by the UI and native analysis.
use serde::{Deserialize, Serialize};

use crate::{BoardStyle, Level};
/// Tutorial saved-progress code.
pub const TUTORIAL_CODE: &str = "hg18a09m4g0m81g00c4068035g14r0v008";
/// Tutorial map position.
pub const TUTORIAL_POSITION: (isize, isize) = (0, 1);
/// One campaign portal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignEntry {
    /// Stable identity, independent of code and map position.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Stable level and saved-progress code.
    pub code: String,
    /// Cardinal map coordinate.
    pub position: (isize, isize),
    /// Presentation style.
    pub style: BoardStyle,
    /// Included in demo builds.
    pub demo: bool,
    /// Guided tutorial identity.
    pub tutorial: bool,
}
impl CampaignEntry {
    /// Decode and apply presentation metadata.
    pub fn level(&self) -> Level {
        let mut level = Level::parse_code(&self.code).expect("valid catalogue code");
        level.board.style = self.style.clone();
        level
    }
}
/// Board style at a map column, independent of level encoding.
pub fn campaign_style(column: isize) -> BoardStyle {
    match column {
        ..=2 => BoardStyle::Grass,
        3..=6 => BoardStyle::Desert,
        7 => BoardStyle::Flesh,
        8..=11 => BoardStyle::Crust,
        _ => BoardStyle::Eldritch,
    }
}
/// Full catalogue in stable order, optionally filtered to demo portals.
pub fn campaign_catalogue(demo: bool) -> Vec<CampaignEntry> {
    let definitions = [
        ("basics-i", "Basics I", "hg12g014cm0j800", (0, 0), true, false),
        ("basics-ii", "Basics II", "e01jg1248m0j8k834g00", (1, 0), true, false),
        ("basics-iii", "Basics III", "j0228014cm0j8v804gp04900", (2, 0), true, false),
        ("basics-iv", "Basics IV", "j022801mcm0j8v804gp04d06201g00s80dm07403g01g", (2, -1), true, false),
        ("patterns-i", "Patterns I", "j032a0j4gm148t818h602n9g09as00jn09r06h03", (3, -1), false, false),
        ("patterns-ii", "Patterns II", "pg2620a48m1m8c038ht02h04gg1jr0wg0d406", (4, -1), false, false),
        ("patterns-iii", "Patterns III", "pg3220j4g41m8h818gr06h4g052780j400", (5, -1), false, false),
        ("diagonals-i", "Diagonals I", "d010812ncm23809408", (4, 0), false, false),
        ("diagonals-ii", "Diagonals II", "f02220t4840m8e018hc06h04a014g0sg0cm04", (4, 1), false, false),
        ("diagonals-iii", "Diagonals III", "bg3200240g248h038gcg6h2s0h23t02408r04b02", (4, 2), false, false),
        ("diagonals-iv", "Diagonals IV", "k036202444148h818ha02h1r0127g0j40m604k01dg1jr0wc08", (4, 3), false, false),
        ("beams-i", "Beams I", "hg22r024dg0m81816j2g0d02500pg08", (5, -2), false, false),
        ("beams-ii", "Beams II", "eg3020t4c40489818gr02h0m0d2780240gp06a03d00pr08", (5, -3), false, false),
        ("beams-iii", "Beams III", "qg22j0t4h41m8d038ja06h04gg13g0mr04j02", (6, -3), false, false),
        ("shields-i", "Shields I", "d012g0an840k80h401200", (5, 2), false, false),
        ("shields-ii", "Shields II", "xg2420a4r40m9b018gp02h06x00080140f406t02gg10", (6, 2), false, false),
        ("shields-iii", "Shields III", "j02280j4500m8t818hpg4h025g06800", (7, 2), false, false),
        ("challenge-i", "Challenge I", "hg2280a4d40490008g6g2h02cg12g00", (2, 1), false, false),
        ("challenge-ii", "Challenge II", "hg2680t44m048a028hmg2h04000gr0mc06004", (6, -1), false, false),
        ("challenge-iii", "Challenge III", "q03220t4840m98828gw02h2r0d2bg0j40x804j03dg0k00s80a807200", (7, -3), false, false),
        ("challenge-iv", "Challenge IV", "qg3200t4000m90048jeg6h5x0523t1241e606c03701s00wm02206j015g1k80h802404", (7, 1), false, false),
        ("rite-i", "Rite I", "t04420a4041m90818k0g6h2g052900a4t01m84038g2tr0n80cm06902d00g", (7, -2), false, false),
        ("rite-ii", "Rite II", "p022a0t4gm048b008j606h06901pg0r80am04k00cg00", (7, -1), false, false),
        ("rite-iii", "Rite III", "pg3820a44m2482808jp00h4g0h2380a410r04000m01r80nm00a06a03hg1g", (7, 0), false, false),
        ("rite-iv", "Rite IV", "j030r024a00m93028g2g4h210528a0240wp04s02900j803c00407203", (8, 0), false, false),
        ("ascension-i", "Ascension I", "zg322024w42499828hw04h6w0h25r02410t05j02n01j80vg0et00k01v01g", (8, -1), false, false),
        ("ascension-ii", "Ascension II", "zg4200t4000m90048kg00h4x0d2bt0a47m249z808g78r0sg0cw07403jg0f80w40d403m025g1k80h802405b03", (9, -1), false, false),
        ("junction-i", "Junction I", "d0108124cm23809400", (9, -1), false, false),
        ("junction-ii", "Junction II", "9010g0248403809408", (10, -2), false, false),
        ("junction-iii", "Junction III", "d01080a4cm0k809404", (11, -2), false, false),
        ("junction-iv", "Junction IV", "9g12r1244423809800", (12, -2), false, false),
        ("junction-v", "Junction V", "dg10g0j4cm0380h809200", (12, -1), false, false),
        ("junction-vi", "Junction VI", "901081248m03809404", (10, 1), false, false),
        ("junction-vii", "Junction VII", "d012g024840k809400", (11, 1), false, false),
        ("junction-viii", "Junction VIII", "9g10g0a48m2380h808j02", (12, 1), false, false),
        ("tutorial", "Tutorial", TUTORIAL_CODE, TUTORIAL_POSITION, true, true),
    ];
    definitions
        .into_iter()
        .filter(|e| !demo || e.4)
        .map(|(id, name, code, position, demo, tutorial)| CampaignEntry {
            id: id.into(),
            name: name.into(),
            code: code.into(),
            position: portal_position(name, position),
            style: campaign_style(portal_position(name, position).0),
            demo,
            tutorial,
        })
        .collect()
}

/// Ordered teaching route, including the tutorial.
pub const MAIN_ROUTE: &[&str] = &[
    "tutorial",
    "basics-i",
    "basics-ii",
    "basics-iii",
    "basics-iv",
    "patterns-i",
    "patterns-ii",
    "patterns-iii",
    "diagonals-i",
    "beams-i",
    "shields-i",
    "junction-i",
    "rite-i",
    "rite-ii",
    "rite-iii",
    "rite-iv",
    "ascension-i",
    "ascension-ii",
];
/// Optional routes: entrance, battles, and destination. Only the final edge is one-way.
pub const OPTIONAL_ROUTES: &[&[&str]] = &[
    &["diagonals-i", "diagonals-ii", "diagonals-iii", "beams-i"],
    &[
        "beams-i",
        "diagonals-iv",
        "beams-ii",
        "beams-iii",
        "challenge-ii",
        "shields-i",
    ],
    &["shields-i", "junction-i", "challenge-i", "rite-ii"],
    &[
        "shields-i",
        "junction-i",
        "challenge-iii",
        "junction-ii",
        "junction-iii",
        "junction-iv",
        "junction-v",
        "rite-iv",
    ],
    &[
        "shields-i",
        "shields-ii",
        "shields-iii",
        "challenge-iv",
        "junction-vi",
        "junction-vii",
        "junction-viii",
        "rite-iv",
    ],
];
/// An explicit connection. Ordinary connections can be followed in either direction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CampaignConnection {
    /// Entrance portal ID.
    pub from: String,
    /// Destination portal ID.
    pub to: String,
    /// Whether travel and unlocks only run from entrance to destination.
    pub one_way: bool,
}
/// Connections filtered to the supplied catalogue (including demo/custom scenarios).
pub fn campaign_connections(entries: &[CampaignEntry]) -> Vec<CampaignConnection> {
    let mut edges = Vec::new();
    for (route, branch) in
        std::iter::once((MAIN_ROUTE, false)).chain(OPTIONAL_ROUTES.iter().map(|r| (*r, true)))
    {
        for (i, pair) in route.windows(2).enumerate() {
            if pair.iter().all(|id| entries.iter().any(|e| e.id == *id)) {
                let edge = CampaignConnection {
                    from: pair[0].into(),
                    to: pair[1].into(),
                    one_way: branch && i == route.len() - 2,
                };
                if !edges.contains(&edge) {
                    edges.push(edge);
                }
            }
        }
    }
    edges
}
/// Whether winning `from` can unlock `to`.
pub fn campaign_connected(edges: &[CampaignConnection], from: &str, to: &str) -> bool {
    edges
        .iter()
        .any(|e| e.from == from && e.to == to || !e.one_way && e.to == from && e.from == to)
}
fn portal_position(name: &str, original: (isize, isize)) -> (isize, isize) {
    match name {
        "Diagonals I" => (6, -1),
        "Beams I" => (7, -1),
        "Shields I" => (8, -1),
        "Rite I" => (10, -1),
        "Rite II" => (10, 0),
        "Rite III" => (11, 0),
        "Rite IV" => (12, 0),
        "Ascension I" => (13, 0),
        "Ascension II" => (14, 0),
        "Diagonals II" => (6, 0),
        "Diagonals III" => (7, 0),
        "Diagonals IV" => (7, -2),
        "Beams II" => (7, -3),
        "Beams III" => (8, -3),
        "Challenge II" => (8, -2),
        "Challenge I" => (9, 0),
        "Challenge III" => (9, -2),
        "Shields II" => (8, 0),
        "Shields III" => (8, 1),
        "Challenge IV" => (9, 1),
        _ => original,
    }
}
