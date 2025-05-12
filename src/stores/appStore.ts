import Database from "@tauri-apps/plugin-sql";
import { cloneDeep } from "lodash-es";
import { writable } from "svelte/store";

import { initializerDB, updateDefaultRaidsTable } from "$lib/db";
import { DEFAULT_RAIDS_VERSION } from "$lib/db/schema";
import { UserSettingsType, type ExtendsRaidType, type TableVersionType } from "$lib/types";
import { invoke } from "$lib/utils/invoke";

// ? 프로그램 모드에 따라서 사용 DB 달라짐
export const liveDbName = import.meta.env.PROD ? "live" : "live_dev";
if (import.meta.env.PROD) {
    console.log(
        "%cThis is the production build.",
        "color:white; font-style:bold; background-color:dodgerBlue; padding:3px; border-radius:4px; font-size:12px;"
    );
} else {
    console.log(
        "%cThis is the development build.",
        "color:white; font-style:bold; background-color:dodgerBlue; padding:3px; border-radius:4px; font-size:12px;"
    );
}

// * Define the shape of the store's data
interface AppState {
    liveDbName: string;
    raidsTable: ExtendsRaidType[];
    gold: number;
    totalReward: number;
    totalRaids: number;
    remainingReward: number;
    remainingRaids: number;
    userSettings: UserSettingsType;
    workResetModalOpen: boolean;
    groupEditModalOpen: boolean;
    updateExists: boolean;
}

// * Default initial state
const initialState: AppState = {
    liveDbName,
    raidsTable: [],
    gold: 0,
    totalReward: 0,
    totalRaids: 0,
    remainingReward: 0,
    remainingRaids: 0,
    userSettings: new UserSettingsType(),
    workResetModalOpen: false,
    groupEditModalOpen: false,
    updateExists: false
};

// + Create a writable store with the initial state
export const appStore = writable<AppState>(initialState);

// + Optional: Create helper functions for updating state
export const loadLiveDB = async () => {
    console.log(
        "%cReload Live DB",
        "color:white; font-style:bold; background-color:limeGreen; padding:3px; border-radius:4px; font-size:12px;"
    );

    await initializerDB(); // * DataBase Initializer

    let raidsTable: ExtendsRaidType[] = [];

    const db = await Database.load(`sqlite:${liveDbName}.db`);

    try {
        const tableVersions: TableVersionType[] = await db.select(
            "SELECT * FROM table_versions WHERE tableName = 'default_raids'"
        );
        console.log(`Current Version: ${tableVersions[0].version}\nProgram Version: ${DEFAULT_RAIDS_VERSION}`);

        if (tableVersions[0].version < DEFAULT_RAIDS_VERSION) {
            // ? Default Raid Version이 현재 DB에 저장된 데이터보다 버전이 높을때
            console.log("Default Raids Table 버전을 업데이트 합니다.");
            await updateDefaultRaidsTable();
        } else {
            // ? Default Raid Version이 현재 DB에 저장된 데이터랑 같거나 낮을때
            console.log("Default Raids Table 버전을 업데이트 하지 않음.");
        }
    } catch (error) {
        console.error("Transaction failed:", error);
    }

    try {
        // ? live_raids 테이블에 등록되어있는 숙제표 기준으로 Table Join
        raidsTable = await db.select(`
            SELECT
                live_raids.id,
                characters.priority,
                live_raids.charId,
                characters.charName,
                characters.class,
                characters.color,
                live_raids.raidType,
                live_raids.complete,
                live_raids.swap,
                COALESCE(dr.raidId, cr.raidId) AS raidId,
                COALESCE(dr.level, cr.level) AS level,
                COALESCE(dr.raidName, cr.raidName) AS raidName,
                COALESCE(dr.difficulty, cr.difficulty) AS difficulty,
                COALESCE(dr.gate, cr.gate) AS gate,
                
                -- Reward 처리 부분
                COALESCE(dr.reward, cr_agg.reward_sum) AS reward,
                
                COALESCE(dr.resetType, cr.resetType) AS resetType

            FROM live_raids
            LEFT JOIN characters ON live_raids.charId = characters.charId

            -- 기존 default_raids
            LEFT JOIN default_raids AS dr ON live_raids.raidId = dr.raidId

            -- 기존 group_raids
            LEFT JOIN group_raids AS cr ON live_raids.raidId = cr.raidId

            -- 그룹 레이드 reward 합산용 서브쿼리
            LEFT JOIN (
                SELECT
                    cr_inner.raidId AS groupRaidId,
                    COALESCE(SUM(dr_inner.reward), 0) AS reward_sum
                FROM group_raids AS cr_inner
                LEFT JOIN group_raids_map AS crm ON cr_inner.raidId = crm.groupId
                LEFT JOIN default_raids AS dr_inner ON crm.raidId = dr_inner.raidId
                GROUP BY cr_inner.raidId
            ) AS cr_agg ON cr.raidId = cr_agg.groupRaidId

            ORDER BY characters.priority, live_raids.id;
        `);
    } catch (error) {
        console.error("Transaction failed:", error);
    } finally {
        await db.close();
    }

    // ? complete 값이 1이면 true로 타입 변환
    raidsTable.map((arr) => {
        arr.complete = Number(arr.complete) === 1 ? true : false;
    });

    // console.log(raidsTable);

    let totalReward: number = 0; // * 총합 골드 보상
    let totalRaids: number = 0; // * 레이드 총 갯수
    let remainingReward: number = 0; // * 남은 골드 보상
    let remainingRaids: number = 0; // * 남은 레이드 갯수

    // ? 등록되어있는 레이드 갯수만큼 Reward, Raids 계산
    raidsTable.map((arr) => {
        totalReward += arr.reward!;
        totalRaids++;
        remainingReward += Number(arr.complete) === 0 ? arr.reward! : 0;
        remainingRaids += Number(arr.complete) === 0 ? 1 : 0;
    });

    appStore.update((state) => ({
        ...state,
        raidsTable,
        totalReward,
        totalRaids,
        remainingReward,
        remainingRaids
    }));
};

