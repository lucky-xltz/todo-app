import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Todo, FilterType, SortType } from '../types/todo'
import { invoke } from '@tauri-apps/api/core'

export const useTodoStore = defineStore('todo', () => {
  const todos = ref<Todo[]>([])
  const filter = ref<FilterType>('all')
  const sort = ref<SortType>('newest')
  const searchQuery = ref('')
  const isLoading = ref(false)

  // 从后端加载待办
  const loadTodos = async () => {
    try {
      isLoading.value = true
      const result = await invoke<Todo[]>('get_todos')
      todos.value = result.map(t => ({
        ...t,
        createdAt: new Date(t.createdAt),
        updatedAt: new Date(t.updatedAt),
        dueDate: t.dueDate ? new Date(t.dueDate) : undefined
      }))
    } catch (error) {
      console.error('加载待办失败:', error)
    } finally {
      isLoading.value = false
    }
  }

  // 添加待办
  const addTodo = async (title: string, description?: string, priority: Todo['priority'] = 'medium', dueDate?: Date) => {
    try {
      const todo = await invoke<Todo>('create_todo', {
        todo: {
          title,
          description: description || null,
          priority,
          dueDate: dueDate ? dueDate.toISOString() : null
        }
      })
      todos.value.unshift({
        ...todo,
        createdAt: new Date(todo.createdAt),
        updatedAt: new Date(todo.updatedAt),
        dueDate: todo.dueDate ? new Date(todo.dueDate) : undefined
      })
    } catch (error) {
      console.error('添加待办失败:', error)
    }
  }

  // 删除待办
  const removeTodo = async (id: string) => {
    try {
      await invoke('delete_todo', { id })
      todos.value = todos.value.filter(t => t.id !== id)
    } catch (error) {
      console.error('删除待办失败:', error)
    }
  }

  // 切换完成状态
  const toggleTodo = async (id: string) => {
    try {
      const todo = await invoke<Todo>('toggle_todo', { id })
      const index = todos.value.findIndex(t => t.id === id)
      if (index !== -1) {
        todos.value[index] = {
          ...todo,
          createdAt: new Date(todo.createdAt),
          updatedAt: new Date(todo.updatedAt),
          dueDate: todo.dueDate ? new Date(todo.dueDate) : undefined
        }
      }
    } catch (error) {
      console.error('切换待办状态失败:', error)
    }
  }

  // 更新待办
  const updateTodo = async (id: string, updates: Partial<Todo>) => {
    try {
      const todo = await invoke<Todo>('update_todo', {
        id,
        updates: {
          title: updates.title || null,
          description: updates.description || null,
          completed: updates.completed ?? null,
          priority: updates.priority || null,
          dueDate: updates.dueDate ? updates.dueDate.toISOString() : null
        }
      })
      const index = todos.value.findIndex(t => t.id === id)
      if (index !== -1) {
        todos.value[index] = {
          ...todo,
          createdAt: new Date(todo.createdAt),
          updatedAt: new Date(todo.updatedAt),
          dueDate: todo.dueDate ? new Date(todo.dueDate) : undefined
        }
      }
    } catch (error) {
      console.error('更新待办失败:', error)
    }
  }

  // 清除已完成
  const clearCompleted = async () => {
    try {
      await invoke('clear_completed')
      todos.value = todos.value.filter(t => !t.completed)
    } catch (error) {
      console.error('清除已完成失败:', error)
    }
  }

  // 筛选后的待办
  const filteredTodos = computed(() => {
    let result = [...todos.value]

    // 搜索过滤
    if (searchQuery.value) {
      const query = searchQuery.value.toLowerCase()
      result = result.filter(t =>
        t.title.toLowerCase().includes(query) ||
        t.description?.toLowerCase().includes(query)
      )
    }

    // 状态过滤
    switch (filter.value) {
      case 'active':
        result = result.filter(t => !t.completed)
        break
      case 'completed':
        result = result.filter(t => t.completed)
        break
    }

    // 排序
    switch (sort.value) {
      case 'newest':
        result.sort((a, b) => b.createdAt.getTime() - a.createdAt.getTime())
        break
      case 'oldest':
        result.sort((a, b) => a.createdAt.getTime() - b.createdAt.getTime())
        break
      case 'priority':
        const priorityOrder = { high: 0, medium: 1, low: 2 }
        result.sort((a, b) => priorityOrder[a.priority] - priorityOrder[b.priority])
        break
      case 'dueDate':
        result.sort((a, b) => {
          if (!a.dueDate) return 1
          if (!b.dueDate) return -1
          return a.dueDate.getTime() - b.dueDate.getTime()
        })
        break
    }

    return result
  })

  // 统计
  const stats = computed(() => {
    const total = todos.value.length
    const completed = todos.value.filter(t => t.completed).length
    const active = total - completed
    const completionRate = total > 0 ? Math.round((completed / total) * 100) : 0
    return { total, completed, active, completionRate }
  })

  // 初始化
  loadTodos()

  return {
    todos,
    filter,
    sort,
    searchQuery,
    filteredTodos,
    stats,
    isLoading,
    addTodo,
    removeTodo,
    toggleTodo,
    updateTodo,
    clearCompleted,
    loadTodos
  }
})