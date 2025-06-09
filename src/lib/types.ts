import type { Component } from "svelte";

import Dashboard from "./components/Dashboard.svelte";
import Setting from "./components/Setting.svelte";
import WorkSheet from "./components/WorkSheet.svelte";

// * 탭 목록 정의
export const TABS: TabType[] = [
    { id: "Tab1", name: "대시보드", component: Dashboard },
    { id: "Tab2", name: "숙제표", component: WorkSheet },
    { id: "Tab3", name: "설정", component: Setting }
];

// * Tab Component Type
export interface TabType {
    id: string;
    name: string;
    component: Component; // component: SvelteComponent<Record<string, any>, any, any>;
}

// * 클래스 타입
export enum ClassType {
    버서커 = 102,
    디스트로이어 = 103,
    워로드 = 104,
    홀리나이트 = 105,
    슬레이어 = 112,
    배틀마스터 = 303,
    인파이터 = 302,
    기공사 = 304,
    창술사 = 305,
    스트라이커 = 312,
    브레이커 = 313,
    데빌헌터 = 503,
    블래스터 = 504,
    호크아이 = 502,
    스카우터 = 505,
    건슬링어 = 512,
    아르카나 = 202,
    서머너 = 203,
    바드 = 204,
    소서리스 = 205,
    데모닉 = 403,
    블레이드 = 402,
    리퍼 = 404,
    소울이터 = 405,
    도화가 = 602,
    기상술사 = 603,
    환수사 = 604
}

// * DB Raid Type
export interface RaidType {
    id?: number;
    raidId: number;
    priority?: number;
    level: number | null;
    raidName: string | null;
    difficulty: string | null;
    gate: string | null;
    reward: number | null;
    resetType: number;
    source?: "default" | "group";
}

// * DB Character Type
export interface CharacterType {
    id?: number;
    charId: number;
    priority?: number;
    charName: string;
    class: string;
    color: string;
}

// * DB Live Raid Type
export interface LiveType {
    id?: number;
    charId: number;
    raidType: "default" | "group" | undefined;
    raidId: number;
    complete: boolean;
    swap: number;
    resetType: number;
}

// * Union Type
export interface ExtendsRaidType extends RaidType, CharacterType, LiveType {
    count?: any;
}

// * DB Table Version Type
export interface TableVersionType {
    tableName: string;
    version: number;
    updatedAt: Date;
}

// * Program User Settings Type
export class UserSettingsType {
    update_check_enabled: boolean;
    theme: string;
    class_image: boolean;
    folded_opacity_enabled: boolean;
    folded_settings: {
        opacity: number;
        idle_time: number;
    };
    auto_focus_enabled: boolean;
    auto_focus_settings: {
        game_title: string;
        shift_idle_time: number;
    };
    focus_border_enabled: boolean;
    default_tab: string;
    close_button_behavior: string;

    // ? 타입 기본값 설정
    constructor(initialSettings?: Partial<UserSettingsType>) {
        this.update_check_enabled = initialSettings?.update_check_enabled ?? true;
        this.theme = initialSettings?.theme ?? "light";
        this.class_image = initialSettings?.class_image ?? true;
        this.folded_opacity_enabled = initialSettings?.folded_opacity_enabled ?? true;
        this.folded_settings = initialSettings?.folded_settings ?? { opacity: 60, idle_time: 10 };
        this.auto_focus_enabled = initialSettings?.auto_focus_enabled ?? true;
        this.auto_focus_settings = initialSettings?.auto_focus_settings ?? {
            game_title: "LOST ARK (64-bit, DX11) v.3.5.7.1",
            shift_idle_time: 1
        };
        this.focus_border_enabled = initialSettings?.focus_border_enabled ?? true;
        this.default_tab = initialSettings?.default_tab ?? "Tab1";
        this.close_button_behavior = initialSettings?.close_button_behavior ?? "tray";
    }

    // ? defaultSettings, currentSettings 두개의 User Setting 객체 비교
    static getChangedSettings(
        defaultSettings: UserSettingsType,
        currentSettings: UserSettingsType
    ): Partial<UserSettingsType> {
        return compareObjects(defaultSettings, currentSettings);
    }
}

// + 객체 비교 유틸리티 함수
function compareObjects<T extends object>(defaultObj: T, currentObj: T): Partial<T> {
    const changes: Partial<T> = {};

    // ? 모든 키 비교
    for (const key of Object.keys(defaultObj) as (keyof T)[]) {
        const defaultValue = defaultObj[key];
        const currentValue = currentObj[key];

        // ? 객체인 경우 재귀적으로 비교
        if (isObject(defaultValue) && isObject(currentValue)) {
            const nestedChanges = compareObjects(defaultValue, currentValue);
            if (Object.keys(nestedChanges).length > 0) {
                changes[key] = nestedChanges as T[keyof T];
            }
        } else if (defaultValue !== currentValue) {
            // ? 값이 다른 경우 변경사항 기록
            changes[key] = currentValue;
        }
    }

    return changes;
}

// + 객체인지 확인하는 유틸리티 함수
function isObject(value: unknown): value is object {
    return typeof value === "object" && value !== null;
}

// + 업데이트 확인 반환 타입
export interface UpdateCheckResult {
    should_update: boolean;
    current_version: string;
    latest_version: string;
    info: {
        version: string;
        bootstrapper: {
            url: string;
            sha256: string;
        };
        release: {
            url: string;
            sha256: string;
        };
    };
}
