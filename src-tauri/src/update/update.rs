use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    process::Command,
};
use tauri::utils::platform::current_exe;
use tempfile::Builder;

#[derive(Debug, Deserialize, Serialize)]
pub struct LatestInfo {
    pub version: String,
    pub pub_date: String,
    pub notes: String,
    pub bootstrapper: DownloadInfo,
    pub release: DownloadInfo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DownloadInfo {
    pub url: String,
    pub sha256: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateCheckResult {
    pub should_update: bool,
    pub current_version: String,
    pub latest_version: String,
    pub info: LatestInfo,
}

/// + SHA256 해시 계산
fn calculate_sha256<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 4096];

    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

/// + 업데이트 확인
#[tauri::command]
pub async fn get_update_check_result(app: tauri::AppHandle) -> Result<UpdateCheckResult, String> {
    use semver::Version;

    // ? 1. 현재 버전 확인
    let current_version = app.package_info().version.to_string();
    let current =
        Version::parse(&current_version).map_err(|e| format!("현재 버전 파싱 실패: {}", e))?;

    // ? 2. latest.json 가져오기
    let response =
        reqwest::get("https://github.com/CasinGit/LoaAssist/releases/latest/download/latest.json")
            .await
            .map_err(|e| format!("요청 실패: {}", e))?;

    let info: LatestInfo = response
        .json()
        .await
        .map_err(|e| format!("JSON 파싱 실패: {}", e))?;

    let latest =
        Version::parse(&info.version).map_err(|e| format!("최신 버전 파싱 실패: {}", e))?;

    println!("Server Version : {:?}", latest);
    println!("Current Version : {:?}", current);

    Ok(UpdateCheckResult {
        should_update: latest > current,
        current_version,
        latest_version: info.version.clone(),
        info,
    })
}

/// + 업데이트 실행
#[tauri::command]
pub async fn run_update_with_info(info: LatestInfo) -> Result<(), String> {
    // ? 임시 위치에 bootstrapper 다운로드
    let temp_dir = Builder::new()
        .prefix("LoaAssist-")
        .tempdir()
        .map_err(|e| format!("임시 디렉토리 생성 실패: {}", e))?;
    let bootstrapper_path = temp_dir.path().join("bootstrapper.exe");

    let bootstrapper_bytes = reqwest::get(&info.bootstrapper.url)
        .await
        .map_err(|e| format!("bootstrapper 다운로드 실패: {}", e))?
        .bytes()
        .await
        .map_err(|e| format!("바이트 변환 실패: {}", e))?;

    tokio::fs::write(&bootstrapper_path, &bootstrapper_bytes)
        .await
        .map_err(|e| format!("bootstrapper 저장 실패: {}", e))?;

    println!("Bootstrapper Path : {:?}", bootstrapper_path);

    // ? 해시 검증
    let actual_hash =
        calculate_sha256(&bootstrapper_path).map_err(|e| format!("SHA256 계산 실패: {}", e))?;
    if actual_hash != info.bootstrapper.sha256.to_lowercase() {
        return Err(format!(
            "❌ bootstrapper 해시 불일치\n예상: {}\n실제: {}",
            info.bootstrapper.sha256, actual_hash
        ));
    }

    // ? 현재 실행 경로 파악
    let original_path =
        current_exe().map_err(|e| format!("현재 실행 파일 경로 확인 실패: {}", e))?;

    // ? bootstrapper 실행
    Command::new(&bootstrapper_path)
        .args([
            "--original-path",
            original_path.to_str().unwrap(),
            "--update-url",
            &info.release.url,
            "--update-sha256",
            &info.release.sha256,
        ])
        .spawn()
        .map_err(|e| format!("bootstrapper 실행 실패: {}", e))?;

    // ! 앱 종료 (bootstrapper에서 종료 확인하고 업데이트 진행함)
    std::process::exit(0);
}
