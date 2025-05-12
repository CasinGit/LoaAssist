import { invoke as tauriInvoke } from "@tauri-apps/api/core";

import type { TauriCommandResponse, TauriCommands } from "../../types/tauri-commands";

// + 반환 타입을 K에 맞게 정확하게 지정
export function invoke<K extends keyof TauriCommands>(cmd: K, args?: TauriCommands[K]): TauriCommandResponse<K> {
    // ? 반환 타입을 TauriCommandResponse<K>로 수정
    return tauriInvoke(cmd, args) as TauriCommandResponse<K>; // ? 명시적으로 타입 캐스팅
}