export const setGold = (gold: number) => {
    appStore.update((state) => ({
        ...state,
        gold
    }));
};

export const openWorkResetModal = () => {
    appStore.update((state) => ({
        ...state,
        workResetModalOpen: true
    }));
};

export const closeWorkResetModal = () => {
    appStore.update((state) => ({
        ...state,
        workResetModalOpen: false
    }));
};

export const openGroupEditModal = () => {
    appStore.update((state) => ({
        ...state,
        groupEditModalOpen: true
    }));
};

export const closeGroupEditModal = () => {
    appStore.update((state) => ({
        ...state,
        groupEditModalOpen: false
    }));
};

export const getUserSettings = async () => {
    console.log(
        "%cGet User Setting Invoke.",
        "color:white; font-style:bold; background-color:limeGreen; padding:3px; border-radius:4px; font-size:12px;"
    );
    let userSettings = await invoke("get_user_settings");

    appStore.update((state) => ({
        ...state,
        userSettings
    }));
};
getUserSettings(); // * 프로그램 실행시 User Settings 불러오기

export const checkUpdate = async () => {
    // ? 프로그램 업데이트가 있는지 확인
    try {
        invoke("get_update_check_result").then((result) => {
            // console.log(result);
            if (result.should_update) {
                console.log("최신 버전이 존재함");
                appStore.update((state) => ({
                    ...state,
                    updateExists: true
                }));
            } else {
                console.log("최신 상태입니다.");
                appStore.update((state) => ({
                    ...state,
                    updateExists: false
                }));
            }
        });
    } catch (err) {
        console.error("업데이트 확인 실패:", err);
    }
};
checkUpdate(); // * 프로그램을 새로고침해도 무조건 한번 실행

export const setUserSettings = async (userSettings: UserSettingsType) => {
    await invoke("set_user_settings", { settings: userSettings });
    // ! 깊은 복사로 동일 참조 문제 방지
    appStore.update((state) => ({
        ...state,
        userSettings: cloneDeep(userSettings)
    }));
};
