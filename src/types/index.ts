// src/types/index.ts

/**
 * Watcher 配置接口
 */
export interface WatcherConfig {
  GamePath: string;
  Width: number;
  Height: number;
  Fps: number;
}

/**
 * Valorant 配置接口
 */
export interface ValorantConfig {
  LauncherPath: string;
  GamePath: string;
}

/**
 * 开发配置接口
 */
export interface DevelopmentConfig {
  Debug: boolean;
}

/**
 * 完整应用配置接口
 */
export interface AppConfig {
  Watcher: WatcherConfig;
  Valorant: ValorantConfig;
  Development: DevelopmentConfig;
}

/**
 * 显示器映射类型（key=显示器名称，value=显卡名称）
 */
export type MonitorsMap = Record<string, string>;

/**
 * 显示模式配置
 */
export interface DisplayMode {
  MonitorName: string;
}

/**
 * AppState 中的 Watcher 状态
 */
export interface WatcherState {
  DisplayMode: DisplayMode | null;
}

/**
 * AppState 数据结构（用于实时刷新）
 */
export interface AppStateData {
  Monitors: MonitorsMap;
  Watcher: WatcherState | null;
}