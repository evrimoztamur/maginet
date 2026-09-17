use shared::*;

fn original() -> Vec<CampaignEntry> {
    let value: serde_json::Value = serde_json::from_str(include_str!(
        "../../assessments/campaign-challenge/original-catalogue.json"
    ))
    .unwrap();
    serde_json::from_value(value["catalogue"].clone()).unwrap()
}

#[test]
fn later_revisions_preserve_identity_unlocks_and_every_previous_star() {
    let old = original();
    let current = campaign_catalogue(false);
    assert_eq!(old.len(), current.len());
    assert_eq!(
        serde_json::to_value(campaign_connections(&old)).unwrap(),
        serde_json::to_value(campaign_connections(&current)).unwrap()
    );
    for (previous, revised) in old.iter().zip(&current) {
        let mut metadata = serde_json::to_value(revised).unwrap();
        metadata["code"] = serde_json::json!(previous.code);
        assert_eq!(
            metadata,
            serde_json::to_value(previous).unwrap(),
            "{} identity",
            previous.id
        );
        let old_code = previous.level().as_code();
        let new_code = revised.level().as_code();
        if old_code != new_code {
            let aliases = campaign_progress_aliases(&new_code);
            assert!(
                aliases.contains(&old_code.as_str()),
                "{} previous star",
                previous.id
            );
            for legacy in campaign_progress_aliases(&old_code) {
                assert!(aliases.contains(legacy), "{} oldest star", previous.id);
            }
        }
        if previous.tutorial
            || previous.chaos
            || ["basics-", "patterns-", "diagonals-"]
                .iter()
                .any(|p| previous.id.starts_with(p))
        {
            assert_eq!(previous.code, revised.code, "{} is preserved", previous.id);
        }
    }
}

#[test]
fn shields_introduction_requires_the_detour_and_then_the_rune() {
    let entry = campaign_catalogue(false)
        .into_iter()
        .find(|e| e.id == "shields-i")
        .unwrap();
    let mut game = Game::new(&entry.level(), true).unwrap();
    let opening = game.search(SearchLimits::depth(10), 1, || false);
    assert_eq!(opening.moves[0].score, 99999);
    assert_eq!(
        opening.best_move(),
        Some(Turn(Position(1, 2), Position(0, 2)))
    );
    assert!(opening.moves.iter().skip(1).all(|m| m.score == -99999));
    game.take_move(Position(1, 2), Position(0, 2)).unwrap();
    for reply in game.legal_turns() {
        let mut next = game.clone();
        next.take_move(reply.0, reply.1).unwrap();
        if next.result().is_none() {
            let result = next.search(SearchLimits::depth(10), 1, || false);
            assert_eq!(
                result.moves[0].score, 99999,
                "the detour withstands every reply"
            );
        }
    }
}

#[test]
fn catalogue_matches_the_reviewed_finalists() {
    let candidates: Vec<serde_json::Value> = serde_json::from_str(include_str!(
        "../../assessments/campaign-challenge/candidates.json"
    ))
    .unwrap();
    let selected: Vec<_> = candidates
        .iter()
        .filter(|e| e["selected"] == true)
        .collect();
    assert_eq!(selected.len(), 18);
    let catalogue = campaign_catalogue(false);
    for candidate in selected {
        let id = candidate["battle"].as_str().unwrap();
        let entry = catalogue.iter().find(|e| e.id == id).unwrap();
        assert_eq!(
            entry.level().as_code(),
            candidate["code"].as_str().unwrap(),
            "{id}"
        );
    }
}
