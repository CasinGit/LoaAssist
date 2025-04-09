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
    totalReward: number;
    gold: number;
    totalRaids: number;
    remainingReward: number;
    userSettings: UserSettingsType;
    workResetModalOpen: boolean;
}

// * Default initial state
const initialState: AppState = {
    liveDbName,
    raidsTable: [],
    totalReward: 0,
    gold: 0,
    totalRaids: 0,
    remainingReward: 0,
    userSettings: new UserSettingsType(),
    workResetModalOpen: false
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
                id, characters.priority, live_raids.charId, charName, class, color, raidType, complete, swap,
                COALESCE(default_raids.raidId, custom_raids.raidId) AS raidId,
                COALESCE(default_raids.level, custom_raids.level) AS level,
                COALESCE(default_raids.raidName, custom_raids.raidName) AS raidName,
                COALESCE(default_raids.difficulty, custom_raids.difficulty) AS difficulty,
                COALESCE(default_raids.gate, custom_raids.gate) AS gate,
                COALESCE(default_raids.reward, custom_raids.reward) AS reward,
                COALESCE(default_raids.resetType, custom_raids.resetType) AS resetType
            FROM live_raids
                LEFT OUTER JOIN default_raids ON live_raids.raidId = default_raids.raidId
                LEFT OUTER JOIN custom_raids ON live_raids.raidId = custom_raids.raidId
                LEFT OUTER JOIN characters ON live_raids.charId = characters.charId
            ORDER BY characters.priority, id
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

    // ? 등록되어있는 레이드 갯수만큼 Reward, Raids 계산
    raidsTable.map((arr) => {
        totalReward += arr.reward!;
        totalRaids += Number(arr.complete) === 0 ? 1 : 0;
        remainingReward += Number(arr.complete) === 0 ? arr.reward! : 0;
    });

    appStore.update((state) => ({
        ...state,
        raidsTable,
        totalReward,
        totalRaids,
        remainingReward
    }));
};

export const setLiveDB = (liveDB: ExtendsRaidType[]) => {
    liveDB.map((arr) => {
        arr.complete = Number(arr.complete) === 1 ? true : false;
    });

    appStore.update((state) => ({
        ...state,
        liveDB
    }));
};

export const setTotalReward = (totalReward: number) => {
    appStore.update((state) => ({
        ...state,
        totalReward
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

export const setTotalRaids = (totalRaids: number) => {
    appStore.update((state) => ({
        ...state,
        totalRaids
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

export const setUserSettings = async (userSettings: UserSettingsType) => {
    await invoke("set_user_settings", { settings: userSettings });
    // ! 깊은 복사로 동일 참조 문제 방지
    appStore.update((state) => ({
        ...state,
        userSettings: cloneDeep(userSettings)
    }));
};
