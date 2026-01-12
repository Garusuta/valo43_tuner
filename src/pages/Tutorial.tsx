import React, { useState } from 'react';
import '../styles/tutorial.css';

interface TutorialStep {
  id: number;
  title: string;
  content: string;
  icon: string;
}

const tutorialSteps: TutorialStep[] = [
  {
    id: 1,
    title: '配置 NVIDIA 控制面板',
    content: `1. 打开 NVIDIA 控制面板
2. 进入 "调整桌面尺寸和位置"
3. 选择 "缩放" 选项卡
4. 设置缩放模式为 "全屏"
5. 勾选 "替代缩放模式" (如果可用)
6. 确保 "在以下项目上执行缩放" 设置为 "GPU"
7. 确保 "覆盖由游戏和程序设置的缩放模式" 被勾选"
`,

    icon: '🖥️',
  },
  {
    id: 2,
    title: '创建自定义分辨率',
    content: `1. 在 NVIDIA 控制面板中选择 "更改分辨率"
2. 点击 "自定义..."
3. 创建新分辨率，例如 1568x1080 或 1440x1080
4. 刷新率保持与显示器原生刷新率一致
5. 测试并应用自定义分辨率`,
    icon: '📐',
  },
  {
    id: 3,
    title: '配置本程序',
    content: `1. 进入 "设置" 页面
2. 设置桌面分辨率为你的显示器原生分辨率
3. 设置游戏分辨率为你想要的 4:3 分辨率
4. 确保刷新率配置正确`,
    icon: '⚙️',
  },
  {
    id: 4,
    title: '开始使用',
    content: `1. 在 "控制面板" 页面开启进程监听
2. 正常启动无畏契约
3. 程序会自动在游戏启动时切换分辨率
4. 游戏关闭后会自动恢复桌面分辨率`,
    icon: '🎮',
  },
];

const faqs = [
  {
    question: '为什么需要禁用副显示器？',
    answer: '在切换主显示器分辨率时，Windows 可能会将窗口移动到副显示器。暂时禁用副显示器可以避免这个问题。',
  },
  {
    question: '分辨率切换后画面模糊怎么办？',
    answer: '确保在 NVIDIA 控制面板中设置了正确的缩放模式，并且游戏内分辨率与系统分辨率匹配。',
  },
  {
    question: '程序无法禁用监视器？',
    answer: '请确保以管理员权限运行本程序。',
  },
  {
    question: '自定义分辨率无法创建？',
    answer: '某些显示器可能不支持特定的自定义分辨率。尝试使用接近的标准分辨率，或者使用 CRU 工具。',
  },
];

const Tutorial: React.FC = () => {
  const [expandedFaq, setExpandedFaq] = useState<number | null>(null);

  return (
    <div className="tutorial-page">
      <div className="page-header">
        <h1>使用教程</h1>
        <p className="page-description">跟随以下步骤配置你的 4:3 游戏体验</p>
      </div>

      {/* 步骤指南 */}
      <section className="tutorial-section">
        <h2 className="section-title">📋 配置步骤</h2>
        <div className="steps-container">
          {tutorialSteps.map((step, index) => (
            <div key={step.id} className="step-card">
              <div className="step-header">
                <div className="step-number">{step.id}</div>
                <div className="step-icon">{step.icon}</div>
              </div>
              <h3>{step.title}</h3>
              <pre className="step-content">{step.content}</pre>
              {index < tutorialSteps.length - 1 && (
                <div className="step-connector">
                  <div className="connector-line"></div>
                  <div className="connector-arrow">↓</div>
                </div>
              )}
            </div>
          ))}
        </div>
      </section>

      {/* 常见问题 */}
      <section className="tutorial-section">
        <h2 className="section-title">❓ 常见问题</h2>
        <div className="faq-container">
          {faqs.map((faq, index) => (
            <div
              key={index}
              className={`faq-item ${expandedFaq === index ? 'expanded' : ''}`}
            >
              <button
                className="faq-question"
                onClick={() => setExpandedFaq(expandedFaq === index ? null : index)}
              >
                <span>{faq.question}</span>
                <span className="faq-toggle">{expandedFaq === index ? '−' : '+'}</span>
              </button>
              <div className="faq-answer">
                <p>{faq.answer}</p>
              </div>
            </div>
          ))}
        </div>
      </section>

      {/* 提示信息 */}
      <section className="tutorial-section">
        <div className="tips-card">
          <div className="tips-icon">💡</div>
          <div className="tips-content">
            <h3>小提示</h3>
            <ul>
              <li>建议在游戏内将显示模式设置为 "全屏"</li>
              <li>确保游戏内分辨率与本程序设置的分辨率一致</li>
              <li>首次使用时建议手动测试分辨率切换是否正常</li>
              <li>如遇问题，可以尝试以管理员身份运行程序</li>
            </ul>
          </div>
        </div>
      </section>
    </div>
  );
};

export default Tutorial;