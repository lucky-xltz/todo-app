<template>
  <div class="app-container">
    <div class="app-wrapper">
      <TodoHeader />
      <TodoInput />
      <TodoFilters />
      <TodoList />
      
      <footer class="app-footer">
        <p>数据保存在本地浏览器中</p>
      </footer>
    </div>
  </div>
</template>

<script setup lang="ts">
import TodoHeader from './components/TodoHeader.vue'
import TodoInput from './components/TodoInput.vue'
import TodoFilters from './components/TodoFilters.vue'
import TodoList from './components/TodoList.vue'
</script>

<style>
/* 全局变量 - 新拟态风格 */
:root {
  /* 背景色 */
  --bg-primary: #e0e5ec;
  --bg-secondary: #e0e5ec;
  
  /* 文字颜色 */
  --text-primary: #2d3436;
  --text-secondary: #636e72;
  
  /* 强调色 */
  --accent-primary: #6c5ce7;
  --accent-secondary: #a29bfe;
  
  /* 优先级颜色 */
  --priority-low: #00b894;
  --priority-medium: #fdcb6e;
  --priority-high: #e17055;
  
  /* 新拟态阴影 */
  --shadow-light: #ffffff;
  --shadow-dark: #a3b1c6;
  --shadow-outset: 6px 6px 12px var(--shadow-dark), -6px -6px 12px var(--shadow-light);
  --shadow-outset-sm: 3px 3px 6px var(--shadow-dark), -3px -3px 6px var(--shadow-light);
  --shadow-inset: inset 3px 3px 6px var(--shadow-dark), inset -3px -3px 6px var(--shadow-light);
}

/* 全局样式重置 */
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: 'PingFang SC', 'Microsoft YaHei', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
  background: var(--bg-primary);
  color: var(--text-primary);
  min-height: 100vh;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

/* 滚动条样式 */
::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-track {
  background: var(--bg-primary);
}

::-webkit-scrollbar-thumb {
  background: var(--shadow-dark);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: var(--text-secondary);
}

/* 选择样式 */
::selection {
  background: var(--accent-primary);
  color: white;
}

.app-container {
  min-height: 100vh;
  padding: 40px 20px;
  background: linear-gradient(135deg, #e0e5ec 0%, #d5dbe3 100%);
}

.app-wrapper {
  max-width: 720px;
  margin: 0 auto;
}

.app-footer {
  text-align: center;
  padding: 24px;
  margin-top: 24px;
}

.app-footer p {
  font-size: 12px;
  color: var(--text-secondary);
}

/* 响应式调整 */
@media (max-width: 768px) {
  .app-container {
    padding: 20px 16px;
  }
}
</style>