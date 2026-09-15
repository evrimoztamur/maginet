use std::time::Duration;

use shared::{Lobby, LobbySettings, LobbySort, Team};

#[test]
fn settings_default_and_round_trip() {
    let old = r#"{"lobby_sort":"Local","loadout_method":"Default","seed":0,"can_stalemate":true}"#;
    assert_eq!(
        serde_json::from_str::<LobbySettings>(old)
            .unwrap()
            .player_team,
        Team::Red
    );
    assert_eq!(LobbySettings::default().player_team, Team::Red);
    for player_team in [Team::Red, Team::Blue] {
        let settings = LobbySettings {
            player_team,
            ..Default::default()
        };
        let encoded = serde_json::to_string(&settings).unwrap();
        assert_eq!(
            serde_json::from_str::<LobbySettings>(&encoded)
                .unwrap()
                .player_team,
            player_team
        );
    }
}

#[test]
fn local_and_ai_assignments_survive_undo_and_rematch() {
    for player_team in [Team::Red, Team::Blue] {
        for lobby_sort in [LobbySort::Local, LobbySort::LocalAI] {
            let mut lobby = Lobby::new(
                LobbySettings {
                    player_team,
                    lobby_sort: lobby_sort.clone(),
                    ..Default::default()
                },
                Duration::ZERO,
            );
            assert_eq!(lobby.game.turn_for(), Team::Red);
            assert_eq!(lobby.player_team(None), Some(player_team));
            assert_eq!(
                lobby.is_active_player(None),
                lobby_sort == LobbySort::Local || player_team == Team::Red
            );
            let turn = lobby.game.legal_turns()[0];
            lobby.game.take_move(turn.0, turn.1).unwrap();
            assert_eq!(lobby.game.turn_for(), Team::Blue);
            assert_eq!(
                lobby.is_active_player(None),
                lobby_sort == LobbySort::Local || player_team == Team::Blue
            );
            lobby.rewind(1);
            assert_eq!(lobby.player_team(None), Some(player_team));
            lobby.remake(Duration::from_secs(1));
            assert_eq!(lobby.player_team(None), Some(player_team));
            assert_eq!(lobby.game.turn_for(), Team::Red);
        }
    }
}

#[cfg(feature = "server")]
#[test]
fn online_assignments_and_rematches() {
    for player_team in [Team::Red, Team::Blue] {
        let mut lobby = Lobby::new(
            LobbySettings {
                player_team,
                lobby_sort: LobbySort::Online(1),
                ..Default::default()
            },
            Duration::ZERO,
        );
        let host = "host".to_string();
        let guest = "guest".to_string();
        lobby.join_player(host.clone()).unwrap();
        lobby.join_player(guest.clone()).unwrap();
        for _ in 0..3 {
            assert!(lobby.all_ready());
            assert_eq!(lobby.player_team(Some(&host)), Some(player_team));
            assert_eq!(lobby.player_team(Some(&guest)), Some(player_team.enemy()));
            assert_eq!(
                lobby.is_active_player(Some(&host)),
                player_team == Team::Red
            );
            assert_eq!(
                lobby.is_active_player(Some(&guest)),
                player_team == Team::Blue
            );
            assert_eq!(lobby.game.turn_for(), Team::Red);
            assert!(!lobby.request_rematch(guest.clone()).unwrap());
            assert!(lobby.request_rematch(host.clone()).unwrap());
            lobby.remake(Duration::from_secs(1));
            assert!(lobby.players().values().all(|player| !player.rematch));
        }
    }
}
