use anyhow::Result;
use clap::{App, Arg};
use reqwest;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path;
use url;

// Computer\HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Epic Games\EpicGamesLauncher
//   AppDataPath C:\ProgramData\Epic\EpicGamesLauncher\Data\

#[derive(Deserialize)]
#[allow(dead_code)]
struct TxtManifest {
    #[serde(rename = "FormatVersion")]
    format_version: u32,
    #[serde(rename = "bIsIncompleteInstall")]
    is_incomplete_install: bool,
    #[serde(rename = "LaunchCommand")]
    launch_command: String,
    #[serde(rename = "LaunchExecutable")]
    launch_executable: String,
    #[serde(rename = "ManifestLocation")]
    manifest_location: String,
    #[serde(rename = "bIsApplication")]
    is_application: bool,
    #[serde(rename = "bIsExecutable")]
    is_executable: bool,
    #[serde(rename = "bIsManaged")]
    is_managed: bool,
    #[serde(rename = "bNeedsValidation")]
    needs_validation: bool,
    #[serde(rename = "bRequiresAuth")]
    requires_auth: bool,
    #[serde(rename = "bCanRunOffline")]
    can_run_offline: bool,
    #[serde(rename = "AppName")]
    app_name: String,
    #[serde(rename = "BaseURLs")]
    base_urls: Vec<String>,
    #[serde(rename = "BuildLabel")]
    build_label: String,
    #[serde(rename = "CatalogItemId")]
    catalog_item_id: String,
    #[serde(rename = "AppCategories")]
    app_categories: Vec<String>,
    #[serde(rename = "ChunkDbs")]
    chunk_dbs: Vec<String>,
    #[serde(rename = "CompatibleApps")]
    compatible_apps: Vec<String>,
    #[serde(rename = "DisplayName")]
    display_name: String,
    #[serde(rename = "FullAppName")]
    full_app_name: String,
    #[serde(rename = "InstallationGuid")]
    installation_guid: String,
    #[serde(rename = "InstallLocation")]
    install_location: String,
    #[serde(rename = "InstallSessionId")]
    install_session_id: String,
    #[serde(rename = "InstallTags")]
    install_tags: Vec<String>,
    #[serde(rename = "InstallComponents")]
    install_components: Vec<String>,
    #[serde(rename = "HostInstallationGuid")]
    host_installation_guid: String,
    #[serde(rename = "PrereqIds")]
    prereq_ids: Vec<String>,
    #[serde(rename = "StagingLocation")]
    staging_location: String,
    #[serde(rename = "TechnicalType")]
    technical_type: String,
    #[serde(rename = "VaultThumbnailUrl")]
    vault_thumbnail_url: String,
    #[serde(rename = "InstallSize")]
    install_size: u64,
    #[serde(rename = "MainWindowProcessName")]
    main_window_process_name: String,
    #[serde(rename = "ProcessNames")]
    process_names: Vec<String>,
    #[serde(rename = "MainGameAppName")]
    main_game_app_name: String,
    #[serde(rename = "MandatoryAppFolderName")]
    mandatory_app_folder_name: String,
    #[serde(rename = "OwnershipToken")]
    ownership_token: String,
}

#[derive(Serialize, Deserialize)]
struct EpicGame {
    name: String,
}

impl EpicGame {
    fn from_manifests() -> Vec<EpicGame> {
        let path: std::path::PathBuf =
            "c:/programdata/epic/epicgameslauncher/data/manifests".into();
        let mut games = Vec::new();
        for entry in std::fs::read_dir(&path).expect("Unable to open epic games launcher data") {
            let entry = entry.expect("Unable to find entry");
            let filename = entry.file_name().into_string().unwrap();
            if filename.ends_with(".item") {
                // println!("{}", &filename);
                let manifest: TxtManifest =
                    serde_json::from_str(&fs::read_to_string(path.join(filename)).unwrap())
                        .unwrap();
                games.push(EpicGame {
                    name: manifest.display_name,
                });
                // println!(
                //     "{} {} {}",
                //     manifest.display_name, manifest.install_location, manifest.launch_executable
                // );
                // let base_url = "https://www.epicgames.com/store/en-US/browse?q=";
                // let response: reqwest::Response = reqwest::get(
                //     url::Url::parse(&format!("{}{}", base_url, manifest.display_name)).unwrap(),
                // )
                // .await
                // .expect("Unable to run search");
                // println!("status: {}", response.status());
                // let contents = response.text().await.unwrap();
                // let doc = Html::parse_document(&contents);
                // println!("{:?}", &doc);
                // let selector = Selector::parse("img").unwrap();
                // for element in doc.select(&selector) {
                //     println!("{}", element.value().name());
                // }
            }
        }
        games
    }

    fn load(dir: &path::PathBuf) -> Result<Vec<EpicGame>> {
        let contents = fs::read_to_string(dir.join(EPIC_GAMES_JSON))?;
        let games = serde_json::from_str(&contents)?;
        Ok(games)
    }
}

static EPIC_GAMES_JSON: &str = "epic_games.json";
trait EpicGames {
    fn save(&self, dir: &path::PathBuf) -> Result<()>;
    fn merge(&mut self, games: Vec<EpicGame>);
}

impl EpicGames for Vec<EpicGame> {
    fn save(&self, dir: &path::PathBuf) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(dir.join(EPIC_GAMES_JSON), json)?;
        Ok(())
    }

    fn merge(&mut self, games: Vec<EpicGame>) {
        let mut existing: std::collections::HashSet<String> =
            self.iter().map(|g| g.name.clone()).collect();
        for game in games {
            let mut found = false;
            for orig in &mut self.iter_mut() {
                if &orig.name == &game.name {
                    // how to avoid the clone here?
                    orig.name = game.name.clone();
                    found = true;
                    existing.remove(&orig.name);
                }
            }
            if !found {
                println!("Adding: {}", &game.name);
                self.push(game);
            }
        }
        for game in existing {
            println!("Possibly removed: {}", game);
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new("epic")
        .arg(Arg::with_name("list").long("list"))
        .arg(Arg::with_name("refresh").long("refresh"))
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
    if matches.is_present("list") {
        for game in &games {
            println!("{}", game.name);
        }
    }
    games.save(&epic_home)?;
    Ok(())
}
