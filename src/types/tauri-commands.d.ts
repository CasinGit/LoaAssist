import type { SystemSounds, UpdateCheckResult, UserSettingsType } from "$lib/types";

// * invoke 커맨드 타입 지정
export interface TauriCommands {
    get_env: { name: string };
    resize_with_custom: {
        width: number;
        height: number;
        easing: "easeInOutQuad" | "easeInOutQuart" | "easeInOutExpo" | "easeInOutBack";
    };
    play_system_sound: { sound: keyof typeof SystemSounds } | undefined;
    get_gold: undefined; // ! Return: number
    set_gold: { value: number };
    increment_gold: { value: number };
    decrement_gold: { value: number };
    get_user_settings: undefined; // ! Return: UserSettingsType
    set_user_settings: { settings: UserSettingsType };
    get_position: undefined; // ! Return: { x: number, y: number }
    set_position: { newPosition: { x: number; y: number } };
    find_window_by_title: { target: string }; // ! Return: string
    pause_auto_focus: undefined;
    resume_auto_focus: undefined;
    get_os_info; // ! Return: [string, string]
    get_update_check_result: { forceRefresh: boolean }; // ! Return: UpdateCheckResult
    run_update_with_info: { info: UpdateCheckResult["info"] };
    get_default_tab; // ! Return: string
    exit_app;
    set_game_title: { title: string };
}

// prettier-ignore
// * invoke 반환값 타입 지정
export type TauriCommandResponse<K extends keyof TauriCommands>
    = K extends "get_gold" ? Promise<number>
    : K extends "get_user_settings" ? Promise<UserSettingsType>
    : K extends "get_position" ? Promise<{ x: number; y: number }>
    : K extends "find_window_by_title" ? Promise<string>
    : K extends "get_os_info" ? Promise<[string, string]>
    : K extends "get_update_check_result" ? Promise<UpdateCheckResult>
    : K extends "get_default_tab" ? Promise<string>
    : Promise<any>; // ? 기본 반환값
