use once_cell::sync::Lazy;
use semver::Version;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    process::Command,
    sync::{
        atomic::{AtomicBool, Ordering},
        RwLock,
    },
};
use tauri::utils::platform::current_exe;
use tempfile::Builder;

static UPDATE_CACHE: Lazy<RwLock<Option<UpdateCheckResult>>> = Lazy::new(|| RwLock::new(None));
static IS_RUNNING: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

struct ResetGuard;

impl Drop for ResetGuard {
    // ? 스코프를 벗어나거나 함수가 종료되면 자동으로 호출됨.
    fn drop(&mut self) {
        IS_RUNNING.store(false, Ordering::SeqCst);
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LatestInfo {
    pub version: String,
    pub pub_date: String,
    pub notes: String,
    pub bootstrapper: DownloadInfo,
    pub release: DownloadInfo,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DownloadInfo {
    pub url: String,
    pub sha256: String,
}

#[derive(Debug, Serialize, Clone)]
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

/// + 프로그램 업데이트 확인
#[tauri::command]
pub async fn get_update_check_result(
    app: tauri::AppHandle,
    force_refresh: bool,
) -> Result<UpdateCheckResult, String> {
    // * 1. 함수가 이미 실행 중인지 확인
    if IS_RUNNING.swap(true, Ordering::SeqCst) {
        println!("🚫 이미 업데이트 확인 로직이 실행 중입니다.");
        return Err("이미 업데이트 확인이 진행 중입니다.".into());
    }

    // ? 종료 시 IS_RUNNING을 무조건 false로 되돌리는 안전장치
    let _reset_guard = ResetGuard;

    // * 2. 캐시 데이터 확인 (cache 생명주기 제한)
    {
        let cache = UPDATE_CACHE.read().unwrap();
        if !force_refresh {
            // ? force_refresh가 false 일때
            if let Some(cached_result) = cache.as_ref() {
                // ? cached_result에 반환(캐시)된 값이 있을때
                println!("✅ 캐시된 업데이트 결과 반환");
                return Ok(cached_result.clone());
            }
        }
    }

    println!("🔄 업데이트 정보 새로 요청 중...");

    // * 3. 현재 프로그램 버전을 확인
    let current_version = app.package_info().version.to_string();

    // * 4. 현재 버전을 파싱
    let current =
        Version::parse(&current_version).map_err(|e| format!("현재 버전 파싱 실패: {}", e))?;

    // * 5. latest.json 가져오기
    let response =
        reqwest::get("https://github.com/CasinGit/LoaAssist/releases/latest/download/latest.json")
            .await
            .map_err(|e| format!("요청 실패: {}", e))?;

    // * 6. 가져온 latest.json 데이터를 파싱
    let info: LatestInfo = response
        .json()
        .await
        .map_err(|e| format!("JSON 파싱 실패: {}", e))?;

    // * 7. 최신 버전을 파싱
    let latest =
        Version::parse(&info.version).map_err(|e| format!("최신 버전 파싱 실패: {}", e))?;

    println!("Server Version : {:?}", latest);
    println!("Current Version : {:?}", current);

    let result = UpdateCheckResult {
        should_update: latest > current,
        current_version,
        latest_version: info.version.clone(),
        info,
    };

    // * 8. 캐시 데이터 갱신
    {
        let mut cache = UPDATE_CACHE.write().unwrap();
        *cache = Some(result.clone());
    }

    Ok(result)
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
