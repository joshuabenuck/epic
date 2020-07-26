use anyhow::Result;
use clap::{App, Arg};
use epic::{EpicGame, EpicGames, EPIC_GAMES_JSON};

#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new("epic")
        .arg(Arg::with_name("list").long("list"))
        .arg(Arg::with_name("refresh").long("refresh"))
        .arg(Arg::with_name("find-images").long("find-images"))
        .get_matches();
    let home = match dirs::home_dir() {
        None => {
            eprintln!("Unable to find home dir!");
            std::process::exit(1);
        }
        Some(h) => {
            if !h.exists() {
                eprintln!("Home dir doesn't exist!");
                std::process::exit(1);
            }
            h
        }
    };
    let epic_home = home.join(".epic");
    if !epic_home.exists() {
        std::fs::create_dir(&epic_home)?;
    }
    let json_path = epic_home.join(EPIC_GAMES_JSON);
    let mut games = if json_path.exists() {
        EpicGame::load(&epic_home)?
    } else {
        Vec::new()
    };
    if !json_path.exists() || matches.is_present("refresh") {
        let games_from_manifests = EpicGame::from_manifests();
        games.merge(games_from_manifests);
    }
    if matches.is_present("find-images") {
        games.find_images()?;
    }
    if matches.is_present("list") {
        for game in &games {
            println!("{}", game.display_name);
        }
    }
    games.save(&epic_home)?;
    Ok(())
}
