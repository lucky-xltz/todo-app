<template>
  <div class="todo-input-container">
    <form @submit.prevent="handleSubmit" class="input-form">
      <div class="input-main">
        <input
          v-model="title"
          type="text"
          placeholder="添加新的待办事项..."
          class="input-field"
          required
        />
        <button type="submit" class="submit-btn" :disabled="!title.trim()">
          <span class="btn-icon">+</span>
        </button>
      </div>
      
      <div class="input-options" v-show="showOptions">
        <textarea
          v-model="description"
          placeholder="添加描述（可选）"
          class="description-field"
          rows="2"
        />
        
        <div class="options-row">
          <div class="priority-selector">
            <label>优先级：</label>
            <div class="priority-buttons">
              <button
                v-for="p in priorities"
                :key="p.value"
                type="button"
                :class="['priority-btn', p.value, { active: priority === p.value }]"
                @click="priority = p.value"
              >
                {{ p.label }}
              </button>
            </div>
          </div>
          
          <div class="due-date">
            <label>截止日期：</label>
            <input
              v-model="dueDate"
              type="date"
              class="date-input"
            />
          </div>
        </div>
      </div>
      
      <button
        type="button"
        class="toggle-options"
        @click="showOptions = !showOptions"
      >
        {{ showOptions ? '收起选项' : '更多选项' }}
        <span :class="['arrow', { up: showOptions }]">▼</span>
      </button>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useTodoStore } from '../stores/todo'

const store = useTodoStore()

const title = ref('')
const description = ref('')
const priority = ref<'low' | 'medium' | 'high'>('medium')
const dueDate = ref('')
const showOptions = ref(false)

const priorities = [
  { value: 'low' as const, label: '低' },
  { value: 'medium' as const, label: '中' },
  { value: 'high' as const, label: '高' }
]

const handleSubmit = () => {
  if (!title.value.trim()) return
  
  store.addTodo(
    title.value.trim(),
    description.value.trim() || undefined,
    priority.value,
    dueDate.value ? new Date(dueDate.value) : undefined
  )
  
  // 重置表单
  title.value = ''
  description.value = ''
  priority.value = 'medium'
  dueDate.value = ''
  showOptions.value = false
}
</script>

<style scoped>
.todo-input-container {
  background: var(--bg-secondary);
  border-radius: 20px;
  padding: 24px;
  margin-bottom: 24px;
  box-shadow: var(--shadow-outset);
}

.input-form {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.input-main {
  display: flex;
  gap: 12px;
}

.input-field {
  flex: 1;
  padding: 16px 20px;
  background: var(--bg-primary);
  border: none;
  border-radius: 12px;
  font-size: 16px;
  color: var(--text-primary);
  box-shadow: var(--shadow-inset);
  transition: all 0.3s ease;
}

.input-field:focus {
  outline: none;
  box-shadow: var(--shadow-inset), 0 0 0 2px var(--accent-primary);
}

.input-field::placeholder {
  color: var(--text-secondary);
}

.submit-btn {
  width: 56px;
  height: 56px;
  background: var(--accent-primary);
  border: none;
  border-radius: 12px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: var(--shadow-outset);
  transition: all 0.2s ease;
}

.submit-btn:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 6px 6px 12px var(--shadow-dark), -6px -6px 12px var(--shadow-light);
}

.submit-btn:active:not(:disabled) {
  transform: translateY(0);
  box-shadow: var(--shadow-inset);
}

.submit-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-icon {
  font-size: 28px;
  color: white;
  font-weight: 300;
}

.input-options {
  display: flex;
  flex-direction: column;
  gap: 16px;
  animation: slideDown 0.3s ease;
}

@keyframes slideDown {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.description-field {
  width: 100%;
  padding: 12px 16px;
  background: var(--bg-primary);
  border: none;
  border-radius: 12px;
  font-size: 14px;
  color: var(--text-primary);
  box-shadow: var(--shadow-inset);
  resize: vertical;
  font-family: inherit;
}

.description-field:focus {
  outline: none;
  box-shadow: var(--shadow-inset), 0 0 0 2px var(--accent-primary);
}

.description-field::placeholder {
  color: var(--text-secondary);
}

.options-row {
  display: flex;
  gap: 24px;
  align-items: flex-end;
}

.priority-selector, .due-date {
  flex: 1;
}

.priority-selector label, .due-date label {
  display: block;
  font-size: 12px;
  color: var(--text-secondary);
  margin-bottom: 8px;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.priority-buttons {
  display: flex;
  gap: 8px;
}

.priority-btn {
  padding: 8px 16px;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  cursor: pointer;
  background: var(--bg-primary);
  color: var(--text-secondary);
  box-shadow: var(--shadow-outset-sm);
  transition: all 0.2s ease;
}

.priority-btn:hover {
  transform: translateY(-1px);
}

.priority-btn.active {
  color: white;
  box-shadow: var(--shadow-inset);
}

.priority-btn.low.active {
  background: var(--priority-low);
}

.priority-btn.medium.active {
  background: var(--priority-medium);
}

.priority-btn.high.active {
  background: var(--priority-high);
}

.date-input {
  width: 100%;
  padding: 10px 16px;
  background: var(--bg-primary);
  border: none;
  border-radius: 8px;
  font-size: 14px;
  color: var(--text-primary);
  box-shadow: var(--shadow-outset-sm);
  cursor: pointer;
}

.date-input:focus {
  outline: none;
  box-shadow: var(--shadow-outset-sm), 0 0 0 2px var(--accent-primary);
}

.toggle-options {
  background: none;
  border: none;
  color: var(--text-secondary);
  font-size: 13px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 0;
  transition: color 0.2s ease;
}

.toggle-options:hover {
  color: var(--accent-primary);
}

.arrow {
  font-size: 10px;
  transition: transform 0.3s ease;
}

.arrow.up {
  transform: rotate(180deg);
}

@media (max-width: 768px) {
  .options-row {
    flex-direction: column;
    gap: 16px;
  }
}
</style>