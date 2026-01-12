export interface ResolutionConfig {
  ResolutionSizeX: number;
  ResolutionSizeY: number;
  RefreshRate: number;
}

export interface Config {
  Desktop: ResolutionConfig;
  Game: ResolutionConfig;
}

export interface MonitorStatus {
  isWatching: boolean;
  valorantRunning: boolean;
}