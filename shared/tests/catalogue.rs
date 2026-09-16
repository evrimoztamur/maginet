use shared::*;
#[test]
fn catalogue_preserves_codes_membership_and_reachability() {
    let full = campaign_catalogue(false);
    let demo = campaign_catalogue(true);
    assert_eq!(full.len(), 30);
    assert_eq!(demo.len(), 4);
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
    assert_eq!(edges.len(), 31);
    assert_eq!(
        entries.iter().map(|e| &e.id).collect::<HashSet<_>>().len(),
        30
    );
    assert_eq!(
        entries
            .iter()
            .map(|e| e.position)
            .collect::<HashSet<_>>()
            .len(),
        30
    );
    assert_eq!(
        entries
            .iter()
            .map(|e| e.level().as_code())
            .collect::<HashSet<_>>()
            .len(),
        30
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
    for required in MAIN_ROUTE
        .iter()
        .filter(|id| !["tutorial", "ascension-ii", "beams-i"].contains(id))
    {
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
    for required in ["challenge-i", "challenge-ii", "challenge-iii"] {
        assert!(
            !reach(&[required]).contains("challenge-iv"),
            "challenge trail bypassed {required}"
        );
    }
    for branch in OPTIONAL_ROUTES {
        let pair = &branch[branch.len() - 2..];
        assert!(campaign_connected(&edges, pair[0], pair[1]));
        assert_eq!(
            campaign_connected(&edges, pair[1], pair[0]),
            !MAIN_ROUTE.contains(&pair[1])
        );
    }
    assert!(!campaign_connected(&edges, "patterns-iii", "diagonals-ii"));
    let demo = campaign_catalogue(true);
    assert_eq!(campaign_connections(&demo).len(), 3);
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
fn every_connection_is_a_cardinal_neighbour_without_filler_or_accidental_contacts() {
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
    assert!(entries.iter().all(|e| !e.id.starts_with("junction-")));
    assert_eq!(entries.iter().filter(|e| e.hidden).count(), 5);
    assert!(campaign_connected(
        &campaign_connections(&entries),
        "side-step",
        "shields-iii"
    ));
    assert!(campaign_connected(
        &campaign_connections(&entries),
        "shields-iii",
        "side-step"
    ));
    assert!(!campaign_connected(
        &campaign_connections(&entries),
        "challenge-i",
        "rite-iv"
    ));
    for a in &entries {
        for b in &entries {
            if (a.position.0 - b.position.0).abs() + (a.position.1 - b.position.1).abs() == 1 {
                let edges = campaign_connections(&entries);
                assert!(
                    campaign_connected(&edges, &a.id, &b.id)
                        || campaign_connected(&edges, &b.id, &a.id),
                    "unconnected neighbours crowd {} and {}",
                    a.id,
                    b.id
                );
            }
        }
    }
}

#[test]
fn tutorial_teaches_cardinal_movement_and_three_basics_puzzles_follow_it() {
    let entries = shared::campaign_catalogue(false);
    let tutorial = entries.iter().find(|e| e.tutorial).unwrap().level();
    assert_eq!(tutorial.as_code(), shared::TUTORIAL_CODE);
    assert_eq!(tutorial.powerups.len(), 4);
    assert!(tutorial
        .powerups
        .values()
        .all(|p| matches!(p, shared::PowerUp::Boulder(_))));
    assert!(tutorial.mages.iter().all(|m| !m.has_diagonals()));
    for (id, original) in [
        ("basics-i", "e01jg1148m0j8k834g00"),
        ("basics-ii", "j022800mcm0j8v804gp04900"),
        ("basics-iii", "dg2gr0145g118g842hjg250100j00"),
    ] {
        let level = entries.iter().find(|e| e.id == id).unwrap().level();
        assert_eq!(level.as_code(), original);
        assert!(level.powerups.is_empty());
    }
    assert!(!entries.iter().any(|entry| entry.id == "basics-iv"));
    assert!(campaign_connected(
        &campaign_connections(&entries),
        "basics-iii",
        "patterns-i"
    ));
    assert_eq!(
        shared::campaign_progress_aliases(shared::TUTORIAL_CODE),
        &["hg18a09m4g0m81g00c4068035g14r0v008"]
    );
    assert_eq!(
        shared::campaign_progress_aliases("e01jg1148m0j8k834g00"),
        &["e01jg1248m0j8k834g00"]
    );
    assert!(shared::campaign_progress_aliases("onscreen_controls").is_empty());
}
