export interface Todo {
  id: string
  title: string
  description?: string
  completed: boolean
  priority: 'low' | 'medium' | 'high'
  category?: string
  createdAt: Date
  updatedAt: Date
  dueDate?: Date
}

export type FilterType = 'all' | 'active' | 'completed'
export type SortType = 'newest' | 'oldest' | 'priority' | 'dueDate'