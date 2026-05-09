<template>
  <div :class="['todo-item', { completed: todo.completed }]">
    <div class="todo-checkbox" @click.stop="store.toggleTodo(todo.id)">
      <input
        type="checkbox"
        :checked="todo.completed"
        class="checkbox-input"
        readonly
      />
      <span class="checkbox-custom">
        <Icon v-if="todo.completed" name="check" :size="14" color="white" />
      </span>
    </div>
    
    <div class="todo-content" @click="startEditing">
      <h3 class="todo-title">{{ todo.title }}</h3>
      <p v-if="todo.description" class="todo-description">{{ todo.description }}</p>
      <div class="todo-meta">
        <span :class="['priority-badge', todo.priority]">
          {{ getPriorityLabel(todo.priority) }}
        </span>
        <span v-if="todo.dueDate" class="due-date" :class="{ overdue: isOverdue }">
          <Icon name="calendar" :size="12" />
          {{ formatDate(todo.dueDate) }}
        </span>
        <span class="created-date">
          {{ formatCreatedDate(todo.createdAt) }}
        </span>
      </div>
    </div>
    
    <div class="todo-actions">
      <button
        @click.stop="startEditing"
        class="action-btn edit"
        title="编辑"
      >
        <Icon name="edit" :size="16" color="var(--accent-primary)" />
      </button>
      <button
        @click.stop="store.removeTodo(todo.id)"
        class="action-btn delete"
        title="删除"
      >
        <Icon name="delete" :size="16" color="var(--priority-high)" />
      </button>
    </div>

    <!-- 编辑模态框 -->
    <Teleport to="body">
      <div v-if="isEditing" class="edit-modal-overlay" @click.self="cancelEdit">
        <div class="edit-modal">
          <h3>编辑待办</h3>
          <form @submit.prevent="saveEdit">
            <div class="form-group">
              <label>标题</label>
              <input v-model="editTitle" type="text" required class="form-input" />
            </div>
            <div class="form-group">
              <label>描述</label>
              <textarea v-model="editDescription" class="form-input" rows="3" />
            </div>
            <div class="form-row">
              <div class="form-group">
                <label>优先级</label>
                <div class="priority-buttons">
                  <button
                    v-for="p in priorities"
                    :key="p.value"
                    type="button"
                    :class="['priority-btn', p.value, { active: editPriority === p.value }]"
                    @click="editPriority = p.value"
                  >
                    {{ p.label }}
                  </button>
                </div>
              </div>
              <div class="form-group">
                <label>截止日期</label>
                <CustomDatePicker v-model="editDueDate" placeholder="选择截止日期" />
              </div>
            </div>
            <div class="modal-actions">
              <button type="button" @click="cancelEdit" class="btn-cancel">取消</button>
              <button type="submit" class="btn-save">保存</button>
            </div>
          </form>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useTodoStore } from '../stores/todo'
import type { Todo } from '../types/todo'
import Icon from './Icon.vue'
import CustomDatePicker from './CustomDatePicker.vue'

const props = defineProps<{
  todo: Todo
}>()

const store = useTodoStore()

const isEditing = ref(false)
const editTitle = ref('')
const editDescription = ref('')
const editPriority = ref<Todo['priority']>('medium')
const editDueDate = ref('')

const priorities = [
  { value: 'low' as const, label: '低' },
  { value: 'medium' as const, label: '中' },
  { value: 'high' as const, label: '高' }
]

const isOverdue = computed(() => {
  if (!props.todo.dueDate) return false
  return new Date(props.todo.dueDate) < new Date() && !props.todo.completed
})

const getPriorityLabel = (priority: Todo['priority']) => {
  const labels = { low: '低', medium: '中', high: '高' }
  return labels[priority]
}

const formatDate = (date: Date | string) => {
  if (!date) return ''
  const d = date instanceof Date ? date : new Date(date)
  if (isNaN(d.getTime())) return ''
  const month = d.getMonth() + 1
  const day = d.getDate()
  return `${month}月${day}日`
}

