import { writable, type Writable } from "svelte/store";
import type { LeagueMetadata } from "./types";

export const isConnected = writable(true);
export const summonerName = writable("Encast");
export const gameData: Writable<LeagueMetadata> = writable({
    version: "unknown"
})
