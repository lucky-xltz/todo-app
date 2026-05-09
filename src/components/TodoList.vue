<template>
  <div class="todo-list">
    <TransitionGroup name="list" tag="div" class="list-container">
      <TodoItem
        v-for="todo in store.filteredTodos"
        :key="todo.id"
        :todo="todo"
      />
    </TransitionGroup>
    
    <div v-if="store.filteredTodos.length === 0" class="empty-state">
      <div class="empty-icon">
        <Icon name="note" :size="48" color="var(--accent-primary)" />
      </div>
      <h3 class="empty-title">{{ emptyTitle }}</h3>
      <p class="empty-description">{{ emptyDescription }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useTodoStore } from '../stores/todo'
import TodoItem from './TodoItem.vue'
import Icon from './Icon.vue'

const store = useTodoStore()

const emptyTitle = computed(() => {
  if (store.searchQuery) {
    return '没有找到匹配的待办'
  }
  switch (store.filter) {
    case 'active':
      return '没有待办事项'
    case 'completed':
      return '没有已完成的事项'
    default:
      return '还没有待办事项'
  }
})

const emptyDescription = computed(() => {
  if (store.searchQuery) {
    return '试试其他关键词'
  }
  switch (store.filter) {
    case 'active':
      return '所有任务都完成了！'
    case 'completed':
      return '完成一些任务后会显示在这里'
    default:
      return '添加你的第一个待办事项吧'
  }
})
</script>

<style scoped>
.todo-list {
  min-height: 200px;
}

.list-container {
  display: flex;
  flex-direction: column;
  gap: 14px;
}

/* 列表动画 */
.list-enter-active {
  transition: all 0.4s ease;
}

.list-leave-active {
  transition: all 0.3s ease;
}

.list-enter-from {
  opacity: 0;
  transform: translateX(-30px);
}

.list-leave-to {
  opacity: 0;
  transform: translateX(30px);
}

.list-move {
  transition: transform 0.4s ease;
}

.empty-state {
  text-align: center;
  padding: 48px 24px;
  background: var(--bg-secondary);
  border-radius: 20px;
  box-shadow: var(--shadow-outset);
}

.empty-icon {
  margin-bottom: 16px;
  animation: float 3s ease-in-out infinite;
}

@keyframes float {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-10px);
  }
}

.empty-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 8px 0;
}

.empty-description {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0;
}

/* 响应式布局 */
@media (max-width: 768px) {
  .list-container {
    gap: 12px;
  }
  
  .empty-state {
    padding: 40px 20px;
    border-radius: 16px;
  }
  
  .empty-title {
    font-size: 16px;
  }
  
  .empty-description {
    font-size: 13px;
  }
}

@media (max-width: 480px) {
  .list-container {
    gap: 10px;
  }
  
  .empty-state {
    padding: 32px 16px;
    border-radius: 14px;
  }
  
  .empty-title {
    font-size: 15px;
  }
}
</style>