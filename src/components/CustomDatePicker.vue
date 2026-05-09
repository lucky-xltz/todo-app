<template>
  <div class="custom-date-picker" :class="{ open: isOpen }">
    <div class="date-display" @click="togglePicker">
      <span class="date-icon">📅</span>
      <span class="date-text" :class="{ placeholder: !modelValue }">
        {{ displayText }}
      </span>
      <span class="arrow" :class="{ up: isOpen }">▼</span>
    </div>
    
    <Transition name="dropdown">
      <div v-if="isOpen" class="date-dropdown">
        <div class="calendar-header">
          <button class="nav-btn" @click="prevMonth">‹</button>
          <span class="current-month">{{ currentMonthYear }}</span>
          <button class="nav-btn" @click="nextMonth">›</button>
        </div>
        
        <div class="weekday-row">
          <span v-for="day in weekdays" :key="day" class="weekday">{{ day }}</span>
        </div>
        
        <div class="days-grid">
          <button
            v-for="(day, index) in calendarDays"
            :key="index"
            :class="['day-btn', {
              'other-month': !day.currentMonth,
              'today': day.isToday,
              'selected': day.isSelected,
              'has-date': day.date
            }]"
            @click="selectDate(day)"
            :disabled="!day.currentMonth"
          >
            {{ day.day }}
          </button>
        </div>
        
        <div class="calendar-footer">
          <button class="today-btn" @click="selectToday">今天</button>
          <button class="clear-btn" @click="clearDate">清除</button>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'

