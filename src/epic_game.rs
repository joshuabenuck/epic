use anyhow::Result;
use clipboard_win::{formats, Clipboard};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path;
use std::process::Command;

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
pub struct EpicGame {
    pub display_name: String,
    pub install_location: String,
    pub launch_executable: String,
    pub image_url: Option<String>,
}

impl EpicGame {
    pub fn from_manifests() -> Vec<EpicGame> {
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
                    display_name: manifest.display_name,
                    install_location: manifest.install_location,
                    launch_executable: manifest.launch_executable,
                    image_url: None,
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

    pub fn load(dir: &path::PathBuf) -> Result<Vec<EpicGame>> {
        let contents = fs::read_to_string(dir.join(EPIC_GAMES_JSON))?;
        let games = serde_json::from_str(&contents)?;
        Ok(games)
    }
}

pub static EPIC_GAMES_JSON: &str = "epic_games.json";

pub trait EpicGames {
    fn save(&self, dir: &path::PathBuf) -> Result<()>;
    fn merge(&mut self, games: Vec<EpicGame>);
    fn find_images(&mut self) -> Result<()>;
}

fn getline() -> Result<String> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    Ok(line)
}

fn yes_or_no(prompt: String) -> Result<bool> {
    let mut response = None;
    while response == None {
        println!("{}", &prompt);
        response = match getline()?.trim() {
            "yes" | "y" => Some(true),
            "no" | "n" => Some(false),
            _ => None,
        }
    }
    Ok(response.unwrap())
}

impl EpicGames for Vec<EpicGame> {
    fn save(&self, dir: &path::PathBuf) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(dir.join(EPIC_GAMES_JSON), json)?;
        Ok(())
    }

    fn merge(&mut self, games: Vec<EpicGame>) {
        let mut existing: std::collections::HashSet<String> =
            self.iter().map(|g| g.display_name.clone()).collect();
        for game in games {
            let mut found = false;
            for orig in &mut self.iter_mut() {
                // is display_name really the best key to use?
                if &orig.display_name == &game.display_name {
                    // how to avoid the clone here?
                    orig.display_name = game.display_name.clone();
                    orig.install_location = game.install_location.clone();
                    orig.launch_executable = game.launch_executable.clone();
                    found = true;
                    existing.remove(&orig.display_name);
                }
            }
            if !found {
                println!("Adding: {}", &game.display_name);
                self.push(game);
            }
        }
        for game in existing {
            println!("Possibly removed: {}", game);
        }
    }

    fn find_images(&mut self) -> Result<()> {
        for game in self.iter_mut() {
            if game.image_url.is_none() {
                let url = format!(
                    "https://www.epicgames.com/store/en-US/browse?q={}",
                    game.display_name
                );
                println!("url: {}", &url);
                let mut browser = Command::new("cmd");
                browser.args(&["/C", "start", "chrome", &url]);
                println!("{:?}", browser);
                browser.status()?;
                loop {
                    println!(
                        "Find the image for \"{}\", options 'skip', 'copy':",
                        &game.display_name
                    );
                    let mut line = getline()?;
                    match line.trim() {
                        "skip" | "s" => break,
                        "copy" | "c" => {
                            let mut paste_buffer = String::new();
                            Clipboard::new().unwrap().get_string(&mut paste_buffer)?;
                            let prompt = format!(
                                "For \"{}\", use the image url \"{}\"?",
                                &game.display_name, &paste_buffer
                            );
                            let yes = yes_or_no(prompt)?;
                            if yes {
                                game.image_url = Some(paste_buffer);
                                break;
                            }
                        }
                        _ => {
                            println!("Unrecognized input!");
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
