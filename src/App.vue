<template>
  <div class="app">
    <header class="header">
      <div class="header-left">
        <h1>{{ t('appTitle') }}</h1>
        <p class="subtitle">{{ t('appSubtitle') }}</p>
      </div>
      <div class="header-center">
        <div class="c-disk-widget" :class="'c-disk-' + cDiskInfo.level">
          <div class="c-disk-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
              <line x1="3" y1="9" x2="21" y2="9"/>
              <line x1="9" y1="21" x2="9" y2="9"/>
            </svg>
          </div>
          <div class="c-disk-content">
            <div class="c-disk-top">
              <span class="c-disk-label">{{ t('cDiskRemaining') }}</span>
              <span class="c-disk-value">{{ cDiskInfo.display }}</span>
            </div>
            <div class="c-disk-bar-wrap">
              <div class="c-disk-bar">
                <div class="c-disk-bar-fill" :style="{ width: cDiskInfo.usedPercent + '%' }"></div>
              </div>
              <span class="c-disk-status-badge">{{ cDiskInfo.label }}</span>
            </div>
          </div>
        </div>
      </div>
      <div class="header-right">
        <span class="admin-badge" :class="{ admin: isAdmin }" @click="!isAdmin && requestAdmin()" :title="!isAdmin ? t('requestAdmin') : ''">
          <span class="admin-icon">{{ isAdmin ? '🛡' : '👤' }}</span>
          <span>{{ isAdmin ? t('admin') : t('normalUser') }}</span>
          <span v-if="!isAdmin" class="admin-toggle-icon">▾</span>
        </span>
        <button @click="toggleLanguage" class="btn-lang" :title="t('languageSwitch')">
          {{ currentLang === 'zh' ? 'EN' : '中' }}
        </button>
      </div>
    </header>

    <div class="main-content">
      <div class="tool-panel">
        <div class="panel-header">
          <h2>{{ t('toolDetection') }} <template v-if="scanning">({{ reactiveFoundCount }}/{{ scanProgress.total }} {{ t('scanning') }})</template><template v-else>({{ installedCount }}/{{ tools.length }})</template></h2>
          <div class="actions">
            <button @click="scanTools" :disabled="scanning" class="btn-primary">
              {{ scanning ? t('scanningPercent', { p: scanPercent }) : t('startScan') }}
            </button>
            <button @click="selectAll" :disabled="tools.length === 0">{{ t('selectAll') }}</button>
            <button @click="deselectAll">{{ t('deselectAll') }}</button>
          </div>
        </div>

        <div class="filter-tabs">
          <span class="filter-tab" :class="{ active: filterMode === 'all' }" @click="filterMode = 'all'">
            {{ t('filterAll') }} ({{ tools.length }})
          </span>
          <span class="filter-tab" :class="{ active: filterMode === 'installed' }" @click="filterMode = 'installed'">
            {{ t('filterInstalled') }} ({{ installedCount }})
          </span>
          <span class="filter-tab" :class="{ active: filterMode === 'uninstalled' }" @click="filterMode = 'uninstalled'">
            {{ t('filterUninstalled') }} ({{ tools.length - installedCount }})
          </span>
        </div>

        <!-- 分类筛选 -->
        <div class="category-filter-bar">
          <span class="cat-filter-tab" :class="{ active: activeCategory === '' }" @click="activeCategory = ''">{{ t('allCategories') }} ({{ installedCount }})</span>
          <span v-for="cat in categories" :key="cat" class="cat-filter-tab" :class="{ active: activeCategory === cat }" @click="activeCategory = cat">
            {{ cat }} ({{ categoryCounts[cat] || 0 }})
          </span>
        </div>

        <div class="tool-list">
          <!-- 空状态：初始欢迎页 -->
          <div v-if="tools.length === 0 && !scanning" class="welcome-state">
            <div class="welcome-animation">
              <div class="floating-icons">
                <span class="float-icon" style="animation-delay:0s">🦀</span>
                <span class="float-icon" style="animation-delay:0.4s">🐍</span>
                <span class="float-icon" style="animation-delay:0.8s">📦</span>
                <span class="float-icon" style="animation-delay:1.2s">⚡</span>
                <span class="float-icon" style="animation-delay:1.6s">🗄️</span>
                <span class="float-icon" style="animation-delay:2s">🖊️</span>
                <span class="float-icon" style="animation-delay:2.4s">🧶</span>
                <span class="float-icon" style="animation-delay:2.8s">🐘</span>
              </div>
              <div class="welcome-main-icon">🧹</div>
            </div>
            <h2 class="welcome-title">{{ t('welcomeTitle') }}</h2>
            <p class="welcome-desc" v-html="t('welcomeDesc')"></p>
            <p class="welcome-sub">{{ t('welcomeSub') }}</p>
            <button @click="scanTools" class="btn-primary btn-large btn-welcome">
              {{ t('startScanBtn') }}
            </button>
            <div class="welcome-features">
              <span>{{ t('feature1') }}</span>
              <span>{{ t('feature2') }}</span>
              <span>{{ t('feature3') }}</span>
            </div>
          </div>

          <!-- 扫描进度 -->
          <div v-if="scanning" class="scan-progress-section">
            <div class="scan-progress-card">
              <div class="scan-progress-header">
                <div class="scan-progress-title">
                  <span class="scan-current-icon">{{ currentScanEmoji }}</span>
                  <span>{{ t('scanningTool') }} <strong>{{ scanProgress.tool_name }}</strong></span>
                </div>
                <span class="scan-progress-badge">{{ scanProgress.current }}/{{ scanProgress.total }}</span>
              </div>
              <div class="progress-bar bar-medium scan-bar">
                <div class="scan-progress-fill" :style="{ width: scanPercent + '%' }"></div>
              </div>
              <div class="scan-progress-info">
                <span>{{ t('scanProgress') }}: {{ scanProgress.current }}/{{ scanProgress.total }}</span>
                <span class="scan-percent">{{ scanPercent }}%</span>
              </div>
              <div class="scan-found-row" v-if="reactiveFoundCount > 0">
                <span class="scan-found-label">{{ t('found') }}</span>
                <span class="scan-found-count">{{ reactiveFoundCount }}</span>
                <span class="scan-found-sep">{{ t('foundTools') }}</span>
                <span class="scan-found-size">{{ scanSizeFormatted }}</span>
              </div>
            </div>
            <div class="scan-skeleton-list">
              <div v-for="i in 5" :key="i" class="skeleton-item">
                <div class="skeleton-icon"></div>
                <div class="skeleton-info">
                  <div class="skeleton-line w60"></div>
                  <div class="skeleton-line w40"></div>
                  <div class="skeleton-line w25"></div>
                </div>
              </div>
            </div>
          </div>

          <template v-if="!scanning && tools.length > 0" v-for="group in filteredGroupedTools" :key="group.category">
            <div class="category-header" @click="toggleCategory(group.category)">
              <span class="collapse-icon">{{ collapsedCategories.includes(group.category) ? '▶' : '▼' }}</span>
              <span class="category-name">{{ group.category }}</span>
              <span class="category-count">{{ group.tools.filter(t => t.installed).length }}/{{ group.tools.length }}</span>
              <span v-if="groupHasVirtualDisk(group)" class="risk-badge">{{ t('noJunction') }}</span>
            </div>

            <div v-show="!collapsedCategories.includes(group.category)" class="category-body">
              <div v-if="groupHasVirtualDisk(group)" class="risk-warning">
                {{ t('virtualDiskWarning') }}
              </div>

              <div v-for="tool in group.tools" :key="tool.id" class="tool-item"
                :class="{ disabled: !tool.installed, migrated: migratedTools.includes(tool.id) }">
                <input type="checkbox" :id="tool.id" v-model="selectedTools" :value="tool.id" :disabled="!tool.installed" />
                <div class="tool-icon" :class="{ active: tool.installed }">{{ getToolEmoji(tool) }}</div>
                <div class="tool-info">
                  <div class="tool-name-row">
                    <span class="tool-name">{{ tool.name }}</span>
                    <span class="risk-tag" :class="'risk-' + tool.risk_level" v-if="tool.installed">
                      {{ tool.risk_level === 'high' ? t('riskHigh') : tool.risk_level === 'medium' ? t('riskMedium') : t('riskLow') }}
                    </span>
                  </div>
                  <div class="tool-desc">{{ tool.description }}</div>
                  <div class="tool-meta">
                    <span class="size" v-if="tool.installed">{{ tool.size_formatted }}</span>
                    <span class="status" :class="{ installed: tool.installed }">{{ tool.installed ? t('installed') : t('notInstalled') }}</span>
                    <span class="strategy">{{ tool.strategy }}</span>
                    <span class="migrated-tag" v-if="migratedTools.includes(tool.id)">
                      <span class="migrated-icon">✓</span>
                      <span class="migrated-text">{{ t('migrated') }}</span>
                      <span class="migrated-count" v-if="(toolStatusMap[tool.id] || 0) > 1">{{ toolStatusMap[tool.id] }}{{ t('times') }}</span>
                    </span>
                  </div>
                </div>
                <button v-if="tool.installed && tool.paths.length > 0" @click.stop="openToolLocation(tool)" class="btn-open-location" :title="t('openLocation')">
                  {{ t('openLocation') }}
                </button>
              </div>
            </div>
          </template>

          <div v-if="!scanning && tools.length > 0 && filteredGroupedTools.length === 0" class="empty-state">
            {{ t('noTools') }}
          </div>
        </div>
      </div>

      <div class="action-panel">
        <div class="config-section">
          <div class="config-header">
            <h3>{{ t('migrationConfig') }}</h3>
            <button @click="openSupportedToolsDialog" class="btn-supported-tools">
              {{ t('supportedTools', { n: allToolDefinitions.length }) }}
            </button>
          </div>
          <div class="config-item">
            <label>{{ t('targetDir') }}</label>
            <div class="path-row">
              <input type="text" v-model="targetPath" :placeholder="t('targetPlaceholder')" class="path-input" />
              <button @click="browseFolder" class="btn-browse">{{ t('browse') }}</button>
              <button @click="openTargetDir" class="btn-browse" :disabled="!targetPath">{{ t('open') }}</button>
            </div>
          </div>
          <div class="config-hint">{{ t('configHint') }}</div>
        </div>

        <div class="button-group">
          <button @click="preCheckAndMigrate" :disabled="selectedTools.length === 0 || !targetPath || migrating" class="btn-primary btn-large">
            {{ migrating ? t('migrating') : t('startMigrate', { n: selectedTools.length }) }}
          </button>
          <div class="btn-row">
            <button @click="openRollbackDialog" :disabled="migratedTools.length === 0 || isRollingBack" class="btn-secondary btn-half">{{ t('rollback', { n: migratedTools.length }) }}</button>
            <button @click="showLogs = true" class="btn-secondary btn-half">{{ t('viewLogs') }}</button>
          </div>
        </div>

        <!-- 总体进度 -->
        <div v-if="overallProgress.total > 0" class="progress-section">
          <h3>{{ t('migrationProgress') }}</h3>
          <div class="overall-progress">
            <div class="overall-header">
              <span>{{ t('total') }}: {{ overallProgress.done }}/{{ overallProgress.total }} {{ t('tools') }}</span>
              <span>{{ overallPercent }}%</span>
            </div>
            <div class="progress-bar">
              <div class="progress-fill" :style="{ width: overallPercent + '%' }"></div>
            </div>
          </div>
          <div v-if="migrationProgress" class="current-tool-progress">
            <div class="progress-header">
              <span>{{ migrationProgress.tool_name }}</span>
              <span>{{ migrationProgress.progress }}%</span>
            </div>
            <div class="progress-bar bar-small">
              <div class="progress-fill" :style="{ width: migrationProgress.progress + '%' }"></div>
            </div>
            <div class="progress-message">{{ migrationProgress.message }}</div>
            <div class="progress-stats" v-if="migrationProgress.speed">
              <span>{{ t('speed') }}: {{ migrationProgress.speed }}</span>
              <span>{{ t('remaining') }}: {{ migrationProgress.eta }}</span>
            </div>
          </div>
        </div>

        <div class="stats-section">
          <h3>{{ t('stats') }}</h3>
          <div class="stat-item">
            <span class="stat-label">{{ t('detectedTools') }}</span>
            <span class="stat-value" :class="{ 'stat-live': scanning }">{{ scanning ? reactiveFoundCount : installedCount }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">{{ t('selectedMigrate') }}</span>
            <span class="stat-value">{{ selectedTools.length }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">{{ t('completedMigrate') }}</span>
            <span class="stat-value">{{ migratedTools.length }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">{{ t('totalSpace') }}</span>
            <span class="stat-value" :class="{ 'stat-live': scanning }">{{ scanning ? scanSizeFormatted : totalInstalledSize }}</span>
          </div>
        </div>

        <div class="about-section">
          <div class="about-logo-wrap">
            <img src="/logo.png" class="about-logo" alt="logo" />
            <div class="about-brand">{{ t('aboutSlogan') }}</div>
          </div>
          <div class="about-line"><span class="al-label">{{ t('company') }}</span>{{ t('companyName') }}</div>
          <div class="about-line"><span class="al-label">{{ t('version') }}</span>1.0.0 <button @click="checkForUpdate" class="btn-check-update" :disabled="updateChecking">{{ updateChecking ? t('checking') : t('checkUpdate') }}</button></div>
          <div class="about-line"><span class="al-label">{{ t('website') }}</span><a href="https://www.hongjuzi.com.cn?spm=c.cleaner" target="_blank">www.hongjuzi.com.cn</a></div>
          <div class="about-line"><span class="al-label">{{ t('email') }}</span>xjiujiu@foxmail.com</div>
          <div class="about-line"><span class="al-label">{{ t('github') }}</span><a href="http://github.com/HongJuZi" target="_blank">HongJuZi</a></div>
        </div>
      </div>
    </div>

    <!-- 前置条件检测 + 风险确认弹窗 -->
    <div v-if="showPreCheck" class="modal-overlay" @click="!preChecking && (showPreCheck = false)">
      <div class="modal modal-precheck" @click.stop>
        <h3>{{ t('precheckTitle') }}</h3>
        <p class="modal-desc">{{ t('precheckDesc') }}</p>

        <div v-if="!preChecking && preCheckResults.length > 0" class="precheck-list">
          <div v-for="r in preCheckResults" :key="r.tool_id" class="precheck-item" :class="'risk-' + r.risk_level">
            <div class="precheck-header">
              <span class="precheck-name">{{ r.tool_name }}</span>
              <span class="risk-tag" :class="'risk-' + r.risk_level">
                {{ r.risk_level === 'high' ? t('riskHigh') : r.risk_level === 'medium' ? t('riskMedium') : t('riskLow') }}
              </span>
              <span class="precheck-status" :class="{ ok: r.can_migrate }">
                {{ r.can_migrate ? t('canMigrate') : t('needConfirm') }}
              </span>
            </div>
            <div class="precheck-details">
              <span>{{ t('diskNeeded') }}: {{ r.disk_needed }}</span>
              <span>{{ t('diskAvailable') }}: {{ r.disk_free }}</span>
            </div>
            <div v-if="r.warnings.length > 0" class="precheck-warnings">
              <div v-for="(w, i) in r.warnings" :key="i" class="warning-line">⚠ {{ w }}</div>
            </div>
            <div v-if="!r.can_migrate" class="precheck-force">
              <label>
                <input type="checkbox" v-model="forceMigrateIds" :value="r.tool_id" />
                {{ t('forceMigrate') }}
              </label>
            </div>
          </div>
        </div>

        <div class="modal-footer">
          <button @click="showPreCheck = false" :disabled="preChecking">{{ t('cancel') }}</button>
          <button @click="confirmAndMigrate" :disabled="preChecking || !canProceedMigration" class="btn-danger">
            {{ preChecking ? t('prechecking') : t('confirmMigrate') }}
          </button>
        </div>
      </div>
    </div>

    <!-- 回滚对话框 -->
    <div v-if="showRollback" class="modal-overlay" @click="isRollingBack ? null : showRollback = false">
      <div class="modal" @click.stop>
        <h3>{{ t('rollbackTitle') }}</h3>

        <!-- 回滚进度 -->
        <div v-if="isRollingBack && rollbackProgress" class="rollback-progress-section">
          <div class="progress-header">
            <span>{{ rollbackProgress.tool_name }}</span>
            <span>{{ rollbackProgress.progress }}%</span>
          </div>
          <div class="progress-bar bar-small">
            <div class="progress-fill" :style="{ width: rollbackProgress.progress + '%' }"></div>
          </div>
          <div class="progress-message">{{ rollbackProgress.message }}</div>
          <div class="progress-stats" v-if="rollbackProgress.speed">
            <span>{{ t('speed') }}: {{ rollbackProgress.speed }}</span>
            <span>{{ t('remaining') }}: {{ rollbackProgress.eta }}</span>
          </div>
          <div class="rollback-warning">
            {{ t('rollbackWarning') }}
          </div>
        </div>

        <!-- 工具选择列表（回滚中隐藏） -->
        <div v-if="!isRollingBack">
          <p class="modal-desc">{{ t('rollbackDesc') }}</p>
          <div class="rollback-list">
            <div v-for="toolId in migratedTools" :key="toolId" class="rollback-item">
              <input type="checkbox" :id="'rollback-' + toolId" v-model="selectedRollback" :value="toolId" />
              <label :for="'rollback-' + toolId">{{ getToolNameById(toolId) }}</label>
            </div>
            <div v-if="migratedTools.length === 0" class="empty-hint">{{ t('noMigratedTools') }}</div>
          </div>
        </div>

        <div class="modal-footer">
          <button @click="showRollback = false" :disabled="isRollingBack">{{ t('cancel') }}</button>
          <button @click="doRollback" :disabled="selectedRollback.length === 0 || isRollingBack" class="btn-danger">
            {{ isRollingBack ? t('rollingBack') : t('confirmRollback', { n: selectedRollback.length }) }}
          </button>
        </div>
      </div>
    </div>

    <!-- 日志对话框 -->
    <div v-if="showLogs" class="modal-overlay" @click="showLogs = false">
      <div class="modal modal-large" @click.stop>
        <h3>{{ t('logsTitle') }}</h3>
        <pre class="log-content">{{ logs || t('noLogs') }}</pre>
        <div class="modal-footer">
          <button @click="exportLogs">{{ t('exportLogs') }}</button>
          <button @click="showLogs = false">{{ t('close') }}</button>
        </div>
      </div>
    </div>

    <!-- 迁移完成对话框 -->
    <div v-if="showCompletion" class="modal-overlay" @click="showCompletion = false">
      <div class="modal" @click.stop>
        <h3>{{ t('completionTitle') }}</h3>
        <p class="modal-desc">{{ t('completionDesc') }}</p>
        <div class="completion-list">
          <div v-for="name in completedToolNames" :key="name" class="completion-item">
            {{ name }}
          </div>
        </div>
        <p class="modal-hint">{{ t('completionHint') }}</p>
        <div class="modal-footer">
          <button @click="showCompletion = false" class="btn-primary">{{ t('gotIt') }}</button>
        </div>
      </div>
    </div>

    <!-- 更新进度对话框 -->
    <div v-if="updateDownloading" class="modal-overlay">
      <div class="modal" @click.stop>
        <h3>{{ t('downloadingUpdate') }}</h3>
        <p class="modal-desc">{{ t('downloadingVersion', { v: updateInfo?.version || '' }) }}</p>
        <div class="download-progress-wrap">
          <div class="progress-bar bar-medium">
            <div class="progress-fill" :style="{ width: downloadPercent + '%' }"></div>
          </div>
          <div class="download-info">
            <span>{{ downloadPercent }}%</span>
            <span>{{ formatDownloadSize(downloadProgress.downloaded) }} / {{ formatDownloadSize(downloadProgress.total) }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- 更新发现对话框 -->
    <div v-if="showUpdatePrompt && updateInfo" class="modal-overlay" @click="showUpdatePrompt = false">
      <div class="modal" @click.stop>
        <h3>{{ t('newVersion') }}</h3>
        <div class="update-info">
          <div class="update-info-row">
            <span class="update-label">{{ t('currentVersion') }}</span>
            <span>{{ updateInfo.local_version }}</span>
          </div>
          <div class="update-info-row">
            <span class="update-label">{{ t('latestVersion') }}</span>
            <span class="update-version">{{ updateInfo.version }}</span>
          </div>
          <div class="update-info-row">
            <span class="update-label">{{ t('releaseDate') }}</span>
            <span>{{ updateInfo.date }}</span>
          </div>
          <div class="update-info-row">
            <span class="update-label">{{ t('publisher') }}</span>
            <span>{{ updateInfo.author }}</span>
          </div>
        </div>
        <div class="modal-footer">
          <button @click="dismissUpdate">{{ t('later') }}</button>
          <button @click="startDownloadUpdate" class="btn-primary">{{ t('upgradeNow') }}</button>
        </div>
      </div>
    </div>

    <!-- 已支持软件列表对话框 -->
    <div v-if="showSupportedTools" class="modal-overlay" @click="showSupportedTools = false">
      <div class="modal modal-large supported-tools-modal" @click.stop>
        <div class="supported-tools-header">
          <h3>{{ t('supportedToolsTitle', { n: allToolDefinitions.length }) }}</h3>
          <div class="supported-tools-filter">
            <span class="st-filter-tab" :class="{ active: stActiveCategory === '' }" @click="stActiveCategory = ''">
              {{ t('allCategories') }} ({{ allToolDefinitions.length }})
            </span>
            <span v-for="cat in stCategories" :key="cat" class="st-filter-tab" :class="{ active: stActiveCategory === cat }" @click="stActiveCategory = cat">
              {{ cat }} ({{ stCategoryCounts[cat] || 0 }})
            </span>
          </div>
        </div>
        <div class="supported-tools-grid">
          <div v-for="tool in filteredSupportedTools" :key="tool.id" class="st-tool-card">
            <span class="st-tool-icon">{{ getToolEmojiById(tool.icon) }}</span>
            <div class="st-tool-info">
              <span class="st-tool-name">{{ tool.name }}</span>
              <span class="st-tool-desc">{{ tool.description }}</span>
            </div>
          </div>
        </div>
        <div class="modal-footer">
          <button @click="showSupportedTools = false">{{ t('close') }}</button>
        </div>
      </div>
    </div>

    <!-- Toast -->
    <div v-if="toast" class="toast" :class="toast.type">{{ toast.message }}</div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

// ==================== i18n ====================
type Lang = 'zh' | 'en';
const currentLang = ref<Lang>('zh');

const translations: Record<Lang, Record<string, string>> = {
  zh: {
    // Header
    appTitle: 'C盘小清新',
    appSubtitle: '生产力软件缓存一键迁移工具 v1.0',
    cDiskRemaining: 'C盘剩余',
    cDiskStatusGood: '充足',
    cDiskStatusModerate: '一般',
    cDiskStatusLow: '不足',
    admin: '管理员',
    normalUser: '普通用户',
    requestAdmin: '申请管理员权限',
    confirmRequestAdmin: '确定要申请管理员权限吗？\n程序将重新启动以获取管理员权限。',
    languageSwitch: '中/EN',

    // Tool panel
    toolDetection: '工具检测',
    scanning: '扫描中',
    startScan: '开始扫描',
    scanningPercent: '扫描中 {p}%',
    selectAll: '全选',
    deselectAll: '反选',
    filterAll: '全部',
    filterInstalled: '已安装',
    filterUninstalled: '未安装',
    allCategories: '全部分类',
    noTools: '当前筛选条件下无工具',

    // Welcome
    welcomeTitle: 'C盘小清新',
    welcomeDesc: '把C盘上占用大量空间的<strong>生产力软件缓存</strong>自动搬家到其他盘符',
    welcomeSub: '支持检测100+款开发工具/服务/编译器，一键迁移释放C盘空间',
    startScanBtn: '🚀 开始扫描检测',
    feature1: '✅ 智能检测已安装工具',
    feature2: '🔄 Junction软链接 / 环境变量 自动迁移',
    feature3: '️ 风险提示 + 一键回滚',

    // Scan progress
    scanningTool: '正在扫描',
    scanProgress: '检测进度',
    found: '已发现',
    foundTools: '个工具，占用',

    // Risk
    riskHigh: '高风险',
    riskMedium: '中风险',
    riskLow: '低风险',
    installed: '已安装',
    notInstalled: '未安装',
    migrated: '已迁移',
    times: '次',
    openLocation: '打开位置',
    virtualDiskWarning: '该类工具包含虚拟磁盘 (禁止软链接)，请使用官方导出/导入方案迁移',
    noJunction: '禁止软链接',

    // Config
    migrationConfig: '迁移配置',
    supportedTools: '已支持软件（{n}）款',
    targetDir: '目标目录：',
    targetPlaceholder: '例如: D:\\DevCache',
    browse: '浏览',
    open: '打开',
    configHint: '迁移后缓存存储在目标目录 c-drive-cleaner 子文件夹',

    // Buttons
    startMigrate: '开始迁移 ({n})',
    migrating: '迁移中...',
    rollback: '一键回滚 ({n})',
    viewLogs: '查看日志',

    // Progress
    migrationProgress: '迁移进度',
    total: '总体',
    tools: '个工具',
    speed: '速度',
    remaining: '剩余',

    // Stats
    stats: '统计信息',
    detectedTools: '已检测工具',
    selectedMigrate: '已选择迁移',
    completedMigrate: '已完成迁移',
    totalSpace: '总占用空间',

    // About
    aboutSlogan: '让C盘不再红 — 生产力软件缓存一键搬家',
    company: '开发公司：',
    companyName: '湖南红橘子科技有限公司',
    version: '版本：',
    checkUpdate: '检测最新版',
    checking: '检查中...',
    website: '网址：',
    email: 'Email：',
    github: 'GitHub：',

    // Precheck modal
    precheckTitle: '迁移前检查',
    precheckDesc: '正在检测前置条件...',
    canMigrate: '可迁移',
    needConfirm: '需确认',
    diskNeeded: '需要',
    diskAvailable: '磁盘可用',
    forceMigrate: '我已了解风险，强制迁移',
    cancel: '取消',
    confirmMigrate: '确认迁移',
    prechecking: '检测中...',

    // Rollback modal
    rollbackTitle: '一键回滚',
    rollbackDesc: '选择要回滚的工具，将恢复原始 C 盘路径：',
    noMigratedTools: '暂无已迁移工具',
    rollbackWarning: '⚠ 回滚过程中请勿关闭窗口或退出程序！可以最小化等待完成',
    confirmRollback: '确认回滚 ({n})',
    rollingBack: '回滚中...',

    // Logs modal
    logsTitle: '迁移日志',
    noLogs: '暂无日志',
    exportLogs: '导出日志',
    close: '关闭',

    // Completion modal
    completionTitle: '迁移完成',
    completionDesc: '以下工具已成功迁移，请重启相关软件使配置生效：',
    completionHint: '提示：关闭并重新打开上述软件即可自动加载新路径的缓存',
    gotIt: '知道了',

    // Update modal
    downloadingUpdate: '正在下载更新',
    downloadingVersion: '正在下载 {v} 版本...',
    newVersion: '发现新版本',
    currentVersion: '当前版本：',
    latestVersion: '最新版本：',
    releaseDate: '发布日期：',
    publisher: '发布者：',
    later: '稍后再说',
    upgradeNow: '立即升级',

    // Supported tools modal
    supportedToolsTitle: '已支持软件（{n}）款',

    // Toasts
    scanComplete: '扫描完成，检测到 {n} 个工具',
    scanFailed: '扫描失败',
    folderSelectUnavailable: '文件夹选择不可用，请手动输入路径',
    openFolderFailed: '打开文件夹失败',
    adminRestart: '请以管理员身份重新启动程序',
    adminFailed: '提权失败',
    checkFailed: '检测失败',
    migrateFailed: '迁移失败',
    rollbackFailed: '回滚失败',
    logsExported: '日志已导出',
    exportUnavailable: '导出不可用',
    loadLogsFailed: '加载日志失败',
    upToDate: '已是最新版本',
    downloadComplete: '下载完成，即将安装...',
    updateFailed: '更新失败',
    hashVerifyFailed: '更新包检验失败，请重试',
  },
  en: {
    // Header
    appTitle: 'C Drive Light',
    appSubtitle: 'One-click cache migration tool v1.0',
    cDiskRemaining: 'C: Free',
    cDiskStatusGood: 'Good',
    cDiskStatusModerate: 'Fair',
    cDiskStatusLow: 'Low',
    admin: 'Admin',
    normalUser: 'Standard User',
    requestAdmin: 'Request Admin',
    confirmRequestAdmin: 'Request administrator privileges?\nThe program will restart to apply.',
    languageSwitch: '中/EN',

    // Tool panel
    toolDetection: 'Tool Detection',
    scanning: 'Scanning',
    startScan: 'Start Scan',
    scanningPercent: 'Scanning {p}%',
    selectAll: 'Select All',
    deselectAll: 'Invert',
    filterAll: 'All',
    filterInstalled: 'Installed',
    filterUninstalled: 'Not Installed',
    allCategories: 'All Categories',
    noTools: 'No tools match current filter',

    // Welcome
    welcomeTitle: 'C Drive Light',
    welcomeDesc: 'Automatically move <strong>productivity software caches</strong> from C: drive to other drives',
    welcomeSub: 'Supports 100+ dev tools/services/compilers, one-click migration to free C: space',
    startScanBtn: '🚀 Start Scan',
    feature1: '✅ Smart detection of installed tools',
    feature2: '🔄 Auto migration via Junction / Env Vars',
    feature3: '🛡️ Risk alerts + one-click rollback',

    // Scan progress
    scanningTool: 'Scanning',
    scanProgress: 'Progress',
    found: 'Found',
    foundTools: 'tools, using',

    // Risk
    riskHigh: 'High Risk',
    riskMedium: 'Medium Risk',
    riskLow: 'Low Risk',
    installed: 'Installed',
    notInstalled: 'Not Installed',
    migrated: 'Migrated',
    times: 'times',
    openLocation: 'Open',
    virtualDiskWarning: 'This category contains virtual disks (Junction not allowed). Use official export/import instead.',
    noJunction: 'No Junction',

    // Config
    migrationConfig: 'Migration Config',
    supportedTools: 'Supported ({n})',
    targetDir: 'Target:',
    targetPlaceholder: 'e.g. D:\\DevCache',
    browse: 'Browse',
    open: 'Open',
    configHint: 'Migrated caches stored in c-drive-cleaner subfolder under target directory',

    // Buttons
    startMigrate: 'Start Migration ({n})',
    migrating: 'Migrating...',
    rollback: 'Rollback ({n})',
    viewLogs: 'View Logs',

    // Progress
    migrationProgress: 'Migration Progress',
    total: 'Total',
    tools: 'tools',
    speed: 'Speed',
    remaining: 'ETA',

    // Stats
    stats: 'Statistics',
    detectedTools: 'Detected',
    selectedMigrate: 'Selected',
    completedMigrate: 'Completed',
    totalSpace: 'Total Size',

    // About
    aboutSlogan: 'Keep C: drive clean — one-click cache migration',
    company: 'Developer: ',
    companyName: 'HongJuZi Technology Co., Ltd.',
    version: 'Version: ',
    checkUpdate: 'Check Update',
    checking: 'Checking...',
    website: 'Website: ',
    email: 'Email: ',
    github: 'GitHub: ',

    // Precheck modal
    precheckTitle: 'Pre-migration Check',
    precheckDesc: 'Checking prerequisites...',
    canMigrate: 'Ready',
    needConfirm: 'Needs Review',
    diskNeeded: 'Required',
    diskAvailable: 'Available',
    forceMigrate: 'I understand the risks, force migrate',
    cancel: 'Cancel',
    confirmMigrate: 'Confirm Migrate',
    prechecking: 'Checking...',

    // Rollback modal
    rollbackTitle: 'One-click Rollback',
    rollbackDesc: 'Select tools to rollback, restoring original C: paths:',
    noMigratedTools: 'No migrated tools',
    rollbackWarning: '⚠ Do not close the window during rollback! You can minimize and wait.',
    confirmRollback: 'Confirm Rollback ({n})',
    rollingBack: 'Rolling back...',

    // Logs modal
    logsTitle: 'Migration Logs',
    noLogs: 'No logs',
    exportLogs: 'Export Logs',
    close: 'Close',

    // Completion modal
    completionTitle: 'Migration Complete',
    completionDesc: 'The following tools have been migrated. Please restart them to apply changes:',
    completionHint: 'Tip: Close and reopen the above software to automatically load caches from the new path',
    gotIt: 'Got it',

    // Update modal
    downloadingUpdate: 'Downloading Update',
    downloadingVersion: 'Downloading version {v}...',
    newVersion: 'New Version Available',
    currentVersion: 'Current: ',
    latestVersion: 'Latest: ',
    releaseDate: 'Released: ',
    publisher: 'Publisher: ',
    later: 'Later',
    upgradeNow: 'Upgrade Now',

    // Supported tools modal
    supportedToolsTitle: 'Supported Software ({n})',

    // Toasts
    scanComplete: 'Scan complete, found {n} tools',
    scanFailed: 'Scan failed',
    folderSelectUnavailable: 'Folder picker unavailable, please enter path manually',
    openFolderFailed: 'Failed to open folder',
    adminRestart: 'Please restart the program as administrator',
    adminFailed: 'Privilege escalation failed',
    checkFailed: 'Check failed',
    migrateFailed: 'Migration failed',
    rollbackFailed: 'Rollback failed',
    logsExported: 'Logs exported',
    exportUnavailable: 'Export unavailable',
    loadLogsFailed: 'Failed to load logs',
    upToDate: 'Already up to date',
    downloadComplete: 'Download complete, installing...',
    updateFailed: 'Update failed',
    hashVerifyFailed: 'Update package verification failed, please retry',
  },
};

function t(key: string, params?: Record<string, string | number>): string {
  const lang = currentLang.value;
  let text = translations[lang][key] || translations['zh'][key] || key;
  if (params) {
    for (const [k, v] of Object.entries(params)) {
      text = text.replace(`{${k}}`, String(v));
    }
  }
  return text;
}

function toggleLanguage() {
  currentLang.value = currentLang.value === 'zh' ? 'en' : 'zh';
}

interface DetectedTool {
  id: string; name: string; category: string; icon: string;
  installed: boolean; size: number; size_formatted: string;
  paths: string[]; strategy: string; description: string; risk_level: string;
}
interface MigrationProgress {
  tool_id: string; tool_name: string; step: string;
  progress: number; message: string; speed: string; eta: string;
}
interface RollbackProgress {
  tool_id: string; tool_name: string;
  progress: number; message: string; speed: string; eta: string;
}
interface PreCheckResult {
  tool_id: string; tool_name: string; risk_level: string;
  processes_running: string[]; disk_space_ok: boolean;
  disk_free: string; disk_needed: string; can_migrate: boolean;
  warnings: string[];
}
interface ToolGroup { category: string; tools: DetectedTool[] }
interface ToolBrief { id: string; name: string; category: string; icon: string; description: string }

const tools = ref<DetectedTool[]>([]);
const selectedTools = ref<string[]>([]);
const migratedTools = ref<string[]>([]);
const toolStatusMap = ref<Record<string, number>>({});
const scanning = ref(false);
const scanProgress = ref({ tool_name: '', current: 0, total: 0, found_count: 0, found_size: 0 });
const reactiveFoundCount = ref(0);
const reactiveFoundSize = ref(0);
const migrating = ref(false);
const cDiskFree = ref(0);

const allCategories = ref<string[]>([]);

const categories = computed(() => {
  // 优先使用从 tool_map 加载的所有分类，其次使用扫描结果中的分类
  if (allCategories.value.length > 0) return allCategories.value;
  return [...new Set(tools.value.map(t => t.category))];
});

const cDiskInfo = computed(() => {
  const bytes = cDiskFree.value;
  const gb = bytes / 1073741824;
  const display = bytes >= 1073741824
    ? gb.toFixed(2) + ' GB'
    : (bytes / 1048576).toFixed(0) + ' MB';
  let level: 'low' | 'moderate' | 'good' = 'good';
  let labelKey = 'cDiskStatusGood';
  if (gb < 5) { level = 'low'; labelKey = 'cDiskStatusLow'; }
  else if (gb < 20) { level = 'moderate'; labelKey = 'cDiskStatusModerate'; }
  const totalGB = 500;
  const usedPercent = Math.min(100, Math.round((1 - gb / totalGB) * 100));
  return { display, level, label: t(labelKey), usedPercent };
});

async function loadCDiskFree() {
  try { cDiskFree.value = await invoke('get_c_disk_free'); } catch {}
}
const targetPath = ref('D:\\DevCache');
const migrationProgress = ref<MigrationProgress | null>(null);
const showRollback = ref(false);
const showLogs = ref(false);
const showCompletion = ref(false);
const showPreCheck = ref(false);
const preChecking = ref(false);
const preCheckResults = ref<PreCheckResult[]>([]);
const forceMigrateIds = ref<string[]>([]);
const logs = ref('');
const selectedRollback = ref<string[]>([]);
const rollbackProgress = ref<RollbackProgress | null>(null);
const isRollingBack = ref(false);
const completedToolNames = ref<string[]>([]);
const isAdmin = ref(false);
const collapsedCategories = ref<string[]>([]);
const filterMode = ref<'all' | 'installed' | 'uninstalled'>('all');
const activeCategory = ref('');
const toast = ref<{ message: string; type: 'success' | 'error' | 'info' } | null>(null);

// 更新相关
const updateChecking = ref(false);
const showUpdatePrompt = ref(false);
const updateDownloading = ref(false);
const updateInfo = ref<{ version: string; version_code: string; url: string; date: string; hash: string; author: string; has_update: boolean; local_version: string } | null>(null);
const downloadProgress = ref({ downloaded: 0, total: 0, percent: 0 });
const downloadPercent = computed(() => downloadProgress.value.percent);

// 已支持软件列表
const showSupportedTools = ref(false);
const allToolDefinitions = ref<ToolBrief[]>([]);
const stActiveCategory = ref('');

const stCategories = computed(() => {
  return [...new Set(allToolDefinitions.value.map(t => t.category))];
});

const stCategoryCounts = computed(() => {
  const counts: Record<string, number> = {};
  for (const t of allToolDefinitions.value) {
    counts[t.category] = (counts[t.category] || 0) + 1;
  }
  return counts;
});

const filteredSupportedTools = computed(() => {
  if (!stActiveCategory.value) return allToolDefinitions.value;
  return allToolDefinitions.value.filter(t => t.category === stActiveCategory.value);
});

async function openSupportedToolsDialog() {
  if (allToolDefinitions.value.length === 0) {
    try {
      allToolDefinitions.value = await invoke('get_all_tool_definitions') as ToolBrief[];
    } catch {}
  }
  stActiveCategory.value = '';
  showSupportedTools.value = true;
}

function getToolEmojiById(icon: string): string {
  const m: Record<string, string> = {
    'npm': '', 'yarn': '🧶', 'pnpm': '', 'vite_turbo_nx': '⚡',
    'rust': '🦀', 'maven': '', 'gradle': '🐘', 'pip': '🐍',
    'anaconda': '🐍', 'go': '', 'nuget': '📦', 'conan': '📦',
    'vscode': '💻', 'jetbrains': '🧠', 'visual_studio': '💜',
    'android_studio': '', 'flutter': '🦋', 'unity': '🎮', 'unreal': '🎮',
    'docker': '', 'wsl': '🐧', 'vagrant': '📦', 'virtualbox': '💿',
    'system_cache': '', 'mingw': '⚙', 'local_db': '🗄',
    'cursor': '🖊', 'claude': '', 'qoderwork': '🧠', 'ollama': '🦙',
    'lm_studio': '🤖', 'huggingface': '🤗', 'stable_diffusion': '🎨',
    'github_copilot': '🤝', 'claw': '',
    'openclaw': '', 'qclaw': '', 'kimi_claw': '🦞',
    'workbuddy': '🤝', 'qoderworkcn': '🧠', 'toclaw': '',
    'autoclaw': '🦞', 'copaw': '', 'stepclaw': '🦞',
    'hbuilderx': '🔨', 'sublime_text': '📝', 'notepad_plus': '📄',
    'spotify': '🎵', 'netease_music': '🎶', 'qq_music': '',
    'kugou': '', 'kuwo': '🎧',
    'potplayer': '🎬', 'vlc': '', 'obs': '🎥', 'windows_terminal': '⌨',
    'davinci_resolve': '🎨', 'bijian': '✂', 'capcut': '✂', 'handbrake': '🔄',
    'lightroom': '📷', 'illustrator': '🖌', 'indesign': '', 'audition': '🎙',
    'affinity_photo': '🖼', 'affinity_designer': '🎯', 'coreldraw': '🖊',
    'meitu': '💄', 'capture_one': '📸', 'luminar': '',
  };
  return m[icon] || '📦';
}

const categoryCounts = computed(() => {
  const counts: Record<string, number> = {};
  for (const t of tools.value) {
    if (t.installed) {
      counts[t.category] = (counts[t.category] || 0) + 1;
    }
  }
  return counts;
});

const scanPercent = computed(() => {
  if (scanProgress.value.total === 0) return 0;
  return Math.round((scanProgress.value.current / scanProgress.value.total) * 100);
});

const scanToolEmojiMap: Record<string, string> = {
  'npm': '📦', 'yarn': '🧶', 'pnpm': '📦', 'nvm': '🔢', 'pip': '🐍',
  'anaconda': '🐍', 'pyenv': '🐍', 'poetry': '📝', 'maven': '🏗️', 'gradle': '🐘',
  'android': '🤖', 'jetbrains': '🧠', 'vscode': '💻', 'cursor': '🖊️',
  'flutter': '🦋', 'nuget': '📦', 'conan': '📦', 'git': '🔀', 'sourcetree': '🔀',
  'gitkraken': '🔀', 'terraform': '🏗️', 'postman': '📮', 'insomnia': '🌙',
  'ollama': '🦙', 'lm studio': '🤖', 'hugging': '🤗', 'copilot': '🤝',
  'unity': '🎮', 'unreal': '🎮', 'vim': '✏️', 'mysql': '🗄️', 'sql server': '🗄️',
  'postgresql': '🐘', 'redis': '🔴', 'elasticsearch': '🔍', 'rabbitmq': '🐇',
  'nginx': '🌐', 'apache': '🌐', 'xampp': '⚙️', 'docker': '🐳', 'wsl': '🐧',
  'iis': '🌐', 'rust': '🦀', 'go': '🔵', 'mingw': '⚙️', 'adobe': '🎨',
  'stable diffusion': '🎨', 'blender': '🎨', 'figma': '🎨', 'jianying': '🎬',
  'wps': '📄', 'onedrive': '☁️', 'baidu': '☁️', 'chrome': '🌐',
  'claude': '🤖', 'qoderwork': '🧠', 'virtualbox': '💿', 'vagrant': '📦',
  'hyper': '💿', 'emulator': '📱', 'cache': '📁', 'db': '🗄️',
  'filebeat': '📊', 'prometheus': '📊', 'localstack': '☁️',
};
const currentScanEmoji = computed(() => {
  const name = scanProgress.value.tool_name.toLowerCase();
  for (const [key, emoji] of Object.entries(scanToolEmojiMap)) {
    if (name.includes(key)) return emoji;
  }
  return '📦';
});

const overallProgress = ref({ done: 0, total: 0 });
const overallPercent = computed(() => {
  if (overallProgress.value.total === 0) return 0;
  return Math.round((overallProgress.value.done / overallProgress.value.total) * 100);
});

const installedCount = computed(() => tools.value.filter(t => t.installed).length);

const filteredTools = computed(() => {
  let result = tools.value;
  // 安装状态筛选
  if (filterMode.value === 'installed') result = result.filter(t => t.installed);
  if (filterMode.value === 'uninstalled') result = result.filter(t => !t.installed);
  // 分类筛选
  if (activeCategory.value) result = result.filter(t => t.category === activeCategory.value);
  return result;
});

const filteredGroupedTools = computed<ToolGroup[]>(() => {
  const map = new Map<string, DetectedTool[]>();
  for (const t of filteredTools.value) {
    if (!map.has(t.category)) map.set(t.category, []);
    map.get(t.category)!.push(t);
  }
  return Array.from(map.entries()).map(([category, tools]) => ({ category, tools }));
});


function formatSize(bytes: number): string {
  if (bytes >= 1073741824) return (bytes / 1073741824).toFixed(2) + ' GB';
  if (bytes >= 1048576) return (bytes / 1048576).toFixed(2) + ' MB';
  if (bytes >= 1024) return (bytes / 1024).toFixed(2) + ' KB';
  return bytes + ' B';
}

const scanSizeFormatted = computed(() => formatSize(reactiveFoundSize.value));

const totalInstalledSize = computed(() => {
  return formatSize(tools.value.filter(t => t.installed).reduce((s, t) => s + t.size, 0));
});

const canProceedMigration = computed(() => {
  return preCheckResults.value.every(r => r.can_migrate || forceMigrateIds.value.includes(r.tool_id));
});

function getToolEmoji(tool: DetectedTool): string {
  const m: Record<string, string> = {
    'cursor': '🖊', 'claude': '', 'qoderwork': '🧠', 'ollama': '🦙', 'lm_studio': '🤖', 'huggingface': '🤗',
    'stable_diffusion': '🎨', 'github_copilot': '🤝', 'npm': '', 'yarn': '🧶', 'pnpm': '📦',
    'vite_turbo_nx': '⚡', 'rust': '🦀', 'maven': '🏗', 'gradle': '🐘', 'pip': '🐍',
    'anaconda': '🐍', 'go': '🔵', 'nuget': '📦', 'conan': '📦', 'vscode': '💻',
    'jetbrains': '🧠', 'visual_studio': '💜', 'android_studio': '', 'flutter': '🦋',
    'unity': '🎮', 'unreal': '🎮', 'docker': '', 'wsl': '🐧', 'vagrant': '📦',
    'virtualbox': '💿', 'system_cache': '', 'mingw': '⚙', 'local_db': '🗄',
    'claw': '🦞',
    'hbuilderx': '🔨', 'sublime_text': '📝', 'notepad_plus': '📄',
    'spotify': '🎵', 'netease_music': '🎶', 'qq_music': '🎼', 'kugou': '🎤', 'kuwo': '🎧',
    'potplayer': '🎬', 'vlc': '', 'obs': '🎥', 'windows_terminal': '⌨',
    'davinci_resolve': '🎨', 'bijian': '✂', 'capcut': '✂', 'handbrake': '🔄',
    'lightroom': '📷', 'illustrator': '🖌', 'indesign': '📐', 'audition': '🎙',
    'affinity_photo': '🖼', 'affinity_designer': '🎯', 'coreldraw': '🖊', 'meitu': '💄',
    'capture_one': '📸', 'luminar': '🌟',
  };
  return m[tool.id] || '';
}

function getToolNameById(id: string): string {
  return tools.value.find(t => t.id === id)?.name || id;
}

function toggleCategory(cat: string) {
  const idx = collapsedCategories.value.indexOf(cat);
  if (idx >= 0) collapsedCategories.value.splice(idx, 1);
  else collapsedCategories.value.push(cat);
}

function groupHasVirtualDisk(group: ToolGroup): boolean {
  return group.tools.some(t => t.strategy === '虚拟磁盘专属');
}

async function scanTools() {
  scanning.value = true;
  reactiveFoundCount.value = 0;
  reactiveFoundSize.value = 0;
  try {
    tools.value = await invoke('scan_all_tools');
    migratedTools.value = await invoke('get_migrated_tool_ids');
    isAdmin.value = await invoke('check_admin');
    reactiveFoundCount.value = tools.value.filter(t => t.installed).length;
    reactiveFoundSize.value = tools.value.filter(t => t.installed).reduce((s, t) => s + t.size, 0);
    await loadToolStatus();
    showToast(t('scanComplete', { n: installedCount.value }), 'success');
    filterMode.value = 'installed';
  } catch (e) {
    showToast(t('scanFailed') + ': ' + e, 'error');
  } finally { scanning.value = false; }
}

async function loadToolStatus() {
  try {
    const statuses = await invoke('get_all_tool_status') as Array<{ tool_id: string; migrate_count: number }>;
    const map: Record<string, number> = {};
    for (const s of statuses) {
      if (s.migrate_count > 0) map[s.tool_id] = s.migrate_count;
    }
    toolStatusMap.value = map;
  } catch { /* ignore */ }
  await reloadMigratedTools();
}

/// 从 DB 实时加载已迁移工具列表
async function reloadMigratedTools() {
  try {
    migratedTools.value = await invoke('get_migrated_tool_ids');
  } catch { /* ignore */ }
}

async function openRollbackDialog() {
  await reloadMigratedTools();
  selectedRollback.value = [];
  showRollback.value = true;
}

function selectAll() {
  selectedTools.value = tools.value.filter(t => t.installed && !migratedTools.value.includes(t.id)).map(t => t.id);
}
function deselectAll() { selectedTools.value = []; }

async function browseFolder() {
  try {
    const { open } = await import('@tauri-apps/plugin-dialog');
    const selected = await open({ directory: true, multiple: false, title: '选择迁移目标目录' });
    if (selected) targetPath.value = selected as string;
  } catch {
    showToast(t('folderSelectUnavailable'), 'info');
  }
}

function openTargetDir() {
  if (!targetPath.value) return;
  invoke('open_directory', { path: targetPath.value });
}

async function openToolLocation(tool: DetectedTool) {
  if (!tool.paths || tool.paths.length === 0) return;
  try {
    await invoke('open_tool_location', { pathTemplate: tool.paths[0] });
  } catch (e) {
    showToast(t('openFolderFailed') + ': ' + e, 'error');
  }
}

async function requestAdmin() {
  if (!confirm(t('confirmRequestAdmin'))) return;
  try {
    await invoke('request_admin_cmd');
    showToast(t('adminRestart'), 'info');
  } catch (e) {
    showToast(t('adminFailed') + ': ' + e, 'error');
  }
}

// 前置条件检测 → 弹确认框
async function preCheckAndMigrate() {
  if (selectedTools.value.length === 0 || !targetPath.value) return;
  preChecking.value = true;
  showPreCheck.value = true;
  forceMigrateIds.value = [];
  try {
    preCheckResults.value = await invoke('check_preconditions', {
      toolIds: selectedTools.value,
      targetPath: targetPath.value,
    });
  } catch (e) {
    showToast(t('checkFailed') + ': ' + e, 'error');
    showPreCheck.value = false;
  } finally {
    preChecking.value = false;
  }
}

// 确认后执行迁移
async function confirmAndMigrate() {
  if (!canProceedMigration.value) return;
  showPreCheck.value = false;

  // 只迁移 can_migrate 或 force 的
  const toMigrate = preCheckResults.value
    .filter(r => r.can_migrate || forceMigrateIds.value.includes(r.tool_id))
    .map(r => r.tool_id);

  if (toMigrate.length === 0) return;

  migrating.value = true;
  completedToolNames.value = [];
  overallProgress.value = { done: 0, total: toMigrate.length };
  const names: string[] = [];

  try {
    for (const toolId of toMigrate) {
      try {
        const result = await invoke('migrate_tool_cmd', { toolId, targetPath: targetPath.value });
        names.push(getToolNameById(toolId));
        showToast(result as string, 'success');
      } catch (e) {
        showToast(`${getToolNameById(toolId)} ${t('migrateFailed')}: ${e}`, 'error');
      }
      overallProgress.value.done++;
    }
    completedToolNames.value = names;
    await reloadMigratedTools();
    await loadToolStatus();
    await loadCDiskFree(); // 实时更新 C 盘空间
    selectedTools.value = [];
    if (names.length > 0) showCompletion.value = true;
  } finally {
    migrating.value = false;
    migrationProgress.value = null;
  }
}

async function doRollback() {
  if (selectedRollback.value.length === 0) return;
  isRollingBack.value = true;
  rollbackProgress.value = null;
  try {
    for (const toolId of selectedRollback.value) {
      try {
        const result = await invoke('rollback_tool_cmd', { toolId });
        showToast(result as string, 'success');
        // 每个工具回滚完成后立即更新计数
        await reloadMigratedTools();
      } catch (e) {
        showToast(`${getToolNameById(toolId)} ${t('rollbackFailed')}: ${e}`, 'error');
      }
    }
    await loadToolStatus();
    await loadCDiskFree(); // 实时更新 C 盘空间
    selectedRollback.value = [];
    if (!isRollingBack.value) return; // 已被取消
    showRollback.value = false;
  } catch (e) {
    showToast(t('rollbackFailed') + ': ' + e, 'error');
  } finally {
    isRollingBack.value = false;
    rollbackProgress.value = null;
  }
}

async function loadLogs() {
  try { logs.value = await invoke('get_all_logs'); }
  catch (e) { logs.value = t('loadLogsFailed') + ': ' + e; }
}

async function exportLogs() {
  try {
    const { save } = await import('@tauri-apps/plugin-dialog');
    const { writeTextFile } = await import('@tauri-apps/plugin-fs');
    const path = await save({ defaultPath: 'migration_log.txt', filters: [{ name: '日志文件', extensions: ['txt', 'log'] }] });
    if (path) {
      await writeTextFile(path, logs.value);
      showToast(t('logsExported'), 'success');
    }
  } catch {
    showToast(t('exportUnavailable'), 'info');
  }
}

function showToast(message: string, type: 'success' | 'error' | 'info') {
  toast.value = { message, type };
  setTimeout(() => { toast.value = null; }, 4000);
}

// ==== 更新相关函数 ====
function formatDownloadSize(bytes: number): string {
  if (bytes >= 1073741824) return (bytes / 1073741824).toFixed(2) + ' GB';
  if (bytes >= 1048576) return (bytes / 1048576).toFixed(2) + ' MB';
  if (bytes >= 1024) return (bytes / 1024).toFixed(2) + ' KB';
  return bytes + ' B';
}

async function checkForUpdate() {
  if (updateChecking.value) return;
  updateChecking.value = true;
  try {
    const info = await invoke('check_update') as any;
    updateInfo.value = info;
    if (info.has_update) {
      showUpdatePrompt.value = true;
    } else {
      showToast(t('upToDate'), 'success');
    }
  } catch (e) {
    showToast(t('upToDate'), 'success');
  } finally {
    updateChecking.value = false;
  }
}

function dismissUpdate() {
  showUpdatePrompt.value = false;
  updateInfo.value = null;
}

async function startDownloadUpdate() {
  if (!updateInfo.value) return;
  showUpdatePrompt.value = false;
  updateDownloading.value = true;
  downloadProgress.value = { downloaded: 0, total: 0, percent: 0 };
  try {
    const filepath = await invoke('download_update', { url: updateInfo.value.url }) as string;
    // 验证哈希
    if (updateInfo.value.hash) {
      const ok = await invoke('verify_hash', { filepath, expectedHash: updateInfo.value.hash }) as boolean;
      if (!ok) {
        showToast(t('hashVerifyFailed'), 'error');
        updateDownloading.value = false;
        return;
      }
    }
    showToast(t('downloadComplete'), 'info');
    // 稍等让toast显示
    await new Promise(r => setTimeout(r, 1500));
    await invoke('install_update', { filepath });
  } catch (e) {
    showToast(t('updateFailed') + ': ' + e, 'error');
    updateDownloading.value = false;
  }
}

onMounted(async () => {
  // 禁用右键菜单（仅保留刷新）
  document.addEventListener('contextmenu', (e) => {
    e.preventDefault();
    // 创建一个简单的自定义右键菜单，只有刷新选项
    let menu = document.getElementById('custom-context-menu');
    if (menu) menu.remove();
    menu = document.createElement('div');
    menu.id = 'custom-context-menu';
    menu.style.cssText = `position:fixed;left:${e.clientX}px;top:${e.clientY}px;background:#fff;border:1px solid #ddd;border-radius:6px;padding:4px 0;z-index:9999;box-shadow:0 2px 8px rgba(0,0,0,0.15);font-size:14px;min-width:120px;`;
    const refreshItem = document.createElement('div');
    refreshItem.textContent = '🔄 刷新';
    refreshItem.style.cssText = 'padding:6px 16px;cursor:pointer;';
    refreshItem.onmouseenter = () => refreshItem.style.background = '#f0f0f0';
    refreshItem.onmouseleave = () => refreshItem.style.background = 'transparent';
    refreshItem.onclick = () => { location.reload(); };
    menu.appendChild(refreshItem);
    document.body.appendChild(menu);
    const closeMenu = (ev: MouseEvent) => {
      if (!menu!.contains(ev.target as Node)) { menu!.remove(); document.removeEventListener('click', closeMenu); }
    };
    setTimeout(() => document.addEventListener('click', closeMenu), 0);
  });

  await listen<MigrationProgress>('migration-progress', (event) => {
    migrationProgress.value = event.payload;
  });
  await listen<RollbackProgress>('rollback-progress', (event) => {
    rollbackProgress.value = event.payload;
  });
  await listen<{ tool_name: string; current: number; total: number; found_count: number; found_size: number }>('scan-progress', (event) => {
    const p = event.payload;
    scanProgress.value = { tool_name: p.tool_name, current: p.current, total: p.total, found_count: p.found_count, found_size: p.found_size };
    reactiveFoundCount.value = p.found_count;
    reactiveFoundSize.value = p.found_size;
  });
  try { await invoke('check_system_version'); } catch {}
  await loadCDiskFree();
  // 30 秒定时刷新 C 盘空间
  setInterval(loadCDiskFree, 30000);
  // 加载所有分类（不依赖扫描结果）
  try {
    const cats = await invoke('get_all_categories') as string[];
    allCategories.value = cats;
  } catch {}
  // 加载持久化的工具状态
  await loadToolStatus();
  // 自动检测最优路径
  try {
    const best = await invoke('get_best_target_path') as string;
    targetPath.value = best;
  } catch {}
  // 监听下载进度
  await listen<{ downloaded: number; total: number; percent: number }>('update:download-progress', (event) => {
    downloadProgress.value = event.payload;
  });
});
watch(showLogs, async (v) => { if (v) await loadLogs(); });
</script>

<style>
* { margin: 0; padding: 0; box-sizing: border-box; }
body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Microsoft YaHei', sans-serif; background: #f5f5f5; color: #333333; }
.app { height: 100vh; display: flex; flex-direction: column; overflow: hidden; }

/* Header */
.header { background: linear-gradient(135deg, #00C67A 0%, #00A86B 100%); color: white; padding: 16px 24px; display: flex; justify-content: space-between; align-items: center; }
.header h1 { font-size: 20px; font-weight: 600; }
.subtitle { font-size: 14px; opacity: 0.85; margin-top: 2px; }
.header-center { display: flex; align-items: center; }
.c-disk-widget { display: flex; align-items: center; gap: 12px; padding: 8px 16px; border-radius: 12px; border: 1px solid; transition: all 0.3s; }
.c-disk-icon { display: flex; align-items: center; justify-content: center; width: 32px; height: 32px; border-radius: 8px; background: rgba(255,255,255,0.2); }
.c-disk-icon svg { width: 20px; height: 20px; }
.c-disk-content { display: flex; flex-direction: column; gap: 4px; }
.c-disk-top { display: flex; align-items: baseline; gap: 8px; }
.c-disk-label { font-size: 12px; opacity: 0.85; color: #ffffff; }
.c-disk-value { font-size: 15px; font-weight: 700; color: #ffffff; }
.c-disk-bar-wrap { display: flex; align-items: center; gap: 8px; }
.c-disk-bar { width: 80px; height: 6px; border-radius: 3px; background: rgba(255,255,255,0.2); overflow: hidden; }
.c-disk-bar-fill { height: 100%; border-radius: 3px; transition: width 0.5s ease; }
.c-disk-status-badge { font-size: 11px; font-weight: 600; padding: 2px 8px; border-radius: 10px; }
.c-disk-good { background: rgba(0,200,83,0.15); border-color: rgba(0,200,83,0.4); }
.c-disk-good .c-disk-bar-fill { background: #00C853; }
.c-disk-good .c-disk-status-badge { background: rgba(0,200,83,0.3); color: #ffffff; }
.c-disk-moderate { background: rgba(33,150,243,0.15); border-color: rgba(33,150,243,0.4); }
.c-disk-moderate .c-disk-bar-fill { background: #2196F3; }
.c-disk-moderate .c-disk-status-badge { background: rgba(33,150,243,0.3); color: #ffffff; }
.c-disk-low { background: rgba(244,67,54,0.15); border-color: rgba(244,67,54,0.4); }
.c-disk-low .c-disk-bar-fill { background: #F44336; }
.c-disk-low .c-disk-status-badge { background: rgba(244,67,54,0.3); color: #ffffff; }
.header-right { display: flex; align-items: center; gap: 10px; }
.admin-badge { display: inline-flex; align-items: center; gap: 6px; padding: 3px 12px; border-radius: 12px; font-size: 14px; background: rgba(255,255,255,0.2); cursor: default; user-select: none; }
.admin-badge:not(.admin) { cursor: pointer; }
.admin-badge:not(.admin):hover { background: rgba(255,255,255,0.35); }
.admin-badge.admin { background: rgba(0,198,122,0.6); }
.admin-icon { font-size: 14px; }
.admin-toggle-icon { font-size: 11px; opacity: 0.7; margin-left: 2px; }
.btn-lang { padding: 4px 10px; border: 1px solid rgba(255,255,255,0.5); border-radius: 8px; background: rgba(255,255,255,0.1); color: white; font-size: 13px; font-weight: 600; cursor: pointer; transition: background 0.15s; white-space: nowrap; }
.btn-lang:hover { background: rgba(255,255,255,0.25); }
.btn-header { padding: 5px 12px; border: 1px solid rgba(255,255,255,0.5); border-radius: 8px; background: transparent; color: white; font-size: 14px; cursor: pointer; }
.btn-header:hover { background: rgba(255,255,255,0.15); }

/* Layout */
.main-content { display: flex; flex: 1; gap: 16px; padding: 16px; overflow: hidden; }
.tool-panel { flex: 1; background: white; border-radius: 8px; box-shadow: 0 2px 8px 0 rgba(0,0,0,0.1); display: flex; flex-direction: column; overflow: hidden; }
.panel-header { padding: 14px 18px; border-bottom: 1px solid #e0e0e0; display: flex; justify-content: space-between; align-items: center; }
.panel-header h2 { font-size: 16px; font-weight: 600; }
.actions { display: flex; gap: 6px; }

/* Filter tabs */
.filter-tabs { display: flex; gap: 2px; padding: 8px 18px; border-bottom: 1px solid #e0e0e0; background: #f5f5f5; }
.filter-tab { padding: 5px 14px; border-radius: 8px; font-size: 14px; color: #666666; cursor: pointer; transition: all 0.15s; user-select: none; }
.filter-tab:hover { background: #f1f1f1; color: #333333; }
.filter-tab.active { background: linear-gradient(135deg, #00C67A 0%, #00A86B 100%); color: white; font-weight: 500; }

/* Category filter bar */
.category-filter-bar { display: flex; gap: 4px; padding: 6px 18px; border-bottom: 1px solid #e0e0e0; background: #fff; flex-wrap: wrap; }
.cat-filter-tab { padding: 3px 12px; border-radius: 14px; font-size: 14px; color: #666666; cursor: pointer; transition: all 0.15s; user-select: none; white-space: nowrap; border: 1px solid #e0e0e0; }
.cat-filter-tab:hover { background: #f1f1f1; color: #333333; border-color: #e0e0e0; }
.cat-filter-tab.active { background: #00C67A; color: white; border-color: #00C67A; font-weight: 500; }

/* Skeleton */
.skeleton-list { padding: 12px 18px; }
.skeleton-item { display: flex; align-items: flex-start; padding: 12px 8px; margin-bottom: 4px; }
.skeleton-icon { width: 36px; height: 36px; border-radius: 8px; background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%); background-size: 200% 100%; animation: shimmer 1.5s infinite; margin-right: 12px; flex-shrink: 0; }
.skeleton-info { flex: 1; }
.skeleton-line { height: 12px; border-radius: 4px; background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%); background-size: 200% 100%; animation: shimmer 1.5s infinite; margin-bottom: 8px; }
.skeleton-line.w60 { width: 60%; }
.skeleton-line.w40 { width: 40%; }
.skeleton-line.w25 { width: 25%; }
@keyframes shimmer { 0% { background-position: 200% 0; } 100% { background-position: -200% 0; } }

/* Welcome state */
.welcome-state { display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 40px 30px; text-align: center; flex: 1; }
.welcome-animation { position: relative; width: 120px; height: 120px; margin-bottom: 20px; display: flex; align-items: center; justify-content: center; }
.welcome-main-icon { font-size: 64px; animation: welcome-bounce 2s ease-in-out infinite; position: relative; z-index: 2; filter: drop-shadow(0 4px 12px rgba(0,198,122,0.3)); }
.floating-icons { position: absolute; width: 100%; height: 100%; top: 0; left: 0; pointer-events: none; }
.float-icon { position: absolute; font-size: 20px; animation: float-up 3s ease-in-out infinite; opacity: 0.6; }
.float-icon:nth-child(1) { top: 0; left: 50%; transform: translateX(-50%); }
.float-icon:nth-child(2) { top: 20%; right: 0; }
.float-icon:nth-child(3) { bottom: 10%; left: 5%; }
.float-icon:nth-child(4) { top: 30%; left: 0; }
.float-icon:nth-child(5) { bottom: 30%; right: 5%; }
.float-icon:nth-child(6) { top: 50%; right: 0; }
.float-icon:nth-child(7) { bottom: 0; left: 40%; }
.float-icon:nth-child(8) { bottom: 20%; right: 15%; }
@keyframes welcome-bounce { 0%,100% { transform: translateY(0) scale(1); } 50% { transform: translateY(-10px) scale(1.05); } }
@keyframes float-up { 0%,100% { transform: translateY(0) rotate(0deg); opacity: 0.6; } 50% { transform: translateY(-15px) rotate(10deg); opacity: 1; } }
.welcome-title { font-size: 28px; font-weight: 700; margin-bottom: 10px; background: linear-gradient(135deg, #00C67A 0%, #00A86B 100%); -webkit-background-clip: text; -webkit-text-fill-color: transparent; background-clip: text; }
.welcome-desc { font-size: 16px; color: #666666; margin-bottom: 6px; }
.welcome-desc strong { color: #dc2626; }
.welcome-sub { font-size: 14px; color: #999999; margin-bottom: 24px; }
.btn-welcome { padding: 14px 36px; font-size: 18px; border-radius: 12px; margin-bottom: 24px; animation: pulse-glow 2s ease-in-out infinite; }
@keyframes pulse-glow { 0%,100% { box-shadow: 0 4px 12px rgba(0,198,122,0.4); } 50% { box-shadow: 0 6px 24px rgba(0,198,122,0.6); } }
.welcome-features { display: flex; flex-direction: column; gap: 8px; font-size: 14px; color: #666666; }
.welcome-features span { padding: 4px 0; }

/* Scan progress */
.scan-progress-section { padding: 16px 18px; }
.scan-progress-card { background: #f0f9f4; border: 1px solid #d1f0e4; border-radius: 10px; padding: 14px 16px; margin-bottom: 16px; }
.scan-progress-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px; }
.scan-progress-title { display: flex; align-items: center; gap: 8px; font-size: 14px; color: #333333; }
.scan-current-icon { font-size: 22px; animation: pulse-icon 1s ease-in-out infinite; }
@keyframes pulse-icon { 0%,100% { transform: scale(1); } 50% { transform: scale(1.15); } }
.scan-progress-badge { background: linear-gradient(135deg, #00C67A 0%, #00A86B 100%); color: white; padding: 2px 10px; border-radius: 12px; font-size: 13px; font-weight: 600; }
.scan-bar { height: 8px !important; border-radius: 4px !important; }
.scan-progress-fill { height: 100%; background: linear-gradient(90deg, #00C67A 0%, #00A86B 100%); border-radius: 4px; transition: width 0.3s ease; background-size: 200% 100%; animation: progress-shimmer 1.5s infinite; }
@keyframes progress-shimmer { 0% { background-position: 200% 0; } 100% { background-position: -200% 0; } }
.scan-progress-info { display: flex; justify-content: space-between; margin-top: 8px; font-size: 13px; color: #666666; }
.scan-percent { font-weight: 600; color: #00C67A; }
.scan-found-row { display: flex; align-items: center; gap: 4px; margin-top: 8px; padding-top: 8px; border-top: 1px solid #d1f0e4; font-size: 14px; color: #333333; }
.scan-found-count { font-weight: 700; color: #059669; font-size: 18px; transition: all 0.2s; }
.scan-found-size { font-weight: 600; color: #00C67A; }
.scan-skeleton-list { padding: 0; }
.scan-skeleton-list .skeleton-item { padding: 10px 4px; }

/* Category grouping */
.category-header { display: flex; align-items: center; gap: 8px; padding: 10px 18px; background: #f5f5f5; border-bottom: 1px solid #f1f1f1; cursor: pointer; user-select: none; position: sticky; top: 0; z-index: 1; }
.category-header:hover { background: #f1f1f1; }
.collapse-icon { font-size: 14px; color: #999999; width: 14px; }
.category-name { font-weight: 600; font-size: 16px; }
.category-count { font-size: 14px; color: #666666; }
.risk-badge { margin-left: auto; font-size: 14px; color: #dc2626; background: #fef2f2; padding: 2px 8px; border-radius: 10px; font-weight: 500; }
.risk-warning { padding: 8px 18px; font-size: 14px; color: #dc2626; background: #fef2f2; border-bottom: 1px solid #fecaca; }
.category-body { padding: 4px 12px; }

/* Tool list */
.tool-list { flex: 1; overflow-y: auto; padding-bottom: 15px; }
.tool-item { display: flex; align-items: flex-start; padding: 10px 8px; border-radius: 8px; margin-bottom: 4px; transition: background 0.15s; }
.tool-item:hover { background: #f5f5f5; }
.tool-item.disabled { opacity: 0.45; }
.tool-item.migrated { background: #f0fdf4; }
.tool-item input[type="checkbox"] { margin-top: 6px; margin-right: 10px; width: 16px; height: 16px; cursor: pointer; flex-shrink: 0; }
.tool-icon { font-size: 22px; margin-right: 10px; flex-shrink: 0; filter: grayscale(1); opacity: 0.4; }
.tool-icon.active { filter: none; opacity: 1; }
.tool-info { flex: 1; min-width: 0; }
.tool-name-row { display: flex; align-items: center; gap: 8px; margin-bottom: 2px; }
.tool-name { font-weight: 600; font-size: 16px; }
.tool-desc { font-size: 14px; color: #666666; margin-bottom: 4px; }
.tool-meta { display: flex; gap: 10px; font-size: 14px; flex-wrap: wrap; }
.size { color: #059669; font-weight: 500; }
.status { color: #dc2626; }
.status.installed { color: #059669; }
.strategy { color: #666666; }
.migrated-tag { display: inline-flex; align-items: center; gap: 4px; padding: 2px 10px; border-radius: 12px; font-weight: 600; font-size: 13px; background: linear-gradient(135deg, #059669, #10b981); color: #fff; box-shadow: 0 1px 4px rgba(5,150,105,0.35); white-space: nowrap; }
.migrated-icon { font-size: 12px; font-weight: 700; }
.migrated-text { font-size: 13px; }
.migrated-count { background: rgba(255,255,255,0.3); padding: 0 6px; border-radius: 8px; font-size: 12px; font-weight: 700; margin-left: 2px; }
.btn-open-location { flex-shrink: 0; align-self: center; padding: 4px 10px; border: 1px solid #e0e0e0; border-radius: 8px; background: #f5f5f5; color: #333333; font-size: 14px; cursor: pointer; white-space: nowrap; margin-left: 8px; transition: all 0.15s; }
.btn-open-location:hover { background: #e0e0e0; border-color: #999999; }
.empty-state { text-align: center; padding: 60px 20px; color: #999999; }

/* Risk tags */
.risk-tag { font-size: 14px; padding: 1px 8px; border-radius: 10px; font-weight: 500; white-space: nowrap; }
.risk-tag.risk-high { color: #dc2626; background: #fef2f2; border: 1px solid #fecaca; }
.risk-tag.risk-medium { color: #d97706; background: #fffbeb; border: 1px solid #fde68a; }
.risk-tag.risk-low { color: #059669; background: #f0fdf4; border: 1px solid #bbf7d0; }

/* Action panel */
.action-panel { width: 360px; display: flex; flex-direction: column; gap: 12px; flex-shrink: 0; position: sticky; top: 16px; align-self: flex-start; max-height: calc(100vh - 100px); overflow-y: auto; }
.config-section, .progress-section, .stats-section { background: white; border-radius: 10px; padding: 14px 18px; box-shadow: 0 2px 8px 0 rgba(0,0,0,0.1); }
.config-section h3, .progress-section h3, .stats-section h3 { font-size: 16px; font-weight: 600; margin-bottom: 10px; }
.config-item label { display: block; font-size: 14px; margin-bottom: 4px; color: #666666; }
.path-row { display: flex; gap: 6px; }
.path-input { flex: 1; padding: 7px 10px; border: 1px solid #e0e0e0; border-radius: 8px; font-size: 14px; }
.btn-browse { padding: 7px 12px; background: #f1f1f1; border: 1px solid #e0e0e0; border-radius: 8px; font-size: 14px; cursor: pointer; white-space: nowrap; }
.btn-browse:hover { background: #e0e0e0; }
.config-hint { font-size: 14px; color: #999999; margin-top: 6px; }

/* Buttons */
.button-group { display: flex; flex-direction: column; gap: 6px; }
.btn-row { display: flex; gap: 6px; }
.btn-half { flex: 1; }
button { padding: 9px 14px; border: none; border-radius: 8px; font-size: 14px; font-weight: 500; cursor: pointer; transition: all 0.15s; }
button:disabled { opacity: 0.45; cursor: not-allowed; }
.btn-primary { background: linear-gradient(135deg, #00C67A 0%, #00A86B 100%); color: white; }
.btn-primary:hover:not(:disabled) { transform: translateY(-1px); box-shadow: 0 4px 12px rgba(0,198,122,0.4); }
.btn-large { padding: 12px 18px; font-size: 16px; }
.btn-secondary { background: #f1f1f1; color: #333333; }
.btn-secondary:hover:not(:disabled) { background: #e0e0e0; }
.btn-danger { background: #dc2626; color: white; }
.btn-danger:hover:not(:disabled) { background: #b91c1c; }

/* Progress */
.overall-progress { margin-bottom: 10px; }
.overall-header { display: flex; justify-content: space-between; margin-bottom: 4px; font-size: 14px; font-weight: 600; }
.current-tool-progress { margin-top: 8px; padding-top: 8px; border-top: 1px solid #f1f1f1; }
.progress-header { display: flex; justify-content: space-between; margin-bottom: 4px; font-size: 14px; }
.progress-bar { height: 6px; background: #e0e0e0; border-radius: 3px; overflow: hidden; margin-bottom: 4px; }
.progress-bar.bar-small { height: 4px; }
.progress-fill { height: 100%; background: linear-gradient(90deg, #00C67A 0%, #00A86B 100%); transition: width 0.3s; }
.progress-message { font-size: 14px; color: #666666; }
.progress-stats { display: flex; justify-content: space-between; font-size: 14px; color: #999999; margin-top: 2px; }

/* Stats */
.stat-item { display: flex; justify-content: space-between; padding: 6px 0; border-bottom: 1px solid #f1f1f1; font-size: 14px; }
.stat-item:last-child { border-bottom: none; }
.stat-label { color: #666666; }
.stat-value { font-weight: 600; color: #333333; }
.stat-live { color: #059669; animation: stat-pulse 1s ease-in-out infinite; }
@keyframes stat-pulse { 0%,100% { opacity: 1; } 50% { opacity: 0.6; } }

/* About section */
.about-section { background: white; border-radius: 10px; padding: 12px 18px; box-shadow: 0 2px 8px 0 rgba(0,0,0,0.1); font-size: 14px; color: #666666; line-height: 1.7; }
.about-logo-wrap { display: flex; align-items: center; gap: 10px; margin-bottom: 6px; }
.about-logo { width: 36px; height: 36px; border-radius: 8px; flex-shrink: 0; }
.about-brand { font-weight: 600; color: #333333; }
.about-line { display: flex; align-items: baseline; }
.about-line a { color: #00C67A; text-decoration: none; }
.about-line a:hover { text-decoration: underline; }
.al-label { display: inline-block; width: 72px; color: #666666; flex-shrink: 0; }
.btn-check-update { margin-left: 8px; padding: 1px 10px; border: 1px solid #00C67A; border-radius: 10px; background: transparent; color: #00C67A; font-size: 12px; cursor: pointer; white-space: nowrap; vertical-align: middle; transition: all 0.15s; }
.btn-check-update:hover { background: #00C67A; color: white; }
.btn-check-update:disabled { opacity: 0.5; cursor: not-allowed; }

/* Update dialog */
.update-info { margin: 10px 0; }
.update-info-row { display: flex; padding: 4px 0; font-size: 14px; color: #333333; }
.update-label { width: 80px; color: #666666; flex-shrink: 0; }
.update-version { color: #00C67A; font-weight: 700; font-size: 16px; }
.download-progress-wrap { padding: 14px 0; }
.download-info { display: flex; justify-content: space-between; margin-top: 6px; font-size: 14px; color: #666666; }

/* Modal */
.modal-overlay { position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.45); display: flex; align-items: center; justify-content: center; z-index: 1000; }
.modal { background: white; border-radius: 10px; padding: 20px; min-width: 400px; max-width: 580px; max-height: 80vh; display: flex; flex-direction: column; }
.modal-large { min-width: 680px; }
.modal-precheck { min-width: 520px; max-width: 640px; }
.modal h3 { font-size: 20px; margin-bottom: 12px; }
.modal-desc { font-size: 14px; color: #666666; margin-bottom: 12px; }
.modal-hint { font-size: 14px; color: #999999; margin-top: 10px; }
.modal-footer { display: flex; justify-content: flex-end; gap: 8px; margin-top: 16px; padding-top: 12px; border-top: 1px solid #e0e0e0; }

/* Config header button */
.config-header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 12px; }
.config-header h3 { margin-bottom: 0; }
.btn-supported-tools { background: linear-gradient(135deg, #6366f1, #8b5cf6); color: #fff; border: none; border-radius: 16px; padding: 4px 14px; font-size: 13px; font-weight: 600; cursor: pointer; white-space: nowrap; transition: transform 0.15s, box-shadow 0.15s; box-shadow: 0 2px 8px rgba(99,102,241,0.3); }
.btn-supported-tools:hover { transform: scale(1.05); box-shadow: 0 3px 12px rgba(99,102,241,0.45); }

/* Supported tools dialog */
.supported-tools-modal { min-width: 720px; max-width: 860px; max-height: 85vh; }
.supported-tools-header h3 { margin-bottom: 10px; }
.supported-tools-filter { display: flex; flex-wrap: wrap; gap: 6px; margin-bottom: 14px; padding-bottom: 10px; border-bottom: 1px solid #e8e8e8; }
.st-filter-tab { padding: 3px 12px; border-radius: 12px; font-size: 13px; cursor: pointer; background: #f0f0f0; color: #666; transition: all 0.15s; white-space: nowrap; }
.st-filter-tab:hover { background: #e0e0e0; }
.st-filter-tab.active { background: #6366f1; color: #fff; font-weight: 600; }
.supported-tools-grid { display: grid; grid-template-columns: repeat(3, 1fr); gap: 8px; max-height: 55vh; overflow-y: auto; padding-right: 4px; }
.st-tool-card { display: flex; align-items: center; gap: 8px; padding: 8px 10px; border-radius: 8px; background: #f8f9fa; transition: background 0.12s; }
.st-tool-card:hover { background: #eef0ff; }
.st-tool-icon { font-size: 20px; width: 28px; text-align: center; flex-shrink: 0; }
.st-tool-info { display: flex; flex-direction: column; min-width: 0; }
.st-tool-name { font-size: 14px; font-weight: 600; color: #222; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.st-tool-desc { font-size: 12px; color: #999; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }

/* Precheck */
.precheck-list { max-height: 360px; overflow-y: auto; display: flex; flex-direction: column; gap: 8px; }
.precheck-item { padding: 10px 12px; border-radius: 8px; border-left: 4px solid #e0e0e0; background: #f5f5f5; }
.precheck-item.risk-high { border-left-color: #dc2626; background: #fef2f2; }
.precheck-item.risk-medium { border-left-color: #d97706; background: #fffbeb; }
.precheck-item.risk-low { border-left-color: #059669; background: #f0fdf4; }
.precheck-header { display: flex; align-items: center; gap: 8px; margin-bottom: 6px; }
.precheck-name { font-weight: 600; font-size: 16px; flex: 1; }
.precheck-status { font-size: 14px; font-weight: 600; }
.precheck-status.ok { color: #059669; }
.precheck-status:not(.ok) { color: #dc2626; }
.precheck-details { display: flex; gap: 16px; font-size: 14px; color: #666666; margin-bottom: 4px; }
.precheck-warnings { margin-top: 4px; }
.warning-line { font-size: 14px; color: #dc2626; padding: 2px 0; }
.precheck-force { margin-top: 6px; font-size: 14px; color: #dc2626; }
.precheck-force label { display: flex; align-items: center; gap: 6px; cursor: pointer; }

/* Rollback */
.rollback-list { max-height: 300px; overflow-y: auto; }
.rollback-item { display: flex; align-items: center; padding: 6px 8px; border-radius: 4px; }
.rollback-item:hover { background: #f5f5f5; }
.rollback-item input { margin-right: 8px; }
.empty-hint { text-align: center; padding: 20px; color: #999999; font-size: 14px; }
.rollback-progress-section { padding: 12px 0; }
.rollback-warning { margin-top: 10px; padding: 10px 12px; background: #fef2f2; border: 1px solid #fecaca; border-radius: 6px; color: #dc2626; font-size: 14px; font-weight: 500; text-align: center; }

/* Completion */
.completion-list { max-height: 200px; overflow-y: auto; }
.completion-item { padding: 6px 10px; background: #f0fdf4; border-radius: 4px; margin-bottom: 4px; font-size: 14px; color: #059669; }

/* Log */
.log-content { background: #1f2937; color: #e0e0e0; padding: 14px; border-radius: 8px; font-family: 'Consolas', 'Monaco', monospace; font-size: 14px; line-height: 1.5; max-height: 450px; overflow-y: auto; white-space: pre-wrap; flex: 1; }

/* Toast */
.toast { position: fixed; bottom: 16px; right: 16px; padding: 10px 18px; border-radius: 8px; color: white; font-size: 14px; box-shadow: 0 4px 12px rgba(0,0,0,0.15); animation: slideIn 0.25s ease; z-index: 2000; }
.toast.success { background: #059669; }
.toast.error { background: #dc2626; }
.toast.info { background: #2563eb; }
@keyframes slideIn { from { transform: translateX(100%); opacity: 0; } to { transform: translateX(0); opacity: 1; } }
</style>
