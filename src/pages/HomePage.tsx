// src/pages/HomePage.tsx

import React, { useState, useEffect, useCallback, useRef } from 'react';
import {
  Card,
  Switch,
  Button,
  Space,
  Typography,
  message,
  Spin,
  Tag,
  Tabs,
  Radio,
  Empty,
  Divider,
  Alert,
} from 'antd';
import {
  PlayCircleOutlined,
  FolderOpenOutlined,
  CheckCircleOutlined,
  CloseCircleOutlined,
  ReloadOutlined,
  DesktopOutlined,
  MonitorOutlined,
} from '@ant-design/icons';
import { loadAllConfig, saveAllConfig } from '../ipc/config';
import { toggleWatching, getWatchingStatus, getGamingStatus } from '../ipc/watcher';
import { startGame } from '../ipc/valorant';
import { openFileDialog } from '../ipc/utils';
import { scanMonitors, getMonitorsMap, selectMonitor } from '../ipc/monitor';
import type { AppConfig, MonitorsMap } from '../types';

const { Text, Title } = Typography;

// 轮询间隔（毫秒）
const POLLING_INTERVAL = 2000;

type RunMode = 'default' | 'multi-monitor';

const HomePage: React.FC = () => {
  // ============ 通用状态 ============
  const [config, setConfig] = useState<AppConfig | null>(null);
  const [watchingStatus, setWatchingStatus] = useState<boolean>(false);
  const [gamingStatus, setGamingStatus] = useState<boolean>(false);
  const [loading, setLoading] = useState<boolean>(true);
  const [switchLoading, setSwitchLoading] = useState<boolean>(false);
  const [startLoading, setStartLoading] = useState<boolean>(false);
  const [selectPathLoading, setSelectPathLoading] = useState<boolean>(false);

  // ============ 模式切换状态 ============
  const [currentMode, setCurrentMode] = useState<RunMode>('default');

  // ============ 多显示器模式状态 ============
  const [monitorsMap, setMonitorsMap] = useState<MonitorsMap>({});
  const [selectedMonitor, setSelectedMonitor] = useState<string | null>(null);
  const [scanLoading, setScanLoading] = useState<boolean>(false);
  const [selectMonitorLoading, setSelectMonitorLoading] = useState<boolean>(false);

  // ============ 轮询控制 ============
  const pollingRef = useRef<ReturnType<typeof setInterval> | null>(null);
  const isPollingRef = useRef<boolean>(false);

  /**
   * 刷新状态（静默模式，不显示错误提示，用于轮询）
   */
  const refreshStatusSilent = useCallback(async () => {
    // 防止并发请求
    if (isPollingRef.current) return;
    isPollingRef.current = true;

    try {
      const [watchStatus, gameStatus, appConfig] = await Promise.all([
        getWatchingStatus(),
        getGamingStatus(),
        loadAllConfig(),
      ]);
      setWatchingStatus(watchStatus);
      setGamingStatus(gameStatus);
      setConfig(appConfig);
    } catch (error) {
      // 轮询时静默处理错误，避免频繁弹出提示
      console.error('轮询刷新失败:', error);
    } finally {
      isPollingRef.current = false;
    }
  }, []);

  /**
   * 刷新所有状态（手动触发，显示错误提示）
   */
  const refreshAllStatus = useCallback(async () => {
    try {
      const [watchStatus, gameStatus, appConfig] = await Promise.all([
        getWatchingStatus(),
        getGamingStatus(),
        loadAllConfig(),
      ]);
      setWatchingStatus(watchStatus);
      setGamingStatus(gameStatus);
      setConfig(appConfig);
      message.success('状态已刷新');
    } catch (error) {
      message.error(`刷新状态失败: ${error}`);
    }
  }, []);

  /**
   * 扫描并获取显示器列表
   */
  const refreshMonitorsList = useCallback(async () => {
    setScanLoading(true);
    try {
      await scanMonitors();
      const monitors = await getMonitorsMap();
      setMonitorsMap(monitors);

      if (selectedMonitor && !monitors[selectedMonitor]) {
        setSelectedMonitor(null);
      }

      message.success('显示器列表已刷新');
    } catch (error) {
      message.error(`扫描显示器失败: ${error}`);
    } finally {
      setScanLoading(false);
    }
  }, [selectedMonitor]);

  /**
   * 启动轮询
   */
  const startPolling = useCallback(() => {
    // 清除现有轮询
    if (pollingRef.current) {
      clearInterval(pollingRef.current);
    }

    // 启动新轮询
    pollingRef.current = setInterval(() => {
      refreshStatusSilent();
    }, POLLING_INTERVAL);

    console.log('轮询已启动，间隔:', POLLING_INTERVAL, 'ms');
  }, [refreshStatusSilent]);

  /**
   * 停止轮询
   */
  const stopPolling = useCallback(() => {
    if (pollingRef.current) {
      clearInterval(pollingRef.current);
      pollingRef.current = null;
      console.log('轮询已停止');
    }
  }, []);

  /**
   * 初始化加载
   */
  useEffect(() => {
    const init = async () => {
      setLoading(true);
      try {
        const [watchStatus, gameStatus, appConfig] = await Promise.all([
          getWatchingStatus(),
          getGamingStatus(),
          loadAllConfig(),
        ]);
        setWatchingStatus(watchStatus);
        setGamingStatus(gameStatus);
        setConfig(appConfig);
      } catch (error) {
        message.error(`初始化失败: ${error}`);
      } finally {
        setLoading(false);
      }
    };

    init();

    // 启动轮询
    startPolling();

    // 组件卸载时停止轮询
    return () => {
      stopPolling();
    };
  }, [startPolling, stopPolling]);

  /**
   * 切换到多显示器模式时自动扫描
   */
  useEffect(() => {
    if (currentMode === 'multi-monitor' && Object.keys(monitorsMap).length === 0) {
      refreshMonitorsList();
    }
  }, [currentMode, monitorsMap, refreshMonitorsList]);

  /**
   * 切换监听器开关
   */
  const handleToggleWatching = async () => {
    if (currentMode === 'multi-monitor' && !selectedMonitor && !watchingStatus) {
      message.warning('请先选择显示器');
      return;
    }

    setSwitchLoading(true);
    try {
      const newStatus = await toggleWatching();
      setWatchingStatus(newStatus);
      message.success(newStatus ? '监听器已启动' : '监听器已停止');
      // 立即刷新状态
      await refreshStatusSilent();
    } catch (error) {
      message.error(`切换监听器失败: ${error}`);
    } finally {
      setSwitchLoading(false);
    }
  };

  /**
   * 一键启动游戏
   */
  const handleStartGame = async () => {
    if (!config?.Watcher.GamePath) {
      message.warning('请先设置游戏路径');
      return;
    }

    setStartLoading(true);
    try {
      await startGame();
      message.success('游戏启动成功');
    } catch (error) {
      message.error(`启动游戏失败: ${error}`);
    } finally {
      setStartLoading(false);
    }
  };

  /**
   * 选择游戏路径
   */
  const handleSelectGamePath = async () => {
    setSelectPathLoading(true);
    try {
      const selectedPath = await openFileDialog({
        title: '选择游戏可执行文件',
        directory: false,
        filters: [{ name: '可执行文件', extensions: ['exe'] }],
      });

      if (!selectedPath) {
        setSelectPathLoading(false);
        return;
      }

      const currentConfig = await loadAllConfig();
      const updatedConfig: AppConfig = {
        ...currentConfig,
        Watcher: {
          ...currentConfig.Watcher,
          GamePath: selectedPath,
        },
      };

      await saveAllConfig(updatedConfig);
      setConfig(updatedConfig);
      message.success('游戏路径设置成功');
    } catch (error) {
      message.error(`设置游戏路径失败: ${error}`);
    } finally {
      setSelectPathLoading(false);
    }
  };

  /**
   * 选择显示器
   */
  const handleSelectMonitor = async (monitorName: string) => {
    setSelectMonitorLoading(true);
    try {
      await selectMonitor(monitorName);
      setSelectedMonitor(monitorName);
      message.success(`已选择显示器: ${monitorName}`);
    } catch (error) {
      message.error(`选择显示器失败: ${error}`);
    } finally {
      setSelectMonitorLoading(false);
    }
  };

  /**
   * 渲染状态信息区域
   */
  const renderStatusSection = () => (
    <div style={{ marginBottom: 24 }}>
      <Title level={5}>状态信息</Title>
      <Space direction="vertical" size="middle" style={{ width: '100%' }}>
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
          <Text>监听器状态：</Text>
          <Space>
            {watchingStatus ? (
              <Tag icon={<CheckCircleOutlined />} color="success">运行中</Tag>
            ) : (
              <Tag icon={<CloseCircleOutlined />} color="error">已停止</Tag>
            )}
            <Switch
              checked={watchingStatus}
              onChange={handleToggleWatching}
              loading={switchLoading}
              disabled={currentMode === 'multi-monitor' && !selectedMonitor && !watchingStatus}
            />
          </Space>
        </div>
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
          <Text>游戏状态：</Text>
          {gamingStatus ? (
            <Tag icon={<CheckCircleOutlined />} color="success">运行中</Tag>
          ) : (
            <Tag icon={<CloseCircleOutlined />} color="default">未运行</Tag>
          )}
        </div>
      </Space>
    </div>
  );

  /**
   * 渲染路径区域
   */
  const renderPathSection = () => (
    <div style={{ marginBottom: 24 }}>
      <Title level={5}>游戏路径</Title>
      <div style={{ padding: 12, background: '#fafafa', borderRadius: 6, border: '1px solid #d9d9d9' }}>
        {config?.Watcher.GamePath ? (
          <Text code>{config.Watcher.GamePath}</Text>
        ) : (
          <Text type="secondary">未设置游戏路径</Text>
        )}
      </div>
    </div>
  );

  /**
   * 渲染操作按钮区域
   */
  const renderActionSection = () => (
    <div>
      <Title level={5}>操作</Title>
      <Space size="middle">
        <Button
          type="primary"
          icon={<PlayCircleOutlined />}
          onClick={handleStartGame}
          loading={startLoading}
          size="large"
        >
          一键启动
        </Button>
        <Button
          icon={<FolderOpenOutlined />}
          onClick={handleSelectGamePath}
          loading={selectPathLoading}
          size="large"
        >
          选择游戏路径
        </Button>
      </Space>
    </div>
  );

  /**
   * 渲染默认模式
   */
  const renderDefaultMode = () => (
    <Card
      title={<Title level={4} style={{ margin: 0 }}>通用控制面板</Title>}
      style={{ width: 600 }}
      extra={
        <Button icon={<ReloadOutlined />} onClick={refreshAllStatus} type="text">
          刷新状态
        </Button>
      }
    >
      {renderStatusSection()}
      {renderPathSection()}
      {renderActionSection()}
    </Card>
  );

  /**
   * 渲染多显示器模式
   */
  const renderMultiMonitorMode = () => {
    const monitorEntries = Object.entries(monitorsMap);
    const isLocked = watchingStatus;

    return (
      <Card
        title={
          <Space>
            <MonitorOutlined />
            <span>多显示器模式</span>
          </Space>
        }
        style={{ width: 700 }}
        extra={
          <Button icon={<ReloadOutlined />} onClick={refreshAllStatus} type="text">
            刷新状态
          </Button>
        }
      >
        {isLocked && (
          <Alert
            message="监听器运行中，显示器选择已锁定"
            type="warning"
            showIcon
            style={{ marginBottom: 16 }}
          />
        )}

        {/* 显示器选择区 */}
        <div style={{ marginBottom: 24 }}>
          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 12 }}>
            <Title level={5} style={{ margin: 0 }}>选择显示器</Title>
            <Button
              icon={<ReloadOutlined />}
              onClick={refreshMonitorsList}
              loading={scanLoading}
              disabled={isLocked}
              size="small"
            >
              刷新显示器列表
            </Button>
          </div>

          {scanLoading ? (
            <div style={{ textAlign: 'center', padding: 24 }}>
              <Spin tip="正在扫描显示器..." />
            </div>
          ) : monitorEntries.length === 0 ? (
            <Empty description="未检测到显示器，请点击刷新重试" style={{ padding: 24 }} />
          ) : (
            <Radio.Group
              value={selectedMonitor}
              onChange={(e) => handleSelectMonitor(e.target.value)}
              disabled={isLocked || selectMonitorLoading}
              style={{ width: '100%' }}
            >
              <Space direction="vertical" style={{ width: '100%' }}>
                {monitorEntries.map(([monitorName, gpuName]) => (
                  <Radio
                    key={monitorName}
                    value={monitorName}
                    style={{
                      width: '100%',
                      padding: '12px 16px',
                      border: '1px solid #d9d9d9',
                      borderRadius: 6,
                      background: selectedMonitor === monitorName ? '#e6f7ff' : '#fff',
                      transition: 'all 0.3s',
                    }}
                  >
                    <Space>
                      <DesktopOutlined style={{ fontSize: 18, color: '#1890ff' }} />
                      <div>
                        <Text strong>{monitorName}</Text>
                        <br />
                        <Text type="secondary" style={{ fontSize: 12 }}>
                          显卡：{gpuName}
                        </Text>
                      </div>
                    </Space>
                  </Radio>
                ))}
              </Space>
            </Radio.Group>
          )}

          {!isLocked && !selectedMonitor && monitorEntries.length > 0 && (
            <Text type="secondary" style={{ display: 'block', marginTop: 8, fontSize: 12 }}>
              请选择一个显示器后，才能开启监听器
            </Text>
          )}
        </div>

        <Divider />

        {renderStatusSection()}
        {renderPathSection()}
        {renderActionSection()}
      </Card>
    );
  };

  if (loading) {
    return (
      <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '100%' }}>
        <Spin size="large" tip="加载中..." />
      </div>
    );
  }

  const tabItems = [
    {
      key: 'default',
      label: (
        <span>
          <DesktopOutlined />
          默认模式
        </span>
      ),
      children: (
        <div style={{ display: 'flex', justifyContent: 'center', paddingTop: 24 }}>
          {renderDefaultMode()}
        </div>
      ),
    },
    {
      key: 'multi-monitor',
      label: (
        <span>
          <MonitorOutlined />
          多显示器模式
        </span>
      ),
      children: (
        <div style={{ display: 'flex', justifyContent: 'center', paddingTop: 24 }}>
          {renderMultiMonitorMode()}
        </div>
      ),
    },
  ];

  return (
    <div style={{ height: '100%' }}>
      <Tabs
        activeKey={currentMode}
        onChange={(key) => setCurrentMode(key as RunMode)}
        items={tabItems}
        centered
        size="large"
      />
    </div>
  );
};

export default HomePage;