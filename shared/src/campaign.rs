//! Stable campaign metadata shared by the UI and native analysis.
use serde::{Deserialize, Serialize};

use crate::{BoardStyle, Level};
/// Tutorial saved-progress code.
pub const TUTORIAL_CODE: &str = "hg18a09m4g0m81g00c4068035g14r0v008";
/// Tutorial map position.
pub const TUTORIAL_POSITION: (isize, isize) = (0, 1);
/// One portal; codes need not be unique (Rite III and IV share progress).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignEntry {
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
        3..=4 => BoardStyle::Desert,
        5 => BoardStyle::Flesh,
        6..=7 => BoardStyle::Crust,
        _ => BoardStyle::Eldritch,
    }
}
/// Full catalogue in stable order, optionally filtered to demo portals.
pub fn campaign_catalogue(demo: bool) -> Vec<CampaignEntry> {
    let definitions = [
        ("Basics I", "hg12g014cm0j800", (0, 0), true, false),
        ("Basics II", "e01jg1148m0j8k834g00", (1, 0), true, false),
        ("Basics III", "j0228014cm0j8v804gp04900", (2, 0), true, false),
        ("Basics IV", "j0228014cm0j8v804gp04906201g00s80dm07403g01g", (2, -1), true, false),
        ("Patterns I", "pg32a0j4gm148t818h602h1g092900j409r06h03", (3, -1), false, false),
        ("Patterns II", "pg2620a48m1m8c038ht02h04gg1jr0wg0d406", (4, -1), false, false),
        ("Patterns III", "pg3220j4g41m8h818gr06h4g052780j400", (5, -1), false, false),
        ("Diagonals I", "h0120124d42480t40e204102", (4, 0), false, false),
        ("Diagonals II", "f02220t4840m8e018hc06h04a014g0sg0cm04", (4, 1), false, false),
        ("Diagonals III", "bg3200240g248h038gcg6h2s0h23t02408r04b02", (4, 2), false, false),
        ("Diagonals IV", "k036202444148h818ha02h1r0127g0j40m604k01dg1jr0wc08", (4, 3), false, false),
        ("Beams I", "j02620t4441m8c038hr06h055g1g00wg0dj06j01", (5, -2), false, false),
        ("Beams II", "eg3020t4c40489818gr02h0m0d2780240gp06a03d00pr08", (5, -3), false, false),
        ("Beams III", "qg22j0t4h41m8d038ja06h04gg13g0mr04j02", (6, -3), false, false),
        ("Shields I", "x01420a4900m81a402204903rg1680r", (5, 2), false, false),
        ("Shields II", "xg2420a4r40m9b018gp02h06x00080140f406t02gg10", (6, 2), false, false),
        ("Shields III", "j02280j4500m8t818hpg4h025g06800", (7, 2), false, false),
        ("Challenge I", "hg2280a4d40490008g6g2h02cg12g00", (2, 1), false, false),
        ("Challenge II", "hg2680t44m048a028hmg2h04000gr0mc06004", (6, -1), false, false),
        ("Challenge III", "q03220t4840m98828gw02h2r0d2bg0j40x804j03dg0k00s80a807200", (7, -3), false, false),
        ("Challenge IV", "qg3200t4000m90048jeg6h5x0523t1241e606c03701s00wm02206j015g1k80h802404", (7, 1), false, false),
        ("Rite I", "t04420a4041m90818k0g6h2g052900a4t01m84038g2tr0n80cm06902d00g", (7, -2), false, false),
        ("Rite II", "zg2220t4r4048f008ke06h0chg1pr0wr0b406w03j01qg0340ba06d03gg02g0r", (7, -1), false, false),
        ("Rite III", "pg3820a44m2482808jp00h4g0h2380a410r04000m01r80nm00a06a03hg1g", (7, 0), false, false),
        ("Rite IV", "pg3820a44m2482808jp00h4g0h2380a410r04000m01r80nm00a06a03hg1g", (8, 0), false, false),
        ("Ascension I", "zg322024w42499828hw04h6w0h25r02410t05j02n01j80vg0et00k01v01g", (8, -1), false, false),
        ("Ascension II", "zg4200t4000m90048kg00h4x0d2bt0a47m249z808g78r0sg0cw07403jg0f80w40d403m025g1k80h802405b03", (9, -1), false, false),
        ("Tutorial", TUTORIAL_CODE, TUTORIAL_POSITION, true, true),
    ];
    definitions
        .into_iter()
        .filter(|e| !demo || e.3)
        .map(|(name, code, position, demo, tutorial)| CampaignEntry {
            name: name.into(),
            code: code.into(),
            position,
            style: campaign_style(position.0),
            demo,
            tutorial,
        })
        .collect()
}
