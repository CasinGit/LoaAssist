use rodio::{Decoder, OutputStream, Sink};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

const DEFAULT_SOUND: &str = "Background"; // ? 기본 사운드 이름

thread_local! {
    static THREAD_AUDIO_MANAGER: Mutex<Option<(OutputStream, Arc<Sink>)>> = Mutex::new(None);
}

/// + 사운드 파일 경로를 매핑
fn get_sound_path(sound: &str) -> Option<&'static str> {
    let sounds: HashMap<&str, &str> = HashMap::from([
        ("Background", r"C:\Windows\Media\Windows Background.wav"),
        ("Foreground", r"C:\Windows\Media\Windows Foreground.wav"),
        ("Logon", r"C:\Windows\Media\Windows Logon.wav"),
        (
            "Alarm",
            r"C:\Windows\Media\Windows Notify System Generic.wav",
        ),
    ]);

    sounds.get(sound).copied()
}

/// + 사운드 파일 재생 함수
pub fn play_system_sound(sound: Option<&str>) -> Result<(), String> {
    // ? 사운드 이름이 제공되지 않으면 기본값 사용
    let sound_name = sound.unwrap_or(DEFAULT_SOUND);

    let sound_path = get_sound_path(sound_name)
        .ok_or_else(|| format!("Sound '{}' not found in predefined list", sound_name))?;

    THREAD_AUDIO_MANAGER.with(|manager: &Mutex<Option<(OutputStream, Arc<Sink>)>>| {
        let mut guard = manager.lock().unwrap();

        // ? 기존 Sink 중단
        if let Some((_, sink)) = &*guard {
            sink.stop();
        }

        // ? 새로운 OutputStream과 Sink 생성
        let (stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| format!("Failed to get default output stream: {:?}", e))?;
        let sink = Sink::try_new(&stream_handle)
            .map_err(|e| format!("Failed to create audio sink: {:?}", e))?;

        // ? WAV 파일 열기 및 디코딩
        let file =
            File::open(sound_path).map_err(|e| format!("Failed to open sound file: {:?}", e))?;
        let source = Decoder::new(BufReader::new(file))
            .map_err(|e| format!("Failed to decode sound file: {:?}", e))?;

        // ? Sink에 사운드 추가 및 재생
        sink.append(source);
        sink.play();

        // ? 새 OutputStream과 Sink 저장
        *guard = Some((stream, Arc::new(sink)));

        Ok::<(), String>(()) // ? 성공 결과 반환
    })
}
