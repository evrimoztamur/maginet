//! Stable campaign metadata shared by the UI and native analysis.
use serde::{Deserialize, Serialize};

use crate::{BoardStyle, Level};
/// Tutorial saved-progress code.
pub const TUTORIAL_CODE: &str = "hg18a09m4g0m81000c4068039g1g";

/// Earlier versions of these battles share their completion stars with the current puzzle.
pub fn campaign_progress_aliases(code: &str) -> &'static [&'static str] {
    match code {
        "dg108094d40j409408" => &["dg1080a4d40j409408", "d010812ncm23809408"],
        TUTORIAL_CODE => &["hg18a09m4g0m81g00c4068035g14r0v008"],
        "e01jg1148m0j8k834g00" => &["e01jg1248m0j8k834g00"],
        "j022800mcm0j8v804gp04900" => &["j0228014cm0j8v804gp04900"],
        "dg2gr0145g118g842hjg250100j00" => &[
            "j022801mcm0j8v804gp04d06001h00s80dm07003j01g",
            "j022801mcm0j8v804gp04d06201g00s80dm07403g01g",
        ],
        "dg30r0j4500k8v048g0g4h250526212400" => &["dg30r0j4500m8v048g0g4h250526212400"],
        "pg3220j4g41m8h818gr06d4g052780j400" => &["pg3220j4g41m8h818gr06h4g052780j400"],
        "d012g0s4841j808800" => &["d012g0an840k80h401200"],
        "dg30r09m5g1m8v048g0g2h210d2621240gj02a028g14g00" => &[
            "dg30r0a45g1m8v048g0g2h210d2621240gj02a028g14g00",
            "dg30r0a45g1m8v048g0g2h210d2621240gm04h024g0mg00",
        ],
        _ => &[],
    }
}

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
    /// Invisible until a connected neighbour is completed.
    #[serde(default)]
    pub hidden: bool,
    /// Fresh independent Chaos teams on every entry and rematch.
    #[serde(default)]
    pub chaos: bool,
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
/// Region theme follows the battle, independently of bends in the map.
fn campaign_style(name: &str) -> BoardStyle {
    if name == "Tutorial" || name.starts_with("Basics ") || name.starts_with("Ascension ") {
        BoardStyle::Grass
    } else if name.starts_with("Patterns ") || name.starts_with("Diagonals ") || name == "Side Step"
    {
        BoardStyle::Desert
    } else if name.starts_with("Beams ") || name == "Crossfire" {
        BoardStyle::Flesh
    } else if name.starts_with("Shields ") || matches!(name, "Rite I" | "Rite II") {
        BoardStyle::Crust
    } else {
        BoardStyle::Eldritch
    }
}
/// Full catalogue in stable order, optionally filtered to demo portals.
pub fn campaign_catalogue(demo: bool) -> Vec<CampaignEntry> {
    let definitions = [
        ("basics-i", "Basics I", "e01jg1148m0j8k834g00", (0, 0), true, false),
        ("basics-ii", "Basics II", "j022800mcm0j8v804gp04900", (1, 0), true, false),
        ("basics-iii", "Basics III", "dg2gr0145g118g842hjg250100j00", (2, 0), true, false),
        ("patterns-i", "Patterns I", "dg30r0j4500k8v048g0g4h250526212400", (3, -1), false, false),
        ("patterns-ii", "Patterns II", "pg2620a48m1m8c038ht02h04gg1jr0wg0d406", (4, -1), false, false),
        ("patterns-iii", "Patterns III", "pg3220j4g41m8h818gr06d4g052780j400", (5, -1), false, false),
        ("diagonals-i", "Diagonals I", "dg108094d40j409408", (4, 0), false, false),
        ("diagonals-ii", "Diagonals II", "f02220t4840m8e018hc06h04a014g0sg0cm04", (4, 1), false, false),
        ("diagonals-iii", "Diagonals III", "bg3200240g248h038gcg6h2s0h23t02408r04b02", (4, 2), false, false),
        ("diagonals-iv", "Diagonals IV", "k036202444148h818ha02h1r0127g0j40m604k01dg1jr0wc08", (4, 3), false, false),
        ("beams-i", "Beams I", "hg22r024dg0m81816j2g0d02500pg08", (5, -2), false, false),
        ("beams-ii", "Beams II", "eg3020t4c40489818gr02h0m0d2780240gp06a03d00pr08", (5, -3), false, false),
        ("beams-iii", "Beams III", "qg22j0t4h41m8d038ja06h04gg13g0mr04j02", (6, -3), false, false),
        ("shields-i", "Shields I", "d012g0s4841j808800", (5, 2), false, false),
        ("shields-ii", "Shields II", "xg2420a4r40m9b018gp02h06x00080140f406t02gg10", (6, 2), false, false),
        ("shields-iii", "Shields III", "j02280j4500m8t818hpg4h025g06800", (7, 2), false, false),
        ("challenge-i", "Challenge I", "hg2280a4d40490008g6g2h02cg12g00", (2, 1), false, false),
        ("challenge-ii", "Challenge II", "hg2680t44m048a028hmg2h04000gr0mc06004", (6, -1), false, false),
        ("challenge-iii", "Challenge III", "q03220t4840m98828gw02h2r0d2bg0j40x804j03dg0k00s80a807200", (7, -3), false, false),
        ("challenge-iv", "Challenge IV", "qg3200t4000m90048jeg6h5x0523t1241e606c03701s00wm02206j015g1k80h802404", (7, 1), false, false),
        ("rite-i", "Rite I", "t04420a4041m90818k0g6h2g052900a4t01m84038g2tr0n80cm06902d00g", (7, -2), false, false),
        ("rite-ii", "Rite II", "dg20r124dg1m80838hgg8h03501480h404", (7, -1), false, false),
        ("rite-iii", "Rite III", "pg3820a44m2482808jp00h4g0h2380a410r04000m01r80nm00a06a03hg1g", (7, 0), false, false),
        ("rite-iv", "Rite IV", "dg30r09m5g1m8v048g0g2h210d2621240gj02a028g14g00", (8, 0), false, false),
        ("ascension-i", "Ascension I", "zg322024w42499828hw04h6w0h25r02410t05j02n01j80vg0et00k01v01g", (8, -1), false, false),
        ("ascension-ii", "Ascension II", "zg4200t4000m90048kg00h4x0d2bt0a47m249z808g78r0sg0cw07403jg0f80w40d403m025g1k80h802405b03", (9, -1), false, false),
        ("crossfire", "Crossfire", "dg1g8092cm112b82240j808", (7, -4), false, false),
        ("side-step", "Side Step", "dg10008h4m11209008", (6, 0), false, false),
        ("ascension-iii", "Ascension III", "dg400024401m8g028hg02h3d0124t0t45m1483818g00", (12, -4), false, false),
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
            style: campaign_style(name),
            hidden: matches!(id, "crossfire" | "side-step") || id.starts_with("ascension-"),
            chaos: id == "ascension-iii",
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
    "patterns-i",
    "patterns-ii",
    "patterns-iii",
    "diagonals-i",
    "beams-i",
    "shields-i",
    "rite-i",
    "rite-ii",
    "rite-iii",
    "rite-iv",
    "ascension-i",
    "ascension-ii",
];
/// Optional practice paths and the post-capstone challenge series.
/// Includes hidden puzzle connections and the optional Chaos epilogue.
pub const OPTIONAL_ROUTES: &[&[&str]] = &[
    &[
        "diagonals-i",
        "diagonals-ii",
        "diagonals-iii",
        "diagonals-iv",
    ],
    &["beams-i", "beams-ii", "beams-iii"],
    &["beams-iii", "crossfire", "challenge-i"],
    &["diagonals-iii", "side-step", "shields-iii"],
    &["ascension-ii", "ascension-iii"],
    &["shields-i", "shields-ii", "shields-iii"],
    &[
        "rite-iv",
        "challenge-i",
        "challenge-ii",
        "challenge-iii",
        "challenge-iv",
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
                    one_way: branch && i == route.len() - 2 && MAIN_ROUTE.contains(&pair[1])
                        || pair == ["rite-iv", "challenge-i"],
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
// Practice paths stay beside their introductions; the challenge trail forks
// away from Ascension after the Rite IV capstone. No battles exist only as paving.
fn portal_position(name: &str, original: (isize, isize)) -> (isize, isize) {
    let position = match name {
        "Patterns I" => (3, -1),
        "Patterns II" => (3, -2),
        "Patterns III" => (4, -2),
        "Diagonals I" => (5, -2),
        "Beams I" => (6, -2),
        "Shields I" => (7, -2),
        "Rite I" => (8, -2),
        "Rite II" => (9, -2),
        "Rite III" => (9, -3),
        "Rite IV" => (9, -4),
        "Ascension I" => (10, -4),
        "Ascension II" => (11, -4),
        "Diagonals II" => (5, -1),
        "Diagonals III" => (5, 0),
        "Diagonals IV" => (4, 0),
        "Beams II" => (6, -3),
        "Beams III" => (6, -4),
        "Shields II" => (7, -1),
        "Shields III" => (7, 0),
        "Challenge I" => (8, -4),
        "Challenge II" => (8, -5),
        "Challenge III" => (8, -6),
        "Challenge IV" => (9, -6),
        _ => original,
    };
    // Three Basics battles lead directly east into Patterns. Shift the entire
    // later region together to preserve every cardinal connection and branch.
    if name == "Tutorial" || name.starts_with("Basics ") {
        position
    } else {
        (position.0, position.1 + 1)
    }
}
