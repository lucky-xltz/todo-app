<template>
  <div class="custom-select" ref="selectRef" :class="{ open: isOpen }">
    <div class="select-display" @click="toggleDropdown">
      <span class="select-text">{{ selectedLabel }}</span>
      <span class="arrow" :class="{ up: isOpen }">
        <Icon name="chevron-down" :size="12" color="var(--text-secondary)" />
      </span>
    </div>
    
    <Transition name="dropdown">
      <div v-if="isOpen" class="select-dropdown">
        <div
          v-for="option in options"
          :key="option.value"
          :class="['select-option', { active: modelValue === option.value }]"
          @click="selectOption(option)"
        >
          <Icon v-if="modelValue === option.value" name="check" :size="14" color="var(--accent-primary)" />
          <span>{{ option.label }}</span>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import Icon from './Icon.vue'

interface Option {
  value: string
  label: string
}

const props = defineProps<{
  modelValue: string
  options: Option[]
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const isOpen = ref(false)
const selectRef = ref<HTMLElement>()

const selectedLabel = computed(() => {
  const option = props.options.find(o => o.value === props.modelValue)
  return option ? option.label : ''
})

const toggleDropdown = () => {
  isOpen.value = !isOpen.value
}

const selectOption = (option: Option) => {
  emit('update:modelValue', option.value)
  isOpen.value = false
}

const handleClickOutside = (e: MouseEvent) => {
  const target = e.target as HTMLElement
  if (!target.closest('.custom-select')) {
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
.custom-select {
  position: relative;
  min-width: 120px;
}

.select-display {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 14px;
  background: var(--bg-primary);
  border: none;
  border-radius: 10px;
  box-shadow: var(--shadow-inset);
  cursor: pointer;
  transition: all 0.2s ease;
  user-select: none;
}

.select-display:hover {
  box-shadow: var(--shadow-inset), 0 0 0 2px var(--accent-secondary);
}

.custom-select.open .select-display {
  box-shadow: var(--shadow-inset), 0 0 0 2px var(--accent-primary);
}

.select-text {
  flex: 1;
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
  white-space: nowrap;
}

.arrow {
  display: flex;
  align-items: center;
  transition: transform 0.2s ease;
  flex-shrink: 0;
}

.arrow.up {
  transform: rotate(180deg);
}

.select-dropdown {
  position: absolute;
  top: calc(100% + 6px);
  left: 0;
  right: 0;
  background: var(--bg-secondary);
  border-radius: 12px;
  box-shadow: 0 6px 24px rgba(0, 0, 0, 0.1);
  padding: 6px;
  z-index: 100;
  min-width: 100%;
}

.select-option {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 12px;
  border-radius: 8px;
  font-size: 13px;
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.15s ease;
}

.select-option:hover {
  background: var(--bg-primary);
}

.select-option.active {
  color: var(--accent-primary);
  font-weight: 600;
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
    transform: translateY(-4px) scale(0.98);
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
    transform: translateY(-4px) scale(0.98);
  }
}
</style>