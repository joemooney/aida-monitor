// trace:STORY-1 | ai:antigravity
// trace:TASK-2 | ai:antigravity
// trace:STORY-10 | ai:antigravity

pub const DASHBOARD_CSS: &str = r#"
:root {
  --bg-main: #090d16;
  --bg-card: #0f172a;
  --bg-card-hover: #172033;
  --bg-surface: #1e293b;
  --border-subtle: rgba(255, 255, 255, 0.08);
  --border-focus: #06b6d4;
  --text-primary: #f8fafc;
  --text-secondary: #94a3b8;
  --text-muted: #64748b;
  
  --color-cyan: #06b6d4;
  --color-cyan-glow: rgba(6, 182, 212, 0.25);
  --color-emerald: #10b981;
  --color-emerald-glow: rgba(16, 185, 129, 0.25);
  --color-amber: #f59e0b;
  --color-amber-glow: rgba(245, 158, 11, 0.25);
  --color-rose: #f43f5e;
  --color-rose-glow: rgba(244, 63, 94, 0.25);
  --color-indigo: #818cf8;
  --color-purple: #c084fc;
}

* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

body {
  background-color: var(--bg-main);
  color: var(--text-primary);
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  line-height: 1.5;
  overflow-x: hidden;
}

.mono {
  font-family: "JetBrains Mono", "SF Mono", Menlo, Consolas, monospace;
}

/* Scrollbar */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}
::-webkit-scrollbar-track {
  background: var(--bg-main);
}
::-webkit-scrollbar-thumb {
  background: var(--border-subtle);
  border-radius: 4px;
}
::-webkit-scrollbar-thumb:hover {
  background: var(--text-muted);
}

