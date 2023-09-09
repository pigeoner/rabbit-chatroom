import { createApp } from 'vue'
import App from './App.vue'
import ElementPlus from 'element-plus'
import router from './router'
import 'element-plus/dist/index.css'
import '@renderer/assets/css/all.min.css'
import VueSocketIO from 'vue-socket.io'
import { registerSockets, destroySockets } from './utils/sockets'

const app = createApp(App)
app.use(router)
app.use(ElementPlus)

const socket = new VueSocketIO({
  debug: true,
  // 服务器端地址
  connection: 'http://127.0.0.1:3000',
  options: {
    path: '/test/'
  },
  vuex: {}
})

// 便于在任意位置获取到 socket 对象
app.config.globalProperties.$socket = socket
// 监听事件
app.config.globalProperties.$addSockets = registerSockets
// 移除事件
app.config.globalProperties.$removeSockets = destroySockets
app.mount('#app')
