use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {
    // Dictionnaire pour stocker les statistiques de chaque équipe
    let mut stats: HashMap<String, (u32, u32, u32, u32, u32)> = HashMap::new();
    // (MP, W, D, L, P)

    for line in match_results.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() != 3 {
            continue;
        }

        let team1 = parts[0].trim();
        let team2 = parts[1].trim();
        let result = parts[2].trim();

        stats.entry(team1.to_string()).or_insert((0, 0, 0, 0, 0));
        stats.entry(team2.to_string()).or_insert((0, 0, 0, 0, 0));

        match result {
            "win" => {
                update(&mut stats, team1, 1, 0, 0, 3);
                update(&mut stats, team2, 0, 0, 1, 0);
            }
            "loss" => {
                update(&mut stats, team1, 0, 0, 1, 0);
                update(&mut stats, team2, 1, 0, 0, 3);
            }
            "draw" => {
                update(&mut stats, team1, 0, 1, 0, 1);
                update(&mut stats, team2, 0, 1, 0, 1);
            }
            _ => {}
        }
    }

    // Tri : par points décroissants puis par ordre alphabétique
    let mut teams: Vec<_> = stats.into_iter().collect();
    teams.sort_by(|a, b| {
        b.1 .4.cmp(&a.1 .4).then_with(|| a.0.to_lowercase().cmp(&b.0.to_lowercase()))
    });

    // Construction du tableau final
    let mut output = String::new();
    output.push_str("Team                           | MP |  W |  D |  L |  P\n");

    for (team, (mp, w, d, l, p)) in teams {
        output.push_str(&format!("{:<31}| {:>2} | {:>2} | {:>2} | {:>2} | {:>2}\n", team, mp, w, d, l, p));
    }

    output.trim_end().to_string()
}

// Fonction utilitaire pour mettre à jour les stats
fn update(stats: &mut HashMap<String, (u32, u32, u32, u32, u32)>, team: &str, w: u32, d: u32, l: u32, p: u32) {
    let entry = stats.get_mut(team).unwrap();
    entry.0 += 1; // MP
    entry.1 += w;
    entry.2 += d;
    entry.3 += l;
    entry.4 += p;
}