const props = defineProps<{
  modelValue: string
  placeholder?: string
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const isOpen = ref(false)
const currentDate = ref(new Date())

const weekdays = ['日', '一', '二', '三', '四', '五', '六']

const displayText = computed(() => {
  if (!props.modelValue) return props.placeholder || '选择日期'
  const date = new Date(props.modelValue)
  return `${date.getFullYear()}年${date.getMonth() + 1}月${date.getDate()}日`
})

const currentMonthYear = computed(() => {
  const year = currentDate.value.getFullYear()
  const month = currentDate.value.getMonth() + 1
  return `${year}年${month}月`
})

const calendarDays = computed(() => {
  const year = currentDate.value.getFullYear()
  const month = currentDate.value.getMonth()
  
  const firstDay = new Date(year, month, 1)
  const lastDay = new Date(year, month + 1, 0)
  
  const startDate = new Date(firstDay)
  startDate.setDate(startDate.getDate() - firstDay.getDay())
  
  const days = []
  const today = new Date()
  const selectedDate = props.modelValue ? new Date(props.modelValue) : null
  
  for (let i = 0; i < 42; i++) {
    const date = new Date(startDate)
    date.setDate(date.getDate() + i)
    
    const isCurrentMonth = date.getMonth() === month
    const isToday = date.toDateString() === today.toDateString()
    const isSelected = selectedDate && date.toDateString() === selectedDate.toDateString()
    
    days.push({
      day: date.getDate(),
      date: date.toISOString().split('T')[0],
      currentMonth: isCurrentMonth,
      isToday,
      isSelected
    })
  }
  
  return days
})

const togglePicker = () => {
  isOpen.value = !isOpen.value
}

const prevMonth = () => {
  currentDate.value = new Date(currentDate.value.getFullYear(), currentDate.value.getMonth() - 1, 1)
}

const nextMonth = () => {
  currentDate.value = new Date(currentDate.value.getFullYear(), currentDate.value.getMonth() + 1, 1)
}

const selectDate = (day: any) => {
  if (!day.currentMonth) return
  emit('update:modelValue', day.date)
  isOpen.value = false
}

const selectToday = () => {
  const today = new Date()
  emit('update:modelValue', today.toISOString().split('T')[0])
  isOpen.value = false
}

const clearDate = () => {
  emit('update:modelValue', '')
  isOpen.value = false
}

// 点击外部关闭
const handleClickOutside = (e: MouseEvent) => {
  const target = e.target as HTMLElement
  if (!target.closest('.custom-date-picker')) {
    isOpen.value = false
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

<style scoped>
.custom-date-picker {
  position: relative;
  width: 100%;
}

.date-display {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px 16px;
  background: var(--bg-primary);
  border: none;
  border-radius: 12px;
  box-shadow: var(--shadow-inset);
  cursor: pointer;
  transition: all 0.3s ease;
}

.date-display:hover {
  box-shadow: var(--shadow-inset), 0 0 0 2px var(--accent-secondary);
}

.custom-date-picker.open .date-display {
  box-shadow: var(--shadow-inset), 0 0 0 2px var(--accent-primary);
}

.date-icon {
  font-size: 18px;
  flex-shrink: 0;
}

.date-text {
  flex: 1;
  font-size: 14px;
  color: var(--text-primary);
}

.date-text.placeholder {
  color: var(--text-secondary);
}

.arrow {
  font-size: 10px;
  color: var(--text-secondary);
  transition: transform 0.3s ease;
}

.arrow.up {
  transform: rotate(180deg);
}

.date-dropdown {
  position: absolute;
  top: calc(100% + 8px);
  left: 0;
  right: 0;
  background: var(--bg-secondary);
  border-radius: 16px;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.15);
  padding: 16px;
  z-index: 100;
}

.calendar-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 16px;
}

.nav-btn {
  width: 32px;
  height: 32px;
  background: var(--bg-primary);
  border: none;
  border-radius: 8px;
  font-size: 18px;
  color: var(--text-primary);
  cursor: pointer;
  box-shadow: var(--shadow-outset-sm);
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
}

.nav-btn:hover {
  transform: translateY(-1px);
}

.nav-btn:active {
  transform: translateY(0);
  box-shadow: var(--shadow-inset);
}

.current-month {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.weekday-row {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 4px;
  margin-bottom: 8px;
}

.weekday {
  text-align: center;
  font-size: 12px;
  color: var(--text-secondary);
  font-weight: 600;
  padding: 8px 0;
}

.days-grid {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 4px;
}

.day-btn {
  width: 100%;
  aspect-ratio: 1;
  background: transparent;
  border: none;
  border-radius: 10px;
  font-size: 14px;
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
}

.day-btn:hover:not(:disabled):not(.selected) {
  background: var(--bg-primary);
  box-shadow: var(--shadow-outset-sm);
}

.day-btn:active:not(:disabled) {
  box-shadow: var(--shadow-inset);
}

.day-btn.other-month {
  color: var(--text-secondary);
  opacity: 0.5;
}

.day-btn.today {
  font-weight: 700;
  color: var(--accent-primary);
}

.day-btn.today::after {
  content: '';
  position: absolute;
  bottom: 4px;
  left: 50%;
  transform: translateX(-50%);
  width: 4px;
  height: 4px;
  background: var(--accent-primary);
  border-radius: 50%;
}

.day-btn.selected {
  background: var(--accent-primary);
  color: white;
  box-shadow: var(--shadow-outset-sm);
}

.calendar-footer {
  display: flex;
  justify-content: space-between;
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px solid var(--shadow-dark);
}

.today-btn,
.clear-btn {
  padding: 8px 16px;
  border: none;
  border-radius: 8px;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.today-btn {
  background: var(--accent-primary);
  color: white;
  box-shadow: var(--shadow-outset-sm);
}

.today-btn:hover {
  transform: translateY(-1px);
}

.clear-btn {
  background: var(--bg-primary);
  color: var(--text-secondary);
  box-shadow: var(--shadow-outset-sm);
}

.clear-btn:hover {
  color: var(--text-primary);
}

/* 下拉动画 */
.dropdown-enter-active {
  animation: dropdownIn 0.3s ease;
}

.dropdown-leave-active {
  animation: dropdownOut 0.2s ease;
}

@keyframes dropdownIn {
  from {
    opacity: 0;
    transform: translateY(-10px) scale(0.95);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

@keyframes dropdownOut {
  from {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
  to {
    opacity: 0;
    transform: translateY(-10px) scale(0.95);
  }
}

/* 响应式 */
@media (max-width: 480px) {
  .date-dropdown {
    position: fixed;
    top: auto;
    bottom: 0;
    left: 0;
    right: 0;
    border-radius: 20px 20px 0 0;
    padding: 20px;
    max-height: 80vh;
    overflow-y: auto;
  }
  
  .day-btn {
    font-size: 16px;
  }
}
</style>