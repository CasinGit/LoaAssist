import { createClient } from "@supabase/supabase-js";
import { message } from "@tauri-apps/plugin-dialog";
import { BaseDirectory, exists } from "@tauri-apps/plugin-fs";
import Database from "@tauri-apps/plugin-sql";

import { CREATE_DEFAULT_TABLES } from "./schema";
import { liveDbName } from "../../stores/appStore";

import type { RaidType, TableVersionType } from "$lib/types";

export let CURRENT_RAIDS_VERSION: number = 0; // * Current Raids Table Version
export let RAIDS_INFO_UPDATE: boolean = false;
export let SUPABASE_OFFLINE: boolean = false;
export let DB_NOT_EXIST: boolean = false;

// + Database Initializer 작업
export async function initializerDB() {
    let db: Database | null = null;
    let dbExists: boolean = false;

    // * 로컬 DB 파일 확인
    try {
        dbExists = await exists(`${liveDbName}.db`, { baseDir: BaseDirectory.AppData });
    } catch (error) {
        console.error("Failed to check database file:", error);
    }

    // * Local DB 파일이 있으면 실행
    if (dbExists) {
        // * DB 연결
        db = await Database.load(`sqlite:${liveDbName}.db`);

        try {
            // * Local DB 버전 불러오기
            const [localVersion]: TableVersionType[] = await db.select(
                "SELECT * FROM table_versions WHERE tableName = 'default_raids'"
            );
            CURRENT_RAIDS_VERSION = localVersion.version;

            if (localVersion.version) {
                CURRENT_RAIDS_VERSION = localVersion.version;
            }
        } catch (err) {
            console.error("Failed to read local version:", err);
        }
    }

    // * Supabase Connecting
    const supabase = createClient(import.meta.env.VITE_SUPABASE_URL, import.meta.env.VITE_SUPABASE_KEY);

    // * Supabase Server Table Version 불러오기
    const { data: versionData, error: versionError } = await supabase
        .from("table_versions")
        .select("*")
        .eq("tableName", "default_raids")
        .single<TableVersionType>();

    // * Supabase Server 데이터가 없을 때
    if (versionError || !versionData) {
        console.error("Failed to load server table version:", versionError.message);

        // * Local DB가 있을 때 실행하고 return
        if (dbExists) {
            // ? 상태 표시창에 경고 알림 표시
            SUPABASE_OFFLINE = true;
            message(
                "서버 데이터베이스 연결 중 오류가 발생했습니다.\n\n로컬 데이터베이스가 있는 경우 일부 기능은 계속 사용할 수 있습니다.\n문제가 지속되면 개발자에게 문의해 주세요.",
                {
                    title: "서버 연결 문제",
                    kind: "warning"
                }
            );
            if (db) await db.close();
            return;
        }

        // ? 상태 표시창에 오류 알림 표시
        DB_NOT_EXIST = true;
        message(
            "서버 데이터베이스에 연결하지 못했고 로컬 데이터베이스도 찾을 수 없습니다.\n\n현재 프로그램을 정상적으로 사용할 수 없는 상태입니다.\n데이터베이스 복구 방법은 프로그램 하단 상태창의 `DB 복구 필요` 버튼을 눌러 안내 페이지에서 확인해 주세요.",
            {
                title: "데이터베이스 오류",
                kind: "error"
            }
        );

        return;
    }

    console.log(`Table Version\nCurrent: ${CURRENT_RAIDS_VERSION}\nServer: ${versionData?.version}`);

    if (dbExists && CURRENT_RAIDS_VERSION < 1) {
        // ? 상태 표시창에 오류 알림 표시
        DB_NOT_EXIST = true;
        message(
            "로컬 데이터베이스 파일이 손상되었습니다.\n현재 프로그램을 정상적으로 사용할 수 없는 상태입니다.\n데이터베이스 복구 방법은 프로그램 하단 상태창의 `DB 복구 필요` 버튼을 눌러 안내 페이지에서 확인해 주세요.",
            {
                title: "데이터베이스 오류",
                kind: "error"
            }
        );

        if (db) await db.close();
        return new Error("Server table version load failed");
    }

    // * Supabase Server 연결에 이상 없을 때 실행
    try {
        RAIDS_INFO_UPDATE = true; // ? UI Lock

        // * 프로그램 첫 실행 (DB 없음, 초기화 분기)
        if (!dbExists) {
            console.warn("DB initializer 작업 진행\n(프로그램 첫 실행)");

            db = await Database.load(`sqlite:${liveDbName}.db`);

            await db.execute("BEGIN TRANSACTION;");

            // * 기본 테이블 생성
            for (const sql of CREATE_DEFAULT_TABLES) {
                await db.execute(sql);
            }

            // * Supabase Server Default Table 불러오기
            const { data: raidData, error: raidError } = await supabase
                .from("default_raids")
                .select("*")
                .order("raidId", { ascending: true })
                .overrideTypes<RaidType[], { merge: false }>();

            if (raidError || !raidData) {
                DB_NOT_EXIST = true;
                throw new Error("Failed to load default_raids from server");
            }

            // * Supabase 방식 초기화 작업
            await insertDefaultRaids(db, raidData, versionData.version, versionData.updatedAt);

            await db.execute("COMMIT");

            return console.log(
                "%cDB initialization complete.",
                "color:white; font-style:bold; background-color:green; padding:3px; border-radius:4px; font-size:12px;"
            );
        }

        console.warn("DB initializer 작업 X\n(첫 실행 아님)");

        // * Table 업데이트 진행 (DB 있음, 업데이트 분기)
        if (CURRENT_RAIDS_VERSION !== null && CURRENT_RAIDS_VERSION < versionData.version && db) {
            console.log("Default Raids Table 버전을 업데이트 합니다.");

            const { data: raidData, error: raidError } = await supabase
                .from("default_raids")
                .select("*")
                .order("raidId", { ascending: true })
                .overrideTypes<RaidType[], { merge: false }>();

            if (raidError || !raidData) {
                throw new Error("Failed to load default_raids from server");
            }

            await db.execute("BEGIN TRANSACTION;");

            // ! Table 업데이트할 때만 테이블 DELETE 진행
            await db.execute("DELETE FROM default_raids");

            await insertDefaultRaids(db, raidData, versionData.version, versionData.updatedAt);

            await db.execute("COMMIT;");

            message(`기본 레이드 정보가 업데이트되었습니다.\n업데이트 날짜: ${versionData.updatedAt}`);
        } else {
            console.log(
                "%cDefault Raids Table Already Latest.",
                "color:white; font-style:bold; background-color:green; padding:3px; border-radius:4px; font-size:12px;"
            );
        }
    } catch (error) {
        console.error("DB initializer error:", error);
        message("DB 초기화 실패: " + String(error), { title: "DB initializer error", kind: "error" });
        if (db) await db.execute("ROLLBACK;");
    } finally {
        RAIDS_INFO_UPDATE = false; // ? UI Un Lock
        CURRENT_RAIDS_VERSION = versionData.version;
        if (db) await db.close();
    }
}

// + Default Raids 데이터 적용 (초기화/업데이트 공용)
export async function insertDefaultRaids(db: Database, raids: RaidType[], version: number, updatedAt: Date) {
    try {
        console.log("초기화/업데이트 공용 함수 시작");

        // * Default Raids 테이블 데이터 생성
        let index: number = 1;
        for (const raid of raids) {
            await db.execute(
                `INSERT OR IGNORE INTO default_raids
                (raidId, priority, level, raidName, difficulty, gate, reward, resetType)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?);`,
                [raid.raidId, index, raid.level, raid.raidName, raid.difficulty, raid.gate, raid.reward, raid.resetType]
            );
            index++;
        }

        // * Default Table Version Update
        await db.execute(
            `INSERT INTO table_versions (tableName, version, updatedAt)
            VALUES ('default_raids', ?, ?)
            ON CONFLICT(tableName) DO UPDATE SET
            version = excluded.version,
            updatedAt = excluded.updatedAt;`,
            [version, updatedAt]
        );
    } catch (error) {
        console.log(error);
    }
}