/* App Container */
.app-container {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

/* Header */
.top-header {
  background: #0b1120;
  border-bottom: 1px solid var(--border-subtle);
  padding: 12px 24px;
  position: sticky;
  top: 0;
  z-index: 50;
}

.header-row-1 {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.brand-section {
  display: flex;
  align-items: center;
  gap: 14px;
}

.brand-badge {
  background: linear-gradient(135deg, #06b6d4, #3b82f6);
  color: #fff;
  font-weight: 800;
  font-size: 13px;
  letter-spacing: 1.5px;
  padding: 4px 10px;
  border-radius: 6px;
  text-transform: uppercase;
}

.brand-title {
  font-size: 18px;
  font-weight: 700;
  letter-spacing: -0.5px;
  color: #fff;
}

.project-tag {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid var(--border-subtle);
  padding: 3px 10px;
  border-radius: 6px;
  font-size: 12px;
  color: var(--text-secondary);
  display: flex;
  align-items: center;
  gap: 6px;
}

.project-name-pill {
  color: var(--color-cyan);
  font-weight: 600;
}

.header-status-ribbon {
  display: flex;
  align-items: center;
  gap: 16px;
  font-size: 13px;
}

.pulse-indicator {
  display: inline-flex;
  align-items: center;
  gap: 7px;
  font-size: 12px;
  font-weight: 600;
  color: var(--color-emerald);
}

.pulse-dot {
  width: 8px;
  height: 8px;
  background-color: var(--color-emerald);
  border-radius: 50%;
  box-shadow: 0 0 10px var(--color-emerald);
  animation: pulse-glow 2s infinite ease-in-out;
}

@keyframes pulse-glow {
  0%, 100% { transform: scale(1); opacity: 1; }
  50% { transform: scale(1.4); opacity: 0.6; }
}

.header-clock {
  color: var(--text-muted);
  font-size: 12px;
}

/* Nav Controls */
.header-row-2 {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 16px;
}

.tab-nav {
  display: flex;
  gap: 6px;
}

.tab-btn {
  background: transparent;
  border: 1px solid transparent;
  color: var(--text-secondary);
  padding: 6px 14px;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
}

.tab-btn:hover {
  background: rgba(255, 255, 255, 0.05);
  color: var(--text-primary);
}

.tab-btn.active {
  background: var(--bg-surface);
  border-color: var(--border-subtle);
  color: var(--color-cyan);
  font-weight: 600;
  box-shadow: 0 2px 8px rgba(0,0,0,0.3);
}

.search-and-tools {
  display: flex;
  align-items: center;
  gap: 10px;
}

.search-input {
  background: rgba(0, 0, 0, 0.3);
  border: 1px solid var(--border-subtle);
  color: var(--text-primary);
  padding: 6px 12px;
  border-radius: 6px;
  font-size: 13px;
  width: 220px;
  outline: none;
  transition: border-color 0.15s ease;
}

.search-input:focus {
  border-color: var(--color-cyan);
  box-shadow: 0 0 0 2px var(--color-cyan-glow);
}

.action-btn {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid var(--border-subtle);
  color: var(--text-primary);
  padding: 6px 14px;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.15s ease;
}

.action-btn:hover {
  background: var(--bg-surface);
  border-color: var(--color-cyan);
}

.action-btn.disabled {
  opacity: 0.5;
  cursor: not-allowed;
  pointer-events: none;
}

/* Zoom Pill */
.zoom-pill {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid var(--border-subtle);
  color: var(--text-secondary);
  padding: 3px 8px;
  border-radius: 6px;
  font-size: 11px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.15s ease;
  user-select: none;
}

.zoom-pill:hover {
  background: var(--bg-surface);
  color: var(--color-cyan);
  border-color: var(--color-cyan);
}

.zoom-pill.active {
  color: var(--color-cyan);
  border-color: rgba(6, 182, 212, 0.4);
  background: rgba(6, 182, 212, 0.1);
}

/* API Guard Pill */
.api-guard-pill {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  background: rgba(16, 185, 129, 0.1);
  border: 1px solid rgba(16, 185, 129, 0.25);
  color: #34d399;
  padding: 4px 10px;
  border-radius: 6px;
  font-size: 11px;
  font-weight: 600;
  letter-spacing: 0.2px;
  cursor: help;
  transition: all 0.15s ease;
}

.api-guard-pill:hover {
  background: rgba(16, 185, 129, 0.18);
  border-color: rgba(16, 185, 129, 0.4);
}

.shield-icon {
  font-size: 12px;
}

/* Interval Selector */
.interval-selector {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
}

.interval-label {
  color: var(--text-muted);
  font-size: 11px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.interval-group {
  display: flex;
  background: rgba(0, 0, 0, 0.35);
  border: 1px solid var(--border-subtle);
  border-radius: 6px;
  padding: 2px;
  gap: 2px;
}

.interval-btn {
  background: transparent;
  border: none;
  color: var(--text-secondary);
  padding: 3px 9px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.15s ease;
}

.interval-btn:hover {
  color: var(--text-primary);
  background: rgba(255, 255, 255, 0.05);
}

.interval-btn.active {
  background: var(--bg-surface);
  color: var(--color-cyan);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
}

/* Metric Ribbon Bar */
.metric-ribbon {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 12px;
  padding: 16px 24px 8px 24px;
}

.metric-card {
  background: var(--bg-card);
  border: 1px solid var(--border-subtle);
  border-radius: 8px;
  padding: 12px 16px;
  display: flex;
  flex-direction: column;
  transition: transform 0.1s ease, border-color 0.15s ease;
}

.metric-card:hover {
  border-color: rgba(255, 255, 255, 0.15);
}

.metric-label {
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.8px;
  color: var(--text-muted);
  font-weight: 600;
}

.metric-value {
  font-size: 24px;
  font-weight: 700;
  color: var(--text-primary);
  margin-top: 2px;
}

.metric-sub {
  font-size: 11px;
  color: var(--text-secondary);
  margin-top: 4px;
}

/* Dashboard Grid */
.dashboard-grid {
  display: grid;
  grid-template-columns: repeat(12, 1fr);
  gap: 18px;
  padding: 16px 24px 32px 24px;
}

.panel-col-12 { grid-column: span 12; }
.panel-col-8 { grid-column: span 8; }
.panel-col-6 { grid-column: span 6; }
.panel-col-4 { grid-column: span 4; }

@media (max-width: 1200px) {
  .panel-col-8, .panel-col-6, .panel-col-4 {
    grid-column: span 12;
  }
}

/* Panel Card */
.panel {
  background: var(--bg-card);
  border: 1px solid var(--border-subtle);
  border-radius: 10px;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.25);
  transition: border-color 0.2s ease;
}

.panel:hover {
  border-color: rgba(255, 255, 255, 0.12);
}

.panel.highlight-priority {
  border-color: rgba(6, 182, 212, 0.35);
  box-shadow: 0 4px 20px rgba(6, 182, 212, 0.08);
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 14px 18px;
  border-bottom: 1px solid var(--border-subtle);
  background: rgba(255, 255, 255, 0.015);
}

.panel-title-area {
  display: flex;
  align-items: center;
  gap: 10px;
}

.panel-icon-pill {
  width: 26px;
  height: 26px;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 13px;
  font-weight: 700;
}

.icon-cyan { background: var(--color-cyan-glow); color: var(--color-cyan); }
.icon-amber { background: var(--color-amber-glow); color: var(--color-amber); }
.icon-emerald { background: var(--color-emerald-glow); color: var(--color-emerald); }
.icon-rose { background: var(--color-rose-glow); color: var(--color-rose); }
.icon-indigo { background: rgba(129, 140, 248, 0.2); color: var(--color-indigo); }

.panel-title {
  font-size: 14px;
  font-weight: 700;
  letter-spacing: -0.2px;
  color: var(--text-primary);
}

.panel-counter {
  background: rgba(255, 255, 255, 0.07);
  padding: 2px 7px;
  border-radius: 12px;
  font-size: 11px;
  font-weight: 700;
  color: var(--text-secondary);
}

.panel-meta {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 11px;
}

.panel-age {
  color: var(--text-muted);
}

.panel-age.stale {
  color: var(--color-amber);
  font-weight: 600;
}

.panel-body {
  padding: 16px 18px;
  flex: 1;
}

.panel-footer {
  padding: 8px 18px;
  border-top: 1px solid var(--border-subtle);
  background: rgba(0, 0, 0, 0.2);
  font-size: 11px;
  color: var(--text-muted);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

/* Badges & Pills */
.badge {
  display: inline-flex;
  align-items: center;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 600;
  letter-spacing: 0.3px;
}

.badge-spec {
  background: rgba(6, 182, 212, 0.15);
  color: var(--color-cyan);
  border: 1px solid rgba(6, 182, 212, 0.3);
  font-weight: 700;
}

.badge-round {
  background: rgba(245, 158, 11, 0.15);
  color: var(--color-amber);
  border: 1px solid rgba(245, 158, 11, 0.3);
}

.badge-shelved {
  background: rgba(244, 63, 94, 0.15);
  color: var(--color-rose);
  border: 1px solid rgba(244, 63, 94, 0.3);
}

.badge-phase {
  background: rgba(129, 140, 248, 0.15);
  color: var(--color-indigo);
  border: 1px solid rgba(129, 140, 248, 0.3);
  text-transform: uppercase;
  font-size: 10px;
}

.badge-green {
  background: rgba(16, 185, 129, 0.15);
  color: var(--color-emerald);
  border: 1px solid rgba(16, 185, 129, 0.3);
}

/* Round Items List */
.rounds-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.round-card {
  background: rgba(255, 255, 255, 0.02);
  border: 1px solid var(--border-subtle);
  border-radius: 8px;
  padding: 14px;
  transition: background 0.15s ease, border-color 0.15s ease;
}

.round-card:hover {
  background: var(--bg-card-hover);
  border-color: rgba(255, 255, 255, 0.12);
}

.round-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.round-header-left {
  display: flex;
  align-items: center;
  gap: 10px;
}

.round-timer {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-primary);
  display: flex;
  align-items: center;
  gap: 6px;
}

.round-reason-box {
  background: rgba(0, 0, 0, 0.25);
  border-left: 3px solid var(--color-rose);
  padding: 8px 12px;
  border-radius: 0 6px 6px 0;
  margin-top: 8px;
  font-size: 12px;
}

.round-reason-title {
  font-weight: 600;
  color: var(--color-rose);
  margin-bottom: 2px;
}

.round-reason-detail {
  color: var(--text-secondary);
}

.round-hint {
  font-size: 11px;
  color: var(--text-muted);
  margin-top: 6px;
  font-style: italic;
}

/* Phase Clock */
.phase-items-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.phase-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  background: rgba(255, 255, 255, 0.02);
  border: 1px solid var(--border-subtle);
  border-radius: 6px;
  padding: 10px 14px;
}

.phase-progress-bar {
  height: 4px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 2px;
  margin-top: 8px;
  overflow: hidden;
  position: relative;
}

.phase-progress-fill {
  height: 100%;
  border-radius: 2px;
  transition: width 0.3s ease;
}

.progress-green { background: var(--color-emerald); }
.progress-amber { background: var(--color-amber); }
.progress-rose { background: var(--color-rose); }

/* Shelve Causes Breakdown */
.stacked-bar-container {
  height: 24px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 6px;
  overflow: hidden;
  display: flex;
  margin-bottom: 16px;
  border: 1px solid var(--border-subtle);
}

.stacked-bar-segment {
  height: 100%;
  transition: width 0.3s ease;
  position: relative;
}

.cause-legend {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
  gap: 10px;
  margin-bottom: 14px;
}

.cause-legend-item {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
}

.legend-dot {
  width: 10px;
  height: 10px;
  border-radius: 3px;
}

.diagnostic-callout {
  background: rgba(6, 182, 212, 0.06);
  border: 1px solid rgba(6, 182, 212, 0.2);
  border-radius: 6px;
  padding: 10px 14px;
  font-size: 12px;
  color: #a5f3fc;
  margin-top: 12px;
}

/* Tables */
.data-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 12px;
}

.data-table th {
  text-align: left;
  padding: 8px 10px;
  color: var(--text-muted);
  border-bottom: 1px solid var(--border-subtle);
  font-weight: 600;
  text-transform: uppercase;
  font-size: 10px;
  letter-spacing: 0.5px;
}

.data-table td {
  padding: 9px 10px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.04);
  color: var(--text-secondary);
}

.data-table tr:hover td {
  background: rgba(255, 255, 255, 0.02);
  color: var(--text-primary);
}

/* Event Feed Terminal Drawer */
.event-feed-box {
  background: #05070e;
  border: 1px solid var(--border-subtle);
  border-radius: 8px;
  padding: 12px;
  max-height: 380px;
  overflow-y: auto;
  font-size: 11px;
}

.event-line {
  display: flex;
  align-items: baseline;
  gap: 10px;
  padding: 4px 6px;
  border-radius: 4px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.02);
}

.event-line:hover {
  background: rgba(255, 255, 255, 0.03);
}

.empty-state {
  text-align: center;
  padding: 32px 16px;
  color: var(--text-muted);
  font-size: 13px;
}
"#;