const formatCreatedDate = (date: Date) => {
  const d = new Date(date)
  const now = new Date()
  const diff = now.getTime() - d.getTime()
  const days = Math.floor(diff / (1000 * 60 * 60 * 24))
  
  if (days === 0) return '今天'
  if (days === 1) return '昨天'
  if (days < 7) return `${days}天前`
  return formatDate(d)
}

const startEditing = () => {
  editTitle.value = props.todo.title
  editDescription.value = props.todo.description || ''
  editPriority.value = props.todo.priority
  editDueDate.value = props.todo.dueDate 
    ? new Date(props.todo.dueDate).toISOString().split('T')[0]
    : ''
  isEditing.value = true
}

const cancelEdit = () => {
  isEditing.value = false
}

const saveEdit = () => {
  store.updateTodo(props.todo.id, {
    title: editTitle.value,
    description: editDescription.value || undefined,
    priority: editPriority.value,
    dueDate: editDueDate.value ? new Date(editDueDate.value) : undefined
  })
  isEditing.value = false
}
</script>

<style scoped>
.todo-item {
  background: var(--bg-secondary);
  border-radius: 16px;
  padding: 18px 20px;
  display: flex;
  align-items: flex-start;
  gap: 14px;
  box-shadow: var(--shadow-outset);
  transition: all 0.3s ease;
  animation: slideIn 0.3s ease;
}

@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.todo-item:hover {
  transform: translateY(-2px);
  box-shadow: 8px 8px 16px var(--shadow-dark), -8px -8px 16px var(--shadow-light);
}

.todo-item.completed {
  opacity: 0.7;
}

.todo-item.completed .todo-title {
  text-decoration: line-through;
  color: var(--text-secondary);
}

.todo-checkbox {
  position: relative;
  flex-shrink: 0;
  cursor: pointer;
}

.checkbox-input {
  position: absolute;
  opacity: 0;
  width: 0;
  height: 0;
}

.checkbox-custom {
  display: block;
  width: 26px;
  height: 26px;
  background: var(--bg-primary);
  border-radius: 8px;
  box-shadow: var(--shadow-inset);
  transition: all 0.3s ease;
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
}

.checkbox-input:checked + .checkbox-custom {
  background: var(--accent-primary);
  box-shadow: var(--shadow-outset-sm);
}

.todo-content {
  flex: 1;
  min-width: 0;
  cursor: pointer;
}

.todo-title {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 6px 0;
  transition: all 0.3s ease;
  line-height: 1.4;
}

