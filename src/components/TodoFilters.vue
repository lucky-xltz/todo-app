<template>
  <div class="todo-filters">
    <div class="search-box">
      <span class="search-icon">
        <Icon name="search" :size="16" color="var(--text-secondary)" />
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
      
      <CustomSelect
        v-model="store.sort"
        :options="sortOptions"
      />
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
import CustomSelect from './CustomSelect.vue'

const store = useTodoStore()

const filters = computed(() => [
  { value: 'all' as const, label: '全部', count: store.stats.total },
  { value: 'active' as const, label: '待办', count: store.stats.active },
  { value: 'completed' as const, label: '完成', count: store.stats.completed }
])

const sortOptions = [
  { value: 'newest', label: '最新' },
  { value: 'oldest', label: '最早' },
  { value: 'priority', label: '优先级' },
  { value: 'dueDate', label: '截止日期' }
]
</script>

<style scoped>
.todo-filters {
  background: var(--bg-secondary);
  border-radius: 16px;
  padding: 18px 20px;
  margin-bottom: 18px;
  box-shadow: var(--shadow-outset);
}

.search-box {
  position: relative;
  margin-bottom: 14px;
}

.search-icon {
  position: absolute;
  left: 14px;
  top: 50%;
  transform: translateY(-50%);
  display: flex;
  align-items: center;
}

.search-input {
  width: 100%;
  padding: 12px 14px 12px 40px;
  background: var(--bg-primary);
  border: none;
  border-radius: 10px;
  font-size: 13px;
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
  gap: 12px;
}

.filter-buttons {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}

.filter-btn {
  padding: 8px 14px;
  background: var(--bg-primary);
  border: none;
  border-radius: 8px;
  font-size: 13px;
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
  font-size: 10px;
  padding: 2px 5px;
  background: rgba(255, 255, 255, 0.2);
  border-radius: 4px;
  font-weight: 600;
}

.filter-btn:not(.active) .filter-count {
  background: var(--bg-secondary);
}

.actions-bar {
  margin-top: 14px;
  display: flex;
  justify-content: flex-end;
}

.clear-btn {
  padding: 8px 16px;
  background: transparent;
  border: none;
  border-radius: 8px;
  font-size: 12px;
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
    padding: 14px 16px;
    border-radius: 14px;
  }
  
  .filter-row {
    flex-direction: column;
    align-items: stretch;
    gap: 10px;
  }
  
  .filter-buttons {
    justify-content: center;
  }
  
  .filter-btn {
    flex: 1;
    justify-content: center;
    padding: 8px 10px;
    font-size: 12px;
  }
}

@media (max-width: 480px) {
  .todo-filters {
    padding: 12px 14px;
    margin-bottom: 14px;
    border-radius: 12px;
  }
  
  .search-input {
    padding: 10px 10px 10px 36px;
    font-size: 12px;
  }
  
  .filter-btn {
    padding: 7px 8px;
    font-size: 11px;
  }
  
  .filter-count {
    font-size: 9px;
    padding: 1px 4px;
  }
}
</style>