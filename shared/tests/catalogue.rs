use shared::*;
#[test]
fn catalogue_preserves_codes_membership_and_reachability() {
    let full = campaign_catalogue(false);
    let demo = campaign_catalogue(true);
    assert_eq!(full.len(), 36);
    assert_eq!(demo.len(), 5);
    assert_eq!(full.iter().filter(|e| e.tutorial).count(), 1);
    let mut reached = std::collections::HashSet::from(["tutorial".to_string()]);
    for _ in 0..full.len() {
        for e in &full {
            if reached
                .iter()
                .any(|p| campaign_connected(&campaign_connections(&full), p, &e.id))
            {
                reached.insert(e.id.clone());
            }
        }
    }
    for e in &full {
        assert_eq!(e.level().as_code(), Level::from(e.code.as_str()).as_code());
        assert!(reached.contains(&e.id));
    }
    assert_ne!(
        full.iter().find(|e| e.name == "Rite III").unwrap().code,
        full.iter().find(|e| e.name == "Rite IV").unwrap().code
    );
}
#[test]
fn malformed_codes_are_fallible() {
    for code in ["", "!", "00", "00000", "zzzzz", "HG12G014CM0J800"] {
        assert!(Level::parse_code(code).is_err(), "{code}");
    }
    let valid = campaign_catalogue(false)[0].code.clone();
    for end in 0..valid.len() {
        assert!(Level::parse_code(&valid[..end]).is_err());
    }
    let bytes = Vec::<u8>::from(&campaign_catalogue(false)[0].level());
    for (index, value) in [(0, 0), (0, 255), (1, 255), (2, 255), (3, 255), (4, 240)] {
        let mut bad = bytes.clone();
        bad[index] = value;
        assert!(Level::parse_code(&BASE32.encode(&bad)).is_err());
    }
}

#[test]
fn teaching_prerequisites_are_cut_vertices() {
    use std::collections::HashSet;
    let entries = campaign_catalogue(false);
    let edges = campaign_connections(&entries);
    assert_eq!(edges.len(), 40);
    assert_eq!(
        entries.iter().map(|e| &e.id).collect::<HashSet<_>>().len(),
        36
    );
    assert_eq!(
        entries
            .iter()
            .map(|e| e.position)
            .collect::<HashSet<_>>()
            .len(),
        36
    );
    assert_eq!(
        entries
            .iter()
            .map(|e| e.level().as_code())
            .collect::<HashSet<_>>()
            .len(),
        36
    );
    let reach = |removed: &[&str]| {
        let mut reached = HashSet::from(["tutorial".to_string()]);
        for _ in 0..entries.len() {
            for e in &entries {
                if !removed.contains(&e.id.as_str())
                    && reached
                        .iter()
                        .any(|id| campaign_connected(&edges, id, &e.id))
                {
                    reached.insert(e.id.clone());
                }
            }
        }
        reached
    };
    for required in MAIN_ROUTE.iter().filter(|id| {
        ![
            "tutorial",
            "junction-i",
            "rite-i",
            "rite-ii",
            "rite-iii",
            "ascension-ii",
        ]
        .contains(id)
    }) {
        assert!(
            !reach(&[required]).contains("ascension-ii"),
            "bypassed {required}"
        );
    }
    assert!(reach(&[
        "challenge-i",
        "challenge-ii",
        "challenge-iii",
        "challenge-iv"
    ])
    .contains("ascension-ii"));
    assert!(reach(&["rite-i", "rite-ii", "rite-iii"]).contains("ascension-ii"));
    for branch in OPTIONAL_ROUTES {
        let pair = &branch[branch.len() - 2..];
        assert!(campaign_connected(&edges, pair[0], pair[1]));
        assert!(!campaign_connected(&edges, pair[1], pair[0]));
    }
    assert!(!campaign_connected(&edges, "patterns-iii", "diagonals-ii"));
    let demo = campaign_catalogue(true);
    assert_eq!(campaign_connections(&demo).len(), 4);
}

#[test]
fn introductions_are_focused_and_capstone_combines_mechanics() {
    let entries = campaign_catalogue(false);
    for (name, kind) in [
        ("Diagonals I", PowerUp::Diagonal),
        ("Beams I", PowerUp::Beam),
        ("Shields I", PowerUp::Shield),
    ] {
        let level = entries.iter().find(|e| e.name == name).unwrap().level();
        assert!(!level.powerups.is_empty());
        assert!(level.powerups.values().all(|p| *p == kind));
        let game = Game::new(&level, true).unwrap();
        assert!(game
            .legal_turns()
            .iter()
            .any(|t| level.powerups.get(&t.1) == Some(&kind)));
    }
    let level = entries
        .iter()
        .find(|e| e.name == "Rite IV")
        .unwrap()
        .level();
    assert_eq!(
        level.mages.iter().filter(|m| m.team == Team::Red).count(),
        3
    );
    assert_eq!(
        level.mages.iter().filter(|m| m.team == Team::Blue).count(),
        3
    );
    for p in [PowerUp::Diagonal, PowerUp::Beam, PowerUp::Shield] {
        assert!(level.powerups.values().any(|v| *v == p));
    }
}

#[test]
fn every_connection_is_a_cardinal_neighbour_and_junctions_are_unique_duels() {
    let entries = campaign_catalogue(false);
    for edge in campaign_connections(&entries) {
        let a = entries.iter().find(|e| e.id == edge.from).unwrap().position;
        let b = entries.iter().find(|e| e.id == edge.to).unwrap().position;
        assert_eq!(
            (a.0 - b.0).abs() + (a.1 - b.1).abs(),
            1,
            "{} -> {}",
            edge.from,
            edge.to
        );
    }
    let junctions: Vec<_> = entries
        .iter()
        .filter(|e| e.id.starts_with("junction-"))
        .collect();
    assert_eq!(junctions.len(), 8);
    for entry in junctions {
        let level = entry.level();
        assert_eq!(level.mages.len(), 2);
        assert_eq!(
            level.mages.iter().filter(|m| m.team == Team::Red).count(),
            1
        );
        assert_eq!(
            level.mages.iter().filter(|m| m.team == Team::Blue).count(),
            1
        );
        assert!(!entry.demo);
    }
    let expected: &[&[&str]] = &[
        &["diagonals-i", "diagonals-ii", "diagonals-iii", "beams-i"],
        &[
            "beams-i",
            "diagonals-iv",
            "beams-ii",
            "beams-iii",
            "challenge-ii",
            "shields-i",
        ],
        &["shields-i", "challenge-i", "rite-ii"],
        &["shields-i", "challenge-iii", "rite-iv"],
        &[
            "shields-i",
            "shields-ii",
            "shields-iii",
            "challenge-iv",
            "rite-iv",
        ],
    ];
    for (route, expected) in OPTIONAL_ROUTES.iter().zip(expected) {
        assert_eq!(
            route
                .iter()
                .copied()
                .filter(|id| !id.starts_with("junction-"))
                .collect::<Vec<_>>(),
            *expected
        );
    }
}
