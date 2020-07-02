use reqwest;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json;
use url;

// Computer\HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Epic Games\EpicGamesLauncher
//   AppDataPath C:\ProgramData\Epic\EpicGamesLauncher\Data\

#[derive(Serialize, Deserialize)]
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

#[tokio::main]
async fn main() {
    let path: std::path::PathBuf = "c:/programdata/epic/epicgameslauncher/data/manifests".into();
    for entry in std::fs::read_dir(&path).expect("Unable to open epic games launcher data") {
        let entry = entry.expect("Unable to find entry");
        let filename = entry.file_name().into_string().unwrap();
        if filename.ends_with(".item") {
            println!("{}", &filename);
            let manifest: TxtManifest =
                serde_json::from_str(&std::fs::read_to_string(path.join(filename)).unwrap())
                    .unwrap();
            println!(
                "{} {} {}",
                manifest.display_name, manifest.install_location, manifest.launch_executable
            );
            let base_url = "https://www.epicgames.com/store/en-US/browse?q=";
            let response: reqwest::Response = reqwest::get(
                url::Url::parse(&format!("{}{}", base_url, manifest.display_name)).unwrap(),
            )
            .await
            .expect("Unable to run search");
            println!("status: {}", response.status());
            let contents = response.text().await.unwrap();
            let doc = Html::parse_document(&contents);
            println!("{:?}", &doc);
            let selector = Selector::parse("img").unwrap();
            for element in doc.select(&selector) {
                println!("{}", element.value().name());
            }
            break;
        }
    }
}