.todo-description {
  font-size: 13px;
  color: var(--text-secondary);
  margin: 0 0 10px 0;
  line-height: 1.4;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.todo-meta {
  display: flex;
  gap: 10px;
  align-items: center;
  flex-wrap: wrap;
}

.priority-badge {
  padding: 3px 8px;
  border-radius: 6px;
  font-size: 11px;
  font-weight: 600;
  letter-spacing: 0.5px;
}

.priority-badge.low {
  background: var(--priority-low);
  color: white;
}

.priority-badge.medium {
  background: var(--priority-medium);
  color: white;
}

.priority-badge.high {
  background: var(--priority-high);
  color: white;
}

.due-date {
  font-size: 12px;
  color: var(--text-secondary);
  display: flex;
  align-items: center;
  gap: 4px;
}

.due-date.overdue {
  color: var(--priority-high);
  font-weight: 600;
}

.created-date {
  font-size: 12px;
  color: var(--text-secondary);
}

.todo-actions {
  display: flex;
  gap: 6px;
  opacity: 1;
  transition: opacity 0.2s ease;
  flex-shrink: 0;
}

.action-btn {
  width: 32px;
  height: 32px;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
  background: var(--bg-primary);
  box-shadow: var(--shadow-outset-sm);
}

.action-btn:hover {
  transform: translateY(-1px);
}

.action-btn:active {
  transform: translateY(0);
  box-shadow: var(--shadow-inset);
}

.action-btn.edit {
  color: var(--accent-primary);
}

.action-btn.delete {
  color: var(--priority-high);
}

/* 编辑模态框 */
.edit-modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  animation: fadeIn 0.2s ease;
  padding: 20px;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

.edit-modal {
  background: var(--bg-secondary);
  border-radius: 24px;
  padding: 28px;
  width: 100%;
  max-width: 480px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  animation: scaleIn 0.3s ease;
  max-height: 90vh;
  overflow-y: auto;
}

@keyframes scaleIn {
  from {
    opacity: 0;
    transform: scale(0.9);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

.edit-modal h3 {
  margin: 0 0 24px 0;
  font-size: 20px;
  color: var(--text-primary);
}

.form-group {
  margin-bottom: 18px;
}

.form-group label {
  display: block;
  font-size: 12px;
  color: var(--text-secondary);
  margin-bottom: 10px;
  text-transform: uppercase;
  letter-spacing: 1px;
  font-weight: 600;
}

.form-input {
  width: 100%;
  padding: 12px 16px;
  background: var(--bg-primary);
  border: none;
  border-radius: 12px;
  font-size: 14px;
  color: var(--text-primary);
  box-shadow: var(--shadow-inset);
  font-family: inherit;
}

.form-input:focus {
  outline: none;
  box-shadow: var(--shadow-inset), 0 0 0 2px var(--accent-primary);
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.priority-buttons {
  display: flex;
  gap: 8px;
}

.priority-btn {
  flex: 1;
  padding: 10px 12px;
  border: none;
  border-radius: 10px;
  font-size: 13px;
  cursor: pointer;
  background: var(--bg-primary);
  color: var(--text-secondary);
  box-shadow: var(--shadow-outset-sm);
  transition: all 0.2s ease;
  font-weight: 500;
}

.priority-btn:hover {
  color: var(--text-primary);
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

.modal-actions {
  display: flex;
  gap: 12px;
  justify-content: flex-end;
  margin-top: 24px;
}

.btn-cancel, .btn-save {
  padding: 12px 24px;
  border: none;
  border-radius: 12px;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-cancel {
  background: var(--bg-primary);
  color: var(--text-secondary);
  box-shadow: var(--shadow-outset-sm);
}

.btn-cancel:hover {
  color: var(--text-primary);
}

.btn-save {
  background: var(--accent-primary);
  color: white;
  box-shadow: var(--shadow-outset-sm);
}

.btn-save:hover {
  transform: translateY(-1px);
  box-shadow: 4px 4px 8px var(--shadow-dark), -4px -4px 8px var(--shadow-light);
}

/* 响应式布局 */
@media (max-width: 768px) {
  .todo-item {
    padding: 16px;
    border-radius: 14px;
    gap: 12px;
  }
  
  .todo-actions {
    opacity: 1;
  }
  
  .edit-modal {
    padding: 24px;
    border-radius: 20px;
  }
  
  .form-row {
    grid-template-columns: 1fr;
    gap: 18px;
  }
}

@media (max-width: 480px) {
  .todo-item {
    padding: 14px;
    gap: 10px;
  }
  
  .checkbox-custom {
    width: 22px;
    height: 22px;
  }
  
  .todo-title {
    font-size: 14px;
  }
  
  .todo-description {
    font-size: 12px;
  }
  
  .todo-meta {
    gap: 8px;
  }
  
  .priority-badge {
    font-size: 10px;
    padding: 2px 6px;
  }
  
  .due-date, .created-date {
    font-size: 11px;
  }
  
  .action-btn {
    width: 28px;
    height: 28px;
  }
  
  .edit-modal {
    padding: 20px;
    border-radius: 18px;
  }
  
  .modal-actions {
    flex-direction: column;
  }
  
  .btn-cancel, .btn-save {
    width: 100%;
    text-align: center;
  }
}
</style>