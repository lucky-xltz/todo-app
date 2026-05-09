<template>
  <header class="todo-header">
    <div class="header-content">
      <h1 class="app-title">
        <span class="title-icon">✓</span>
        待办清单
      </h1>
      <div class="stats">
        <div class="stat-item">
          <span class="stat-number">{{ stats.total }}</span>
          <span class="stat-label">总计</span>
        </div>
        <div class="stat-item">
          <span class="stat-number">{{ stats.active }}</span>
          <span class="stat-label">待办</span>
        </div>
        <div class="stat-item">
          <span class="stat-number">{{ stats.completed }}</span>
          <span class="stat-label">完成</span>
        </div>
        <div class="stat-item progress">
          <div class="progress-ring">
            <svg viewBox="0 0 36 36">
              <path
                class="progress-bg"
                d="M18 2.0845
                   a 15.9155 15.9155 0 0 1 0 31.831
                   a 15.9155 15.9155 0 0 1 0 -31.831"
              />
              <path
                class="progress-fill"
                :stroke-dasharray="`${stats.completionRate}, 100`"
                d="M18 2.0845
                   a 15.9155 15.9155 0 0 1 0 31.831
                   a 15.9155 15.9155 0 0 1 0 -31.831"
              />
            </svg>
            <span class="progress-text">{{ stats.completionRate }}%</span>
          </div>
        </div>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import { useTodoStore } from '../stores/todo'
import { storeToRefs } from 'pinia'

const store = useTodoStore()
const { stats } = storeToRefs(store)
</script>

<style scoped>
.todo-header {
  background: var(--bg-secondary);
  border-radius: 20px;
  padding: 24px 32px;
  margin-bottom: 24px;
  box-shadow: var(--shadow-outset);
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.app-title {
  font-size: 28px;
  font-weight: 700;
  color: var(--text-primary);
  display: flex;
  align-items: center;
  gap: 12px;
}

.title-icon {
  font-size: 32px;
  color: var(--accent-primary);
}

.stats {
  display: flex;
  gap: 24px;
  align-items: center;
}

.stat-item {
  text-align: center;
  padding: 8px 16px;
  background: var(--bg-primary);
  border-radius: 12px;
  box-shadow: var(--shadow-inset);
}

.stat-number {
  display: block;
  font-size: 24px;
  font-weight: 700;
  color: var(--accent-primary);
}

.stat-label {
  font-size: 12px;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 1px;
}

.progress-ring {
  position: relative;
  width: 56px;
  height: 56px;
}

.progress-ring svg {
  transform: rotate(-90deg);
}

.progress-bg {
  fill: none;
  stroke: var(--bg-primary);
  stroke-width: 3;
}

.progress-fill {
  fill: none;
  stroke: var(--accent-primary);
  stroke-width: 3;
  stroke-linecap: round;
  transition: stroke-dasharray 0.6s ease;
}

.progress-text {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  font-size: 12px;
  font-weight: 700;
  color: var(--text-primary);
}

@media (max-width: 768px) {
  .header-content {
    flex-direction: column;
    gap: 16px;
  }

  .stats {
    width: 100%;
    justify-content: space-between;
  }
}
</style>