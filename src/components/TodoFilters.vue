<template>
  <div class="todo-filters">
    <div class="search-box">
      <span class="search-icon">
        <Icon name="search" :size="18" color="var(--text-secondary)" />
      </span>
      <input
        v-model="store.searchQuery"
        type="text"
        placeholder="搜索待办事项..."
        class="search-input"
      />
    </div>
    
    <div class="filter-row">
      <div class="filter-buttons">
        <button
          v-for="f in filters"
          :key="f.value"
          :class="['filter-btn', { active: store.filter === f.value }]"
          @click="store.filter = f.value"
        >
          {{ f.label }}
          <span v-if="f.count !== undefined" class="filter-count">{{ f.count }}</span>
        </button>
      </div>
      
      <div class="sort-selector">
        <select v-model="store.sort" class="sort-select">
          <option value="newest">最新</option>
          <option value="oldest">最早</option>
          <option value="priority">优先级</option>
          <option value="dueDate">截止日期</option>
        </select>
      </div>
    </div>
    
    <div class="actions-bar" v-if="store.stats.completed > 0">
      <button @click="store.clearCompleted" class="clear-btn">
        清除已完成 ({{ store.stats.completed }})
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useTodoStore } from '../stores/todo'
import Icon from './Icon.vue'

const store = useTodoStore()

const filters = computed(() => [
  { value: 'all' as const, label: '全部', count: store.stats.total },
  { value: 'active' as const, label: '待办', count: store.stats.active },
  { value: 'completed' as const, label: '完成', count: store.stats.completed }
])
</script>

<style scoped>
.todo-filters {
  background: var(--bg-secondary);
  border-radius: 20px;
  padding: 20px 24px;
  margin-bottom: 24px;
  box-shadow: var(--shadow-outset);
}

.search-box {
  position: relative;
  margin-bottom: 16px;
}

.search-icon {
  position: absolute;
  left: 16px;
  top: 50%;
  transform: translateY(-50%);
  display: flex;
  align-items: center;
}

.search-input {
  width: 100%;
  padding: 14px 16px 14px 44px;
  background: var(--bg-primary);
  border: none;
  border-radius: 12px;
  font-size: 14px;
  color: var(--text-primary);
  box-shadow: var(--shadow-inset);
  transition: all 0.3s ease;
}

.search-input:focus {
  outline: none;
  box-shadow: var(--shadow-inset), 0 0 0 2px var(--accent-primary);
}

.search-input::placeholder {
  color: var(--text-secondary);
}

.filter-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 16px;
}

.filter-buttons {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.filter-btn {
  padding: 10px 18px;
  background: var(--bg-primary);
  border: none;
  border-radius: 10px;
  font-size: 14px;
  color: var(--text-secondary);
  cursor: pointer;
  box-shadow: var(--shadow-outset-sm);
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  gap: 6px;
  font-weight: 500;
}

.filter-btn:hover {
  color: var(--text-primary);
  transform: translateY(-1px);
}

.filter-btn.active {
  background: var(--accent-primary);
  color: white;
  box-shadow: var(--shadow-inset);
}

.filter-count {
  font-size: 11px;
  padding: 2px 6px;
  background: rgba(255, 255, 255, 0.2);
  border-radius: 4px;
  font-weight: 600;
}

.filter-btn:not(.active) .filter-count {
  background: var(--bg-secondary);
}

.sort-selector {
  flex-shrink: 0;
}

.sort-select {
  padding: 10px 36px 10px 16px;
  background: var(--bg-primary);
  border: none;
  border-radius: 10px;
  font-size: 14px;
  color: var(--text-primary);
  box-shadow: var(--shadow-inset);
  cursor: pointer;
  transition: all 0.2s ease;
  appearance: none;
  -webkit-appearance: none;
  background-image: url("data:image/svg+xml;charset=UTF-8,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='%23636e72' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3e%3cpolyline points='6 9 12 15 18 9'%3e%3c/polyline%3e%3c/svg%3e");
  background-repeat: no-repeat;
  background-position: right 12px center;
  background-size: 16px;
  font-weight: 500;
}

.sort-select:focus {
  outline: none;
  box-shadow: var(--shadow-inset), 0 0 0 2px var(--accent-primary);
}

.sort-select option {
  background: var(--bg-primary);
  color: var(--text-primary);
  padding: 8px;
}

.actions-bar {
  margin-top: 16px;
  display: flex;
  justify-content: flex-end;
}

.clear-btn {
  padding: 10px 20px;
  background: transparent;
  border: none;
  border-radius: 10px;
  font-size: 13px;
  color: var(--priority-high);
  cursor: pointer;
  transition: all 0.2s ease;
  font-weight: 500;
}

.clear-btn:hover {
  background: rgba(231, 76, 60, 0.1);
}

/* 响应式布局 */
@media (max-width: 768px) {
  .todo-filters {
    padding: 16px 20px;
    border-radius: 16px;
  }
  
  .filter-row {
    flex-direction: column;
    align-items: stretch;
    gap: 12px;
  }
  
  .filter-buttons {
    justify-content: center;
  }
  
  .filter-btn {
    flex: 1;
    justify-content: center;
    padding: 10px 12px;
    font-size: 13px;
  }
  
  .sort-select {
    width: 100%;
  }
}

@media (max-width: 480px) {
  .todo-filters {
    padding: 14px 16px;
    margin-bottom: 16px;
    border-radius: 14px;
  }
  
  .search-input {
    padding: 12px 12px 12px 40px;
    font-size: 13px;
  }
  
  .filter-btn {
    padding: 8px 10px;
    font-size: 12px;
  }
  
  .filter-count {
    font-size: 10px;
    padding: 1px 4px;
  }
}
</style>