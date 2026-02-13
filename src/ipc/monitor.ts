// src/ipc/monitor.ts

import { invoke } from '@tauri-apps/api/core';
import type { MonitorsMap } from '../types';

/**
 * 扫描显示器并持久化到 AppState
 */
export async function scanMonitors(): Promise<void> {
  await invoke('scan_monitors');
}

/**
 * 获取 AppState 中的显示器映射
 */
export async function getMonitorsMap(): Promise<MonitorsMap> {
  const result = await invoke<MonitorsMap>('get_monitors_map');
  return result;
}

/**
 * 选择指定显示器（写入 AppState）
 * 注意：Tauri V2 会将 Rust 的 snake_case 参数自动转为 camelCase
 * Rust 端: monitor_name -> 前端: monitorName
 */
export async function selectMonitor(monitorName: string): Promise<void> {
  // 使用 camelCase 参数名
  await invoke('select_monitor', { monitorName });
}