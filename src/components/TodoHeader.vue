<template>
  <header class="todo-header">
    <div class="header-content">
      <h1 class="app-title">
        <span class="title-icon">
          <Icon name="check" :size="28" color="white" />
        </span>
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
import Icon from './Icon.vue'

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
  background: var(--accent-primary);
  width: 48px;
  height: 48px;
  border-radius: 14px;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: var(--shadow-outset);
}

.stats {
  display: flex;
  gap: 16px;
  align-items: center;
}

.stat-item {
  text-align: center;
  padding: 10px 16px;
  background: var(--bg-primary);
  border-radius: 12px;
  box-shadow: var(--shadow-inset);
  min-width: 70px;
}

.stat-number {
  display: block;
  font-size: 22px;
  font-weight: 700;
  color: var(--accent-primary);
  line-height: 1.2;
}

.stat-label {
  font-size: 11px;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 1px;
  margin-top: 2px;
}

.progress-ring {
  position: relative;
  width: 52px;
  height: 52px;
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
  font-size: 11px;
  font-weight: 700;
  color: var(--text-primary);
}

/* 响应式布局 */
@media (max-width: 768px) {
  .todo-header {
    padding: 20px;
    border-radius: 16px;
  }
  
  .header-content {
    flex-direction: column;
    gap: 16px;
  }
  
  .app-title {
    font-size: 24px;
  }
  
  .stats {
    width: 100%;
    justify-content: space-between;
    gap: 10px;
  }
  
  .stat-item {
    flex: 1;
    padding: 8px 10px;
    min-width: auto;
  }
  
  .stat-number {
    font-size: 18px;
  }
  
  .stat-label {
    font-size: 10px;
  }
  
  .progress-ring {
    width: 44px;
    height: 44px;
  }
  
  .progress-text {
    font-size: 10px;
  }
}

@media (max-width: 480px) {
  .todo-header {
    padding: 16px;
    margin-bottom: 16px;
    border-radius: 14px;
  }
  
  .app-title {
    font-size: 20px;
    gap: 10px;
  }
  
  .title-icon {
    width: 40px;
    height: 40px;
    border-radius: 12px;
  }
  
  .stat-item {
    padding: 6px 8px;
    border-radius: 10px;
  }
  
  .stat-number {
    font-size: 16px;
  }
  
  .stat-label {
    font-size: 9px;
  }
}
</style>