import { BaseDirectory, exists } from "@tauri-apps/plugin-fs";
import Database from "@tauri-apps/plugin-sql";

import { CREATE_DEFAULT_TABLES, DEFAULT_RAIDS, DEFAULT_RAIDS_VERSION } from "./schema";
import { liveDbName } from "../../stores/appStore";

// + Database Initializer 작업
export async function initializerDB() {
    let dbExists: boolean = false;

    try {
        dbExists = await exists(`${liveDbName}.db`, { baseDir: BaseDirectory.AppData });
        // console.log(`Database file exists: ${dbExists}`);
    } catch (error) {
        console.error("Failed to check database file:", error);
    }

    if (dbExists) return console.log("DB 초기화 작업 거부됨");

    console.log("DB 초기화 작업 시작");
    const db = await Database.load(`sqlite:${liveDbName}.db`);

    try {
        await db.execute(CREATE_DEFAULT_TABLES);

        await db.execute("BEGIN TRANSACTION;");

        let index: number = 1;
        for (const raid of DEFAULT_RAIDS) {
            await db.execute(
                `
                INSERT OR IGNORE INTO default_raids
                (raidId, priority, level, raidName, difficulty, gate, reward, resetType)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8);
                `,
                [
                    raid.raidId,
                    index,
                    raid.level,
                    raid.raidName,
                    raid.difficulty,
                    raid.gate,
                    raid.reward,
                    raid.resetType,
                    0
                ]
            );
            index++;
        }

        await db.execute("COMMIT;");

        console.log(
            "%cTransaction committed successfully.",
            "color:white; font-style:bold; background-color:green; padding:3px; border-radius:4px; font-size:12px;"
        );
    } catch (error) {
        await db.execute("ROLLBACK;"); // ! 오류가 발생하면 롤백
        console.error("Transaction failed and rolled back:", error);
    } finally {
        await db.close();
    }
}

// + Default Raids Table 버전 업데이트 작업
export async function updateDefaultRaidsTable() {
    console.log("Default Raids Table Update...");

    const db = await Database.load(`sqlite:${liveDbName}.db`);

    try {
        await db.execute("BEGIN TRANSACTION;");

        await db.execute("DELETE FROM default_raids");

        let index: number = 1;
        for (const raid of DEFAULT_RAIDS) {
            await db.execute(
                `
                INSERT OR IGNORE INTO default_raids
                (raidId, priority, level, raidName, difficulty, gate, reward, resetType)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8);
                `,
                [
                    raid.raidId,
                    index,
                    raid.level,
                    raid.raidName,
                    raid.difficulty,
                    raid.gate,
                    raid.reward,
                    raid.resetType,
                    0
                ]
            );
            index++;
        }

        await db.execute(
            `UPDATE table_versions SET version = ${DEFAULT_RAIDS_VERSION}, updatedAt = CURRENT_TIMESTAMP WHERE tableName = 'default_raids'`
        );

        await db.execute("COMMIT;");

        console.log(
            "%cTransaction committed successfully.",
            "color:white; font-style:bold; background-color:green; padding:3px; border-radius:4px; font-size:12px;"
        );
    } catch (error) {
        await db.execute("ROLLBACK;"); // ! 오류가 발생하면 롤백
        console.error("Transaction failed and rolled back:", error);
    } finally {
        // await db.close();
        console.log("Default Raids Table Update Finish!");
    }
}
