import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';

export interface ResolutionConfig {
  ResolutionSizeX: number;
  ResolutionSizeY: number;
  RefreshRate: number;
}

export interface Config {
  Desktop: ResolutionConfig;
  Game: ResolutionConfig;
}

const defaultConfig: Config = {
  Desktop: { ResolutionSizeX: 1920, ResolutionSizeY: 1080, RefreshRate: 144 },
  Game: { ResolutionSizeX: 1568, ResolutionSizeY: 1080, RefreshRate: 144 },
};

export function useConfig() {
  // 已保存的配置（来自文件）
  const [savedConfig, setSavedConfig] = useState<Config>(defaultConfig);
  // 当前编辑的配置（本地状态）
  const [config, setConfig] = useState<Config>(defaultConfig);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    loadConfig();
  }, []);

  const loadConfig = async () => {
    try {
      const result = await invoke<Config>('get_config');
      setSavedConfig(result);
      setConfig(result);
    } catch (error) {
      console.error('Failed to load config:', error);
    } finally {
      setLoading(false);
    }
  };

  // 检查是否有未保存的更改
  const hasChanges = useCallback(() => {
    return JSON.stringify(config) !== JSON.stringify(savedConfig);
  }, [config, savedConfig]);

  // 应用配置（保存到文件）
  const applyConfig = async () => {
    setSaving(true);
    try {
      await invoke('save_config', { config });
      setSavedConfig(config);
      return true;
    } catch (error) {
      console.error('Failed to save config:', error);
      return false;
    } finally {
      setSaving(false);
    }
  };

  // 重置为已保存的配置
  const resetConfig = () => {
    setConfig(savedConfig);
  };

  // 更新桌面配置（仅本地状态）
  const updateDesktop = (key: keyof ResolutionConfig, value: number) => {
    setConfig((prev) => ({
      ...prev,
      Desktop: {
        ...prev.Desktop,
        [key]: value,
      },
    }));
  };

  // 更新游戏配置（仅本地状态）
  const updateGame = (key: keyof ResolutionConfig, value: number) => {
    setConfig((prev) => ({
      ...prev,
      Game: {
        ...prev.Game,
        [key]: value,
      },
    }));
  };

  // 设置预设（仅本地状态）
  const setGamePreset = (width: number, height: number) => {
    setConfig((prev) => ({
      ...prev,
      Game: {
        ...prev.Game,
        ResolutionSizeX: width,
        ResolutionSizeY: height,
      },
    }));
  };

  return {
    config,
    savedConfig,
    loading,
    saving,
    hasChanges,
    updateDesktop,
    updateGame,
    setGamePreset,
    applyConfig,
    resetConfig,
    reloadConfig: loadConfig,
  };
}