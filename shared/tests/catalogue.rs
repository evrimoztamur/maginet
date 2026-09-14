use shared::*;
#[test]
fn catalogue_preserves_codes_membership_and_reachability() {
    let full = campaign_catalogue(false);
    let demo = campaign_catalogue(true);
    assert_eq!(full.len(), 28);
    assert_eq!(demo.len(), 5);
    assert_eq!(full.iter().filter(|e| e.tutorial).count(), 1);
    let mut reached = std::collections::HashSet::from([TUTORIAL_POSITION]);
    for _ in 0..full.len() {
        for e in &full {
            if reached
                .iter()
                .any(|p| (p.0 - e.position.0).abs() + (p.1 - e.position.1).abs() == 1)
            {
                reached.insert(e.position);
            }
        }
    }
    for e in &full {
        assert_eq!(e.level().as_code(), Level::from(e.code.as_str()).as_code());
        assert!(reached.contains(&e.position));
    }
    assert_eq!(
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
