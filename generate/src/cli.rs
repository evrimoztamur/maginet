use std::{
    collections::VecDeque,
    fs,
    path::{Path, PathBuf},
};

use generate::analysis::*;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use shared::{campaign_catalogue, CampaignEntry, Level};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const HELP: &str = "Scenario analyser (simulated agents, not validated human difficulty)
Usage: generate campaign [--demo] [OPTIONS]
       generate analyse --code CODE [OPTIONS]
       generate generate
OPTIONS: --games N (30) --seed N (1) --max-plies N (200)
         --easy-nodes N (1000) --normal-nodes N (5000) --hard-nodes N (20000)
         --red-profile easy|normal|hard --blue-profile easy|normal|hard
         --seed-namespace TEXT --replays
         --workers N (at most 4 by default) --output DIR (analysis-output)
By default all skill matchups run; tutorial opponents are Easy only, without stalemates.
Completed matchups resume automatically only with identical configuration/catalogue.
No arguments or --help prints this help.";
#[derive(Serialize, Deserialize)]
struct Metadata {
    config: RunConfig,
    catalogue: Vec<CampaignEntry>,
    graph: Vec<shared::CampaignConnection>,
    main_route: Vec<String>,
    engine: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Matchup {
    level: usize,
    red: usize,
    blue: usize,
    games: Vec<GameResultRecord>,
}
#[derive(Serialize)]
struct Summary {
    level: usize,
    red: usize,
    blue: usize,
    stats: Aggregate,
}
fn write_json(path: &Path, value: &impl Serialize) -> Result<()> {
    let temporary = path.with_extension("tmp");
    fs::write(&temporary, serde_json::to_vec_pretty(value)?)?;
    fs::rename(temporary, path)?;
    Ok(())
}
pub fn run() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let Some(command) = args.next() else {
        println!("{HELP}");
        return Ok(());
    };
    if command == "--help" || command == "-h" {
        println!("{HELP}");
        return Ok(());
    }
    if command == "generate" {
        if args.next().is_some() {
            return Err("generate takes no options".into());
        }
        crate::generate();
        return Ok(());
    }
    if command != "campaign" && command != "analyse" {
        return Err(format!("unknown command {command}; use --help").into());
    }
    let mut config = RunConfig::default();
    let mut demo = false;
    let mut code = None;
    let mut workers = std::thread::available_parallelism().map_or(1, |n| n.get().min(4));
    let mut output = PathBuf::from("analysis-output");
    while let Some(flag) = args.next() {
        if flag == "--help" {
            println!("{HELP}");
            return Ok(());
        }
        if flag == "--replays" {
            config.replays = true;
            continue;
        }
        if flag == "--demo" {
            demo = true;
            continue;
        }
        if ![
            "--seed-namespace",
            "--red-profile",
            "--blue-profile",
            "--code",
            "--output",
            "--games",
            "--seed",
            "--max-plies",
            "--workers",
            "--easy-nodes",
            "--normal-nodes",
            "--hard-nodes",
        ]
        .contains(&flag.as_str())
        {
            return Err(format!("unknown option {flag}; use --help").into());
        }
        let value = args
            .next()
            .ok_or_else(|| format!("missing value for {flag}"))?;
        let number = || {
            value
                .parse::<u64>()
                .map_err(|_| format!("{flag} requires a nonnegative integer"))
        };
        match flag.as_str() {
            "--seed-namespace" => config.seed_namespace = Some(value),
            "--red-profile" | "--blue-profile" => {
                let profile = match value.as_str() {
                    "easy" => 0,
                    "normal" => 1,
                    "hard" => 2,
                    _ => return Err("profile must be easy, normal, or hard".into()),
                };
                if flag == "--red-profile" {
                    config.red_profile = Some(profile);
                } else {
                    config.blue_profile = Some(profile);
                }
            }
            "--code" => code = Some(value),
            "--output" => output = value.into(),
            "--games" => config.games = usize::try_from(number()?)?,
            "--seed" => config.seed = number()?,
            "--max-plies" => config.max_plies = usize::try_from(number()?)?,
            "--workers" => workers = usize::try_from(number()?)?,
            "--easy-nodes" => config.profiles[0].nodes = number()?,
            "--normal-nodes" => config.profiles[1].nodes = number()?,
            "--hard-nodes" => config.profiles[2].nodes = number()?,
            _ => unreachable!(),
        }
    }
    if config.games == 0 || workers == 0 {
        return Err("--games and --workers must be positive".into());
    }
    let catalogue = if command == "campaign" {
        if code.is_some() {
            return Err("--code belongs to analyse".into());
        }
        campaign_catalogue(demo)
    } else {
        if demo {
            return Err("--demo belongs to campaign".into());
        }
        let level = Level::parse_code(&code.ok_or("analyse requires --code CODE")?)?;
        let canonical = level.as_code();
        let known = campaign_catalogue(false)
            .into_iter()
            .find(|e| e.level().as_code() == canonical);
        vec![known.unwrap_or(CampaignEntry {
            id: "custom".into(),
            name: "Custom scenario".into(),
            code: canonical,
            position: (0, 0),
            style: level.board.style,
            demo: false,
            tutorial: false,
            hidden: false,
            chaos: false,
        })]
    };
    // Bind resumes to the actual rules/search/simulator sources, not an incidental git HEAD.
    let engine = [
        include_str!("../../shared/src/logic/game.rs"),
        include_str!("../../shared/src/logic/deadlock.rs"),
        include_str!("../../shared/src/logic/search.rs"),
        include_str!("analysis.rs"),
        include_str!("../../shared/src/logic/mage.rs"),
        include_str!("../../shared/src/logic/powerup.rs"),
        include_str!("../../shared/src/logic/level.rs"),
        include_str!("../../shared/src/logic/board.rs"),
        include_str!("../../shared/src/logic/spell.rs"),
        include_str!("../../shared/src/logic/mana.rs"),
        include_str!("../../shared/src/logic/position.rs"),
        include_str!("../../shared/src/logic/team.rs"),
        include_str!("../../shared/src/logic/turn.rs"),
    ]
    .join("\n");
    let metadata = Metadata {
        graph: shared::campaign_connections(&catalogue),
        main_route: shared::MAIN_ROUTE.iter().map(|s| s.to_string()).collect(),
        config,
        catalogue,
        engine: format!("fnv1a-{:016x}", trial_seed(0, &engine, 0)),
    };
    execute(&output, workers, metadata)
}
fn execute(output: &Path, workers: usize, metadata: Metadata) -> Result<()> {
    fs::create_dir_all(output)?;
    let path = output.join("metadata.json");
    if path.exists() {
        let old: serde_json::Value = serde_json::from_slice(&fs::read(&path)?)?;
        if old != serde_json::to_value(&metadata)? {
            return Err(
                "resume configuration/catalogue/engine mismatch; use a new --output directory"
                    .into(),
            );
        }
    } else {
        if fs::read_dir(output)?
            .any(|entry| entry.map_or(true, |e| e.path().extension().is_some_and(|x| x == "json")))
        {
            return Err(
                "output contains data without metadata; use a new --output directory".into(),
            );
        }
        write_json(&path, &metadata)?;
    }
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(workers)
        .build()?;
    let mut matchups = Vec::new();
    for (level, entry) in metadata.catalogue.iter().enumerate() {
        if entry.chaos {
            continue;
        } // Random teams are not a fixed scenario assessment.
        let scenario = entry.level();
        for red in 0..3 {
            for blue in 0..if entry.tutorial { 1 } else { 3 } {
                if metadata.config.red_profile.is_some_and(|p| p != red)
                    || metadata.config.blue_profile.is_some_and(|p| p != blue)
                {
                    continue;
                }
                let path = output.join(format!("matchup-{level:02}-{red}-{blue}.json"));
                let matchup = if path.exists() {
                    let saved: Matchup = serde_json::from_slice(&fs::read(&path)?)?;
                    if saved.level != level
                        || saved.red != red
                        || saved.blue != blue
                        || saved.games.len() != metadata.config.games
                        || saved.games.iter().enumerate().any(|(i, g)| {
                            g.trial != i
                                || g.seed != metadata.config.trial_seed(&scenario, i)
                                || (metadata.config.replays && g.replay.len() != g.plies)
                                || g.plies > metadata.config.max_plies
                                || g.red.searches + g.blue.searches != g.plies
                        })
                    {
                        return Err(format!("invalid checkpoint {}", path.display()).into());
                    }
                    saved
                } else {
                    let games = pool.install(|| {
                        (0..metadata.config.games)
                            .into_par_iter()
                            .map(|trial| {
                                simulate(
                                    &scenario,
                                    !entry.tutorial,
                                    &metadata.config,
                                    red,
                                    blue,
                                    trial,
                                )
                            })
                            .collect()
                    });
                    let matchup = Matchup {
                        level,
                        red,
                        blue,
                        games,
                    };
                    write_json(&path, &matchup)?;
                    matchup
                };
                eprintln!(
                    "{}: {} / {} complete",
                    entry.name,
                    metadata.config.profiles[red].difficulty.label(),
                    metadata.config.profiles[blue].difficulty.label()
                );
                matchups.push(matchup);
                reports(output, &metadata, &matchups)?;
            }
        }
    }
    if matchups.is_empty() {
        reports(output, &metadata, &matchups)?;
    }
    Ok(())
}
fn distances(entries: &[CampaignEntry]) -> Vec<usize> {
    let mut distances = vec![usize::MAX; entries.len()];
    let mut queue = VecDeque::new();
    for (i, e) in entries.iter().enumerate() {
        if e.tutorial {
            distances[i] = 0;
            queue.push_back(i);
        }
    }
    while let Some(i) = queue.pop_front() {
        for j in 0..entries.len() {
            if shared::campaign_connected(
                &shared::campaign_connections(entries),
                &entries[i].id,
                &entries[j].id,
            ) && distances[j] == usize::MAX
            {
                distances[j] = distances[i] + 1;
                queue.push_back(j);
            }
        }
    }
    distances
}
fn reports(output: &Path, metadata: &Metadata, matchups: &[Matchup]) -> Result<()> {
    use std::fmt::Write;
    let summaries: Vec<_> = matchups
        .iter()
        .map(|m| Summary {
            level: m.level,
            red: m.red,
            blue: m.blue,
            stats: aggregate(&m.games),
        })
        .collect();
    write_json(&output.join("aggregate.json"), &summaries)?;
    let expected: usize = metadata
        .catalogue
        .iter()
        .map(|e| {
            (0..3)
                .filter(|r| metadata.config.red_profile.is_none_or(|p| p == *r))
                .count()
                * (0..if e.tutorial { 1 } else { 3 })
                    .filter(|b| metadata.config.blue_profile.is_none_or(|p| p == *b))
                    .count()
        })
        .sum();
    let mut md=format!("# Campaign scenario survey\n\nExploratory simulated-agent difficulty; not validated human difficulty. {} / {expected} matchups complete, {} games. Red is the player. Seed {}, {} games per matchup, {}-ply safety limit. Node caps are analysis defaults, not browser time equivalents.\n\nWin rates count draws as resolved non-wins. Cells show resolved Red win % [95% Wilson interval], unresolved count, and possible overall win % range. More than 10% unresolved precludes a definitive ranking. Tutorial uses Easy opponents and disables rule stalemates.\n",summaries.len(),matchups.iter().map(|m|m.games.len()).sum::<usize>(),metadata.config.seed,metadata.config.games,metadata.config.max_plies);
    writeln!(md, "\nEngine `{}`; bounded table capacity {} entries per search.\n\n| Profile | Depth | Nodes/move | Ranked weights |\n|---|---:|---:|---|", metadata.engine, metadata.config.table_capacity)?;
    for p in &metadata.config.profiles {
        writeln!(
            md,
            "| {} | {} | {} | {}/{}/{} |",
            p.difficulty.label(),
            p.depth,
            p.nodes,
            p.weights[0],
            p.weights[1],
            p.weights[2]
        )?;
    }
    let find = |level, red, blue| {
        summaries
            .iter()
            .find(|s| s.level == level && s.red == red && s.blue == blue)
    };
    let distance = distances(&metadata.catalogue);
    for (i, e) in metadata.catalogue.iter().enumerate() {
        if e.chaos {
            writeln!(
                md,
                "\n## {}\n\nRandomized Chaos teams; excluded from fixed-scenario statistics.",
                e.name
            )?;
            continue;
        }
        writeln!(md,"\n## {}\n\nMap {:?}; distance {}.\n\n| Player / opponent | Easy | Normal | Hard |\n|---|---|---|---|",e.name,e.position,if distance[i]==usize::MAX {"n/a".into()} else {distance[i].to_string()})?;
        for red in 0..3 {
            write!(
                md,
                "| {} |",
                metadata.config.profiles[red].difficulty.label()
            )?;
            for blue in 0..3 {
                if let Some(s) = find(i, red, blue) {
                    let a = &s.stats;
                    let cell = match (a.resolved_win_rate, a.wilson95) {
                        (Some(p), Some(ci)) => format!(
                            "{:.1}% [{:.1}, {:.1}]",
                            100.0 * p,
                            100.0 * ci[0],
                            100.0 * ci[1]
                        ),
                        _ => "n/a".into(),
                    };
                    write!(
                        md,
                        " {cell}; n={}; U={}; range {:.1}–{:.1}% |",
                        a.wins + a.losses + a.draws + a.unresolved,
                        a.unresolved,
                        100.0 * a.overall_win_range[0],
                        100.0 * a.overall_win_range[1]
                    )?;
                } else {
                    write!(md, " — |")?;
                }
            }
            writeln!(md)?;
        }
        for s in summaries.iter().filter(|s| s.level == i) {
            let a = &s.stats;
            let searches = a.red.searches + a.blue.searches;
            let saturated =
                a.red.stops.get("Nodes").unwrap_or(&0) + a.blue.stops.get("Nodes").unwrap_or(&0);
            writeln!(md,"\n- {}/{}: W/L/D/U {}/{}/{}/{}; mean {:.1} plies ({}–{}); {} nodes; node saturation {:.1}%; fallbacks {}/{}.",metadata.config.profiles[s.red].difficulty.label(),metadata.config.profiles[s.blue].difficulty.label(),a.wins,a.losses,a.draws,a.unresolved,a.mean_plies,a.min_plies,a.max_plies,a.red.nodes+a.blue.nodes,100.0*saturated as f64/searches.max(1) as f64,a.red.fallbacks+a.blue.fallbacks,searches)?;
        }
    }
    writeln!(md,"\n## Main-route progression candidates\n\nNormal/Normal comparisons in teaching-route order. Directed shortest tutorial distances are context only. A ≥20-point drop is supported only with disjoint intervals and ≤10% unresolved at both ends. These exploratory flags are not corrected for multiple comparisons.\n")?;
    for (route_index, route) in std::iter::once(shared::MAIN_ROUTE)
        .chain(shared::OPTIONAL_ROUTES.iter().copied())
        .enumerate()
    {
        if route_index > 0 {
            writeln!(
                md,
                "\n### Optional route {route_index} ({})\n",
                if shared::MAIN_ROUTE.contains(route.last().unwrap()) {
                    "one-way return"
                } else {
                    "dead end"
                }
            )?;
        }
        for pair in route.windows(2) {
            let Some(i) = metadata.catalogue.iter().position(|e| e.id == pair[0]) else {
                continue;
            };
            let Some(j) = metadata.catalogue.iter().position(|e| e.id == pair[1]) else {
                continue;
            };
            let (a, b) = (&metadata.catalogue[i], &metadata.catalogue[j]);
            if let (Some(sa), Some(sb)) = (find(i, 1, 1), find(j, 1, 1)) {
                if let (Some(pa), Some(pb)) =
                    (sa.stats.resolved_win_rate, sb.stats.resolved_win_rate)
                {
                    {
                        let supported = sa.stats.wilson95.unwrap()[0]
                            > sb.stats.wilson95.unwrap()[1]
                            && sa.stats.unresolved * 10
                                <= sa.stats.wins
                                    + sa.stats.losses
                                    + sa.stats.draws
                                    + sa.stats.unresolved
                            && sb.stats.unresolved * 10
                                <= sb.stats.wins
                                    + sb.stats.losses
                                    + sb.stats.draws
                                    + sb.stats.unresolved;
                        writeln!(
                            md,
                            "- {} → {}: {:.1}-point drop; {}.",
                            a.name,
                            b.name,
                            100.0 * (pa - pb),
                            if pa - pb < 0.2 - 1e-10 {
                                "below spike threshold"
                            } else if supported {
                                "supported candidate"
                            } else {
                                "followup candidate"
                            }
                        )?;
                    }
                }
            }
        }
    }
    writeln!(md,"\n## Sensitivity and followups\n\nThresholds below are exploratory: ≥20 percentage points for sensitivity/reversals, >10% unresolved, ≥50% searches reaching the node cap. Hard is not assumed stronger in every position. Use larger paired-seed runs to confirm these observations.\n")?;
    for (i, e) in metadata.catalogue.iter().enumerate() {
        if let (Some(a), Some(b)) = (find(i, 0, 1), find(i, 2, 1)) {
            if let (Some(pa), Some(pb)) = (a.stats.resolved_win_rate, b.stats.resolved_win_rate) {
                if (pb - pa).abs() >= 0.2 - 1e-10 {
                    writeln!(
                        md,
                        "- {}: player Easy→Hard sensitivity against Normal: {:+.1} points.",
                        e.name,
                        100.0 * (pb - pa)
                    )?;
                }
            }
        }
        for red in 0..3 {
            for blue in 0..2 {
                if let (Some(a), Some(b)) = (find(i, red, blue), find(i, red, blue + 1)) {
                    if let (Some(pa), Some(pb)) =
                        (a.stats.resolved_win_rate, b.stats.resolved_win_rate)
                    {
                        if pb - pa >= 0.2 - 1e-10 {
                            writeln!(md,"- {}: opponent-strength reversal for {} player, {}→{}: {:+.1} Red win points.",e.name,metadata.config.profiles[red].difficulty.label(),metadata.config.profiles[blue].difficulty.label(),metadata.config.profiles[blue+1].difficulty.label(),100.0*(pb-pa))?;
                        }
                    }
                }
            }
        }
        for s in summaries.iter().filter(|s| s.level == i) {
            let a = &s.stats;
            let label = format!(
                "{} {}/{}",
                e.name,
                metadata.config.profiles[s.red].difficulty.label(),
                metadata.config.profiles[s.blue].difficulty.label()
            );
            if a.unresolved * 10 > a.wins + a.losses + a.draws + a.unresolved {
                writeln!(
                    md,
                    "- {label}: frequent unresolved games; extend the safety limit before ranking."
                )?;
            }
            for (side, t) in [("Red", &a.red), ("Blue", &a.blue)] {
                if t.searches > 0 && t.stops.get("Nodes").copied().unwrap_or(0) * 2 >= t.searches {
                    writeln!(md,"- {label}: {side} search-budget saturation ≥50%; compare larger node caps.")?;
                }
            }
        }
    }
    fs::write(output.join("report.md"), md)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn progression_flags_require_disjoint_intervals_and_resolved_games() {
        let root =
            std::env::temp_dir().join(format!("maginet-progression-test-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let all = campaign_catalogue(false);
        let metadata = Metadata {
            config: RunConfig::default(),
            catalogue: vec![all[0].clone(), all[1].clone(), all.last().unwrap().clone()],
            graph: shared::campaign_connections(&campaign_catalogue(false)),
            main_route: shared::MAIN_ROUTE.iter().map(|s| s.to_string()).collect(),
            engine: "test".into(),
        };
        assert_eq!(distances(&metadata.catalogue), vec![1, 2, 0]);
        let games = |wins: usize, unresolved: usize| {
            (0..30)
                .map(|trial| GameResultRecord {
                    overcharge_at: None,
                    overcharge_mages: None,
                    replay: Vec::new(),
                    termination: String::new(),
                    trial,
                    seed: 0,
                    outcome: if trial < wins {
                        Outcome::Win
                    } else if trial >= 30 - unresolved {
                        Outcome::SafetyLimit
                    } else {
                        Outcome::Loss
                    },
                    plies: 0,
                    red: Telemetry::default(),
                    blue: Telemetry::default(),
                })
                .collect()
        };
        for (a, b, u, expected) in [
            (30, 20, 0, "supported candidate"),
            (20, 14, 0, "followup candidate"),
            (30, 0, 5, "followup candidate"),
            (20, 19, 0, "absent"),
        ] {
            reports(
                &root,
                &metadata,
                &[
                    Matchup {
                        level: 0,
                        red: 1,
                        blue: 1,
                        games: games(a, 0),
                    },
                    Matchup {
                        level: 1,
                        red: 1,
                        blue: 1,
                        games: games(b, u),
                    },
                ],
            )
            .unwrap();
            let report = fs::read_to_string(root.join("report.md")).unwrap();
            let flag = report
                .lines()
                .find(|line| line.starts_with("- Basics I → Basics II:"));
            if expected == "absent" {
                assert!(flag.unwrap().contains("below spike threshold"));
            } else {
                assert!(flag.unwrap().contains(expected));
            }
        }
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn serial_parallel_resume_are_identical() {
        let root =
            std::env::temp_dir().join(format!("maginet-analysis-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let metadata = || Metadata {
            config: RunConfig {
                games: 3,
                max_plies: 200,
                replays: true,
                seed_namespace: Some("paired-baseline".into()),
                ..Default::default()
            },
            catalogue: vec![campaign_catalogue(false).remove(0)],
            graph: shared::campaign_connections(&campaign_catalogue(false)),
            main_route: shared::MAIN_ROUTE.iter().map(|s| s.to_string()).collect(),
            engine: "test".into(),
        };
        execute(&root.join("serial"), 1, metadata()).unwrap();
        execute(&root.join("parallel"), 3, metadata()).unwrap();
        fs::remove_file(root.join("parallel/matchup-00-2-2.json")).unwrap();
        execute(&root.join("parallel"), 2, metadata()).unwrap();
        for entry in fs::read_dir(root.join("serial")).unwrap() {
            let entry = entry.unwrap();
            assert_eq!(
                fs::read(entry.path()).unwrap(),
                fs::read(root.join("parallel").join(entry.file_name())).unwrap()
            );
        }
        let mut changed = metadata();
        changed.config.seed = 2;
        assert!(execute(&root.join("serial"), 1, changed).is_err());
        let mut changed = metadata();
        changed.catalogue[0].name.push_str(" changed");
        assert!(execute(&root.join("serial"), 1, changed).is_err());
        let mut changed = metadata();
        changed.config.seed_namespace = Some("different-baseline".into());
        assert!(execute(&root.join("serial"), 1, changed).is_err());
        let mut changed = metadata();
        changed.graph[0].one_way = !changed.graph[0].one_way;
        assert!(execute(&root.join("serial"), 1, changed).is_err());
        let mut changed = metadata();
        changed.main_route.swap(0, 1);
        assert!(execute(&root.join("serial"), 1, changed).is_err());
        let checkpoint = root.join("serial/matchup-00-0-0.json");
        let mut value: serde_json::Value =
            serde_json::from_slice(&fs::read(&checkpoint).unwrap()).unwrap();
        value["games"][0]["trial"] = serde_json::json!(999);
        fs::write(&checkpoint, serde_json::to_vec(&value).unwrap()).unwrap();
        assert!(execute(&root.join("serial"), 1, metadata()).is_err());
        fs::remove_file(root.join("serial/metadata.json")).unwrap();
        assert!(execute(&root.join("serial"), 1, metadata()).is_err());
        fs::remove_dir_all(root).unwrap();
    }
}
