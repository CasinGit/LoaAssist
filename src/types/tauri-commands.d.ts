import type { UpdateCheckResult, UserSettingsType } from "$lib/types";

const _SystemSounds = {
    Background: "일반 메세지, 경고음 사운드",
    Foreground: "시스템 오류 사운드",
    Logon: "시스템 로그온 사운드",
    Alarm: "일반적인 알림 사운드"
} as const;

export interface TauriCommands {
    get_env: { name: string };
    resize_with_custom: {
        width: number;
        height: number;
        easing: "easeInOutQuad" | "easeInOutQuart" | "easeInOutExpo" | "easeInOutBack";
    };
    play_system_sound: { sound: keyof typeof _SystemSounds } | undefined;
    get_gold: undefined; // ! No Arg, Return: number
    set_gold: { value: number };
    increment_gold: { value: number };
    decrement_gold: { value: number };
    get_user_settings: undefined; // ! No Arg, Return: UserSettingsType
    set_user_settings: { settings: UserSettingsType };
    get_position: undefined; // ! No Arg, Return: { x: number, y: number }
    set_position: { newPosition: { x: number; y: number } };
    find_window_by_title: { target: string }; // ! Arg: { target: string }, Return: string
    pause_auto_focus: undefined; // ! No Arg, No Return
    resume_auto_focus: undefined; // ! No Arg, No Return
    get_os_info; // ! No Arg, Return: [string, string]
    get_update_check_result; // ! No Arg, Return: UpdateCheckResult
    run_update_with_info; // ! Arg: { info: UpdateCheckResult.info }, No Return
}

// prettier-ignore
export type TauriCommandResponse<K extends keyof TauriCommands> 
    = K extends "get_gold" ? Promise<number>
    : K extends "get_user_settings" ? Promise<UserSettingsType>
    : K extends "get_position" ? Promise<{ x: number; y: number }>
    : K extends "find_window_by_title" ? Promise<string>
    : K extends "get_os_info" ? Promise<[string, string]>
    : K extends "get_update_check_result" ? Promise<UpdateCheckResult>
    : Promise<any>; // ? 기본 반환값
