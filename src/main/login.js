import axios from './utils/http'
import store from './utils/store'

export default async (userInfo) => {
  const result = await axios
    .post('http://127.0.0.1:3000/user/login', userInfo)
    .then(function (response) {
      const status = response.status
      if (status === 200) {
        store.set('token', '666')
        return { status, msg: '登录成功' }
      }
      return { status, msg: '登录失败' }
    })
    .catch(function (error) {
      const { status, data } = error.response
      return { status, msg: data }
    })
  return result
}
