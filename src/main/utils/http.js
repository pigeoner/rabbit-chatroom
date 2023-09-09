import axios from 'axios'

// 拦截器
axios.interceptors.response.use(
  (response) => {
    return response
  },
  (error) => {
    if (error.response === undefined) {
      error.response = {
        status: 500,
        data: '服务器错误'
      }
      return Promise.reject(error)
    }
    if (401 === error.response.status) {
      // 特殊HTTP码处理
      return Promise.reject(error)
    } else {
      return Promise.reject(error)
    }
  }
)

export default axios
