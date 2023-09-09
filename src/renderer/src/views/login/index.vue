<template>
  <div class="login-container" :class="{ logining: loginState, 'fade-out': fadeOut }">
    <el-container>
      <el-header>
        <span class="app-name">Rabbit Chatroom</span>
        <appframe />
        <span class="avatar">
          <img :src="rabbit" alt="" />
        </span>
        <span v-show="loginState" class="login-state">
          <span>{{ getLoginState() }}</span>
          <el-button v-if="loginState === -1" @click="loginState = 0">确定</el-button>
        </span>
      </el-header>
      <el-main>
        <el-form>
          <el-form-item class="username">
            <el-input v-model="username" placeholder="请输入用户名"></el-input>
          </el-form-item>
          <el-form-item class="password">
            <el-input v-model="password" placeholder="请输入密码" show-password></el-input>
          </el-form-item>
          <el-form-item class="login">
            <el-button type="primary" @click="login">登录</el-button>
          </el-form-item>
        </el-form>
      </el-main>
    </el-container>
  </div>
</template>
<script setup>
import rabbit from '@renderer/assets/images/rabbit.png'
import appframe from '@renderer/components/appFrame/index.vue'
import { ref } from 'vue'
import { useRouter } from 'vue-router'
const ipcRenderer = window.electron.ipcRenderer
const router = useRouter()

// 登录
const username = ref('')
const password = ref('')
const loginState = ref(0) // 0 未登录 1 登录中 2 登录成功 -1 登录失败
const fadeOut = ref(false)

const login = async () => {
  try {
    loginState.value = 1

    const res = await ipcRenderer.invoke('login', {
      username: username.value,
      password: password.value
    })
    await new Promise((resolve) => setTimeout(resolve, 1000))

    loginState.value = res.status === 200 ? 2 : -1
    await new Promise((resolve) => setTimeout(resolve, 1000))
    if (loginState.value === 2) {
      fadeOut.value = true
      ipcRenderer.send('after-login')
      await new Promise((resolve) => setTimeout(resolve, 500))
      router.push({ name: 'Home' })
    }
  } catch (error) {
    // Handle any errors that occurred during the login process
    console.error(error)
  }
}

const getLoginState = () => {
  switch (loginState.value) {
    case 0:
      return ''
    case 1:
      return '登录中...'
    case 2:
      return '登录成功'
    case -1:
      return '登录失败'
    default:
      return ''
  }
}
</script>
<style lang="less">
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}
.login-container {
  background-color: #fff;
  overflow: hidden;
  @keyframes show {
    0% {
      opacity: 0;
      transform: translateX(-440px);
    }
    100% {
      opacity: 1;
      transform: translateX(0);
    }
  }

  @keyframes hide {
    0% {
      opacity: 1;
      transform: translateX(0);
    }
    100% {
      opacity: 0;
      transform: translateX(440px);
    }
  }
  animation: show 0.5s ease-in-out;
  &.fade-out {
    animation: hide 0.5s ease-in-out;
  }
  .el-header {
    position: relative;
    height: 40vh;
    color: azure;
    transition: all 0.3s ease;
    background: linear-gradient(
      90deg,
      rgba(9, 9, 121, 1) 0%,
      rgb(51, 45, 155) 30%,
      rgb(136, 79, 229) 70%,
      rgba(0, 212, 255, 1) 100%
    );
    .avatar {
      position: fixed;
      left: 50%;
      bottom: 0;
      transform: translate(-50%, -240%);
      display: inline-block;
      border: 3px solid #e7e7e7a4;
      border-radius: 50%;
      width: 4.4rem;
      height: 4.4rem;
      z-index: 5;
      transition: all 0.3s ease;
      img {
        width: 100%;
        height: 100%;
        border-radius: 50%;
      }
    }
    .login-state {
      position: fixed;
      display: flex;
      flex-direction: column;
      left: 50%;
      top: 60%;
      transform: translate(-50%, 0);
      font-size: 1.2rem;
      font-weight: 500;
      z-index: 3;
      transition: all 0.3s ease;
      .el-button {
        -webkit-app-region: no-drag;
        margin-top: 2rem;
        background-color: #a3cff540;
        color: aliceblue;
      }
    }
    .app-name {
      position: absolute;
      top: 10px;
      left: 10px;
      font-size: 1.2rem;
      font-weight: 700;
    }
  }
  .el-main {
    position: relative;
    height: 60vh;
    overflow: hidden;
    transition: all 0.3s ease;
    -webkit-app-region: no-drag;
    .el-form {
      position: absolute;
      left: 50%;
      top: 50%;
      transform: translate(-50%, -50%);
      width: 60%;
      margin-top: 1rem;
      .el-form-item {
        .el-input {
          width: 100%;
          border-bottom: 1px solid #bdbdbda4;
        }
        .el-button {
          width: 100%;
        }
        & .el-input__wrapper {
          box-shadow: none;
        }
      }
      // mixins
      .usernamePasswordBefore() {
        font-family: 'Font Awesome 5 Free';
        font-weight: 900;
        color: #a2a2a2e3;
      }
      .username {
        margin-bottom: 10px;
        .el-input::before {
          content: '\f007';
          .usernamePasswordBefore();
        }
      }
      .password {
        .el-input::before {
          content: '\f023';
          .usernamePasswordBefore();
        }
      }
      .login {
        .el-button {
          background-color: rgb(0, 183, 255);
          border: none;
          box-shadow: 1px 1px 3px 0px #a2a2a2e3;
          height: 2.5rem;
        }
      }
    }
  }
}
.login-container.logining {
  .el-header {
    height: 100vh;
    .avatar {
      transform: translate(-50%, -250%);
    }
  }
  .el-main {
    height: 0vh;
    * {
      display: none;
    }
  }
}
</style>
