/* eslint-disable */
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

declare global {
    interface Window {
        __TAURI_INVOKE__<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
    }
}

// Function avoids 'window not defined' in SSR
const invoke = () => window.__TAURI_INVOKE__;

export function recordAudio() {
    return invoke()<null>("record_audio")
}

export function getConfig() {
    return invoke()<Config>("get_config")
}

export function updateConfigKey(key: ConfigUpdatableKey) {
    return invoke()<Config>("update_config_key", { key })
}

export function getAcapFiles() {
    return invoke()<AcapFile[]>("get_acap_files")
}

export type ConfigUpdatableKey = { savePath: string } | { recordingDurationInSecs: number } | { theme: Theme } | { startRecordingKey: string }
export type Theme = "system" | "light" | "dark"
export type Config = { configFilePath: string; savePath: string; recordingDurationInSecs: number; theme: Theme; startRecordingKey: string }
export type AcapFile = { name: string; path: string }
