<template>
  <svg :width="size" :height="size" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
    <!-- 搜索 -->
    <template v-if="name === 'search'">
      <circle cx="11" cy="11" r="7" :stroke="color" stroke-width="2"/>
      <path d="M16 16L21 21" :stroke="color" stroke-width="2" stroke-linecap="round"/>
    </template>
    
    <!-- 日历 -->
    <template v-else-if="name === 'calendar'">
      <rect x="3" y="4" width="18" height="18" rx="2" :stroke="color" stroke-width="2"/>
      <path d="M3 10H21" :stroke="color" stroke-width="2"/>
      <path d="M8 2V6" :stroke="color" stroke-width="2" stroke-linecap="round"/>
      <path d="M16 2V6" :stroke="color" stroke-width="2" stroke-linecap="round"/>
      <circle cx="8" cy="15" r="1" :fill="color"/>
      <circle cx="12" cy="15" r="1" :fill="color"/>
      <circle cx="16" cy="15" r="1" :fill="color"/>
    </template>
    
    <!-- 笔记 -->
    <template v-else-if="name === 'note'">
      <path d="M14 2H6C5.46957 2 4.96086 2.21071 4.58579 2.58579C4.21071 2.96086 4 3.46957 4 4V20C4 20.5304 4.21071 21.0391 4.58579 21.4142C4.96086 21.7893 5.46957 22 6 22H18C18.5304 22 19.0391 21.7893 19.4142 21.4142C19.7893 21.0391 20 20.5304 20 20V8L14 2Z" :stroke="color" stroke-width="2"/>
      <path d="M14 2V8H20" :stroke="color" stroke-width="2"/>
      <path d="M8 13H16" :stroke="color" stroke-width="2" stroke-linecap="round"/>
      <path d="M8 17H12" :stroke="color" stroke-width="2" stroke-linecap="round"/>
    </template>
    
    <!-- 勾选 -->
    <template v-else-if="name === 'check'">
      <path d="M5 12L10 17L19 8" :stroke="color" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    </template>
    
    <!-- 编辑 -->
    <template v-else-if="name === 'edit'">
      <path d="M11 4H4C3.46957 4 2.96086 4.21071 2.58579 4.58579C2.21071 4.96086 2 5.46957 2 6V20C2 20.5304 2.21071 21.0391 2.58579 21.4142C2.96086 21.7893 3.46957 22 4 22H18C18.5304 22 19.0391 21.7893 19.4142 21.4142C19.7893 21.0391 20 20.5304 20 20V13" :stroke="color" stroke-width="2"/>
      <path d="M18.5 2.50023C18.8978 2.10243 19.4374 1.87891 20 1.87891C20.5626 1.87891 21.1022 2.10243 21.5 2.50023C21.8978 2.89804 22.1213 3.43762 22.1213 4.00023C22.1213 4.56284 21.8978 5.10243 21.5 5.50023L12 15.0002L8 16.0002L9 12.0002L18.5 2.50023Z" :stroke="color" stroke-width="2"/>
    </template>
    
    <!-- 删除 -->
    <template v-else-if="name === 'delete'">
      <path d="M3 6H5H21" :stroke="color" stroke-width="2" stroke-linecap="round"/>
      <path d="M8 6V4C8 3.46957 8.21071 2.96086 8.58579 2.58579C8.96086 2.21071 9.46957 2 10 2H14C14.5304 2 15.0391 2.21071 15.4142 2.58579C15.7893 2.96086 16 3.46957 16 4V6M19 6V20C19 20.5304 18.7893 21.0391 18.4142 21.4142C18.0391 21.7893 17.5304 22 17 22H7C6.46957 22 5.96086 21.7893 5.58579 21.4142C5.21071 21.0391 5 20.5304 5 20V6H19Z" :stroke="color" stroke-width="2"/>
    </template>
    
    <!-- 加号 -->
    <template v-else-if="name === 'plus'">
      <path d="M12 5V19" :stroke="color" stroke-width="2" stroke-linecap="round"/>
      <path d="M5 12H19" :stroke="color" stroke-width="2" stroke-linecap="round"/>
    </template>
    
    <!-- 下箭头 -->
    <template v-else-if="name === 'chevron-down'">
      <path d="M6 9L12 15L18 9" :stroke="color" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    </template>
    
    <!-- 左箭头 -->
    <template v-else-if="name === 'chevron-left'">
      <path d="M15 18L9 12L15 6" :stroke="color" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    </template>
    
    <!-- 右箭头 -->
    <template v-else-if="name === 'chevron-right'">
      <path d="M9 18L15 12L9 6" :stroke="color" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    </template>
  </svg>
</template>

<script setup lang="ts">
defineProps<{
  name: 'search' | 'calendar' | 'note' | 'check' | 'edit' | 'delete' | 'plus' | 'chevron-down' | 'chevron-left' | 'chevron-right'
  size?: number
  color?: string
}>()
</script>