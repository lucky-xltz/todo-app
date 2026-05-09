<template>
  <div class="custom-date-picker" ref="pickerRef" :class="{ open: isOpen }">
    <div class="date-display" @click="togglePicker">
      <Icon name="calendar" :size="16" color="var(--text-secondary)" />
      <span class="date-text" :class="{ placeholder: !modelValue }">
        {{ displayText }}
      </span>
      <span class="arrow" :class="{ up: isOpen }">
        <Icon name="chevron-down" :size="12" color="var(--text-secondary)" />
      </span>
    </div>
    
    <Transition name="dropdown">
      <div v-if="isOpen" ref="dropdownRef" class="date-dropdown" :class="{ 'drop-up': dropUp }">
        <div class="calendar-header">
          <button class="nav-btn" @click="prevMonth">
            <Icon name="chevron-left" :size="14" color="var(--text-primary)" />
          </button>
          <span class="current-month">{{ currentMonthYear }}</span>
          <button class="nav-btn" @click="nextMonth">
            <Icon name="chevron-right" :size="14" color="var(--text-primary)" />
          </button>
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
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue'
import Icon from './Icon.vue'

const props = defineProps<{
  modelValue: string
  placeholder?: string
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const pickerRef = ref<HTMLElement>()
const dropdownRef = ref<HTMLElement>()
const isOpen = ref(false)
const dropUp = ref(false)
const currentDate = ref(new Date())

const weekdays = ['日', '一', '二', '三', '四', '五', '六']

const displayText = computed(() => {
  if (!props.modelValue) return props.placeholder || '选择日期'
  const date = new Date(props.modelValue)
  return `${date.getFullYear()}/${date.getMonth() + 1}/${date.getDate()}`
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

const checkPosition = () => {
  if (!pickerRef.value) return
  
  const rect = pickerRef.value.getBoundingClientRect()
  const viewportHeight = window.innerHeight
  const dropdownHeight = 300
  const spaceBelow = viewportHeight - rect.bottom
  const spaceAbove = rect.top
  
  dropUp.value = spaceBelow < dropdownHeight && spaceAbove > spaceBelow
}

const togglePicker = () => {
  isOpen.value = !isOpen.value
  if (isOpen.value) {
    nextTick(() => {
      checkPosition()
    })
  }
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

const handleClickOutside = (e: MouseEvent) => {
  const target = e.target as HTMLElement
  if (!target.closest('.custom-date-picker')) {
    isOpen.value = false
  }
}

const handleScroll = () => {
  if (isOpen.value) {
    checkPosition()
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
  window.addEventListener('scroll', handleScroll, true)
  window.addEventListener('resize', handleScroll)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
  window.removeEventListener('scroll', handleScroll, true)
  window.removeEventListener('resize', handleScroll)
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
  gap: 8px;
  padding: 10px 12px;
  background: var(--bg-primary);
  border: none;
  border-radius: 10px;
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

.date-text {
  flex: 1;
  font-size: 13px;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.date-text.placeholder {
  color: var(--text-secondary);
}

.arrow {
  display: flex;
  align-items: center;
  transition: transform 0.3s ease;
  flex-shrink: 0;
}

.arrow.up {
  transform: rotate(180deg);
}

.date-dropdown {
  position: absolute;
  left: 0;
  right: 0;
  background: var(--bg-secondary);
  border-radius: 14px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.12);
  padding: 14px;
  z-index: 100;
  top: calc(100% + 6px);
  min-width: 260px;
  max-width: 300px;
}

.date-dropdown.drop-up {
  top: auto;
  bottom: calc(100% + 6px);
}

.calendar-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
}

.nav-btn {
  width: 28px;
  height: 28px;
  background: var(--bg-primary);
  border: none;
  border-radius: 6px;
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
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.weekday-row {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 2px;
  margin-bottom: 6px;
}

.weekday {
  text-align: center;
  font-size: 11px;
  color: var(--text-secondary);
  font-weight: 600;
  padding: 4px 0;
}

.days-grid {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 2px;
}

.day-btn {
  width: 100%;
  aspect-ratio: 1;
  background: transparent;
  border: none;
  border-radius: 8px;
  font-size: 13px;
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  max-height: 36px;
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
  bottom: 3px;
  left: 50%;
  transform: translateX(-50%);
  width: 3px;
  height: 3px;
  background: var(--accent-primary);
  border-radius: 50%;
}

.day-btn.selected {
  background: var(--accent-primary);
  color: white;
  box-shadow: var(--shadow-outset-sm);
}

.day-btn.selected::after {
  display: none;
}

.calendar-footer {
  display: flex;
  justify-content: space-between;
  margin-top: 10px;
  padding-top: 10px;
  border-top: 1px solid var(--shadow-dark);
}

.today-btn,
.clear-btn {
  padding: 6px 14px;
  border: none;
  border-radius: 8px;
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s ease;
  font-weight: 500;
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
  animation: dropdownIn 0.2s ease;
}

.dropdown-leave-active {
  animation: dropdownOut 0.15s ease;
}

@keyframes dropdownIn {
  from {
    opacity: 0;
    transform: translateY(-6px) scale(0.98);
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
    transform: translateY(-6px) scale(0.98);
  }
}

.drop-up.dropdown-enter-active {
  animation: dropdownUpIn 0.2s ease;
}

.drop-up.dropdown-leave-active {
  animation: dropdownUpOut 0.15s ease;
}

@keyframes dropdownUpIn {
  from {
    opacity: 0;
    transform: translateY(6px) scale(0.98);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

@keyframes dropdownUpOut {
  from {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
  to {
    opacity: 0;
    transform: translateY(6px) scale(0.98);
  }
}

/* 响应式 */
@media (max-width: 480px) {
  .date-dropdown,
  .date-dropdown.drop-up {
    position: fixed;
    top: auto !important;
    bottom: 0;
    left: 0;
    right: 0;
    border-radius: 16px 16px 0 0;
    padding: 16px;
    max-width: none;
    min-width: auto;
  }
  
  .day-btn {
    font-size: 15px;
    max-height: 42px;
  }
}
</style>