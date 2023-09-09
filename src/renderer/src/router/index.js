import { createRouter, createWebHistory } from 'vue-router'

const routerHistory = createWebHistory()

import login from '@renderer/views/login/index.vue'
import home from '@renderer/views/home/index.vue'
// 定义路由
const routes = [
  {
    path: '/login',
    name: 'Login',
    component: login
  },
  {
    path: '/',
    name: 'Home',
    component: home
  }
]

// 创建路由器
const router = createRouter({
  history: routerHistory,
  routes: routes
})

router.beforeEach(async (to, from) => {
  const ipcRenderer = window.electron.ipcRenderer
  const token = await ipcRenderer.invoke('get-token').then((res) => res)
  if (token === '666') {
    console.log('666')
    return true
  } else {
    if (to.path === '/login') {
      return true
    } else {
      console.log('nonono')
      return '/login'
    }
  }
  // 返回 false 以取消导航
  return false
})

export default router
