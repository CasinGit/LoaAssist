// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const prerender = true;
export const ssr = false;

import type { LayoutLoad } from "./$types";

import { initializerDB } from "$lib/db";

export const load: LayoutLoad = async () => {
    // DB 초기화가 끝날 때까지 기다림
    await initializerDB();

    return {};
};
