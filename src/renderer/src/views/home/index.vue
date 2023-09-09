<template>
  <div class="chatroom">
    <el-container>
      <!-- 左侧列表，包括头像、昵称、签名、好友列表、群聊列表 -->
      <el-aside width="50px" class="left">
        <span class="avatar"><img :src="rabbit" alt="" /></span>
        <span class="content-list">
          <span
            class="message-icon"
            :class="{ 'is-active': activeIcon === 0 }"
            @click="activeIcon = 0"
            ><i class="fa-comment" :class="activeIcon === 0 ? 'fas' : 'far'"
          /></span>
          <span class="user-icon" :class="{ 'is-active': activeIcon === 1 }" @click="activeIcon = 1"
            ><i class="fa-user" :class="activeIcon === 1 ? 'fas' : 'far'"
          /></span>
        </span>
      </el-aside>
      <!-- 左侧群聊和好友聊天列表 -->
      <el-aside width="240px">
        <div class="search-box">
          <el-input
            v-model="input2"
            size="small"
            class="w-50 m-2"
            placeholder="搜索"
            :prefix-icon="Search"
          />
        </div>
        <div v-if="activeIcon === 0" class="message-list">
          <span v-for="i in 15" :key="i" class="message-preview">
            <span class="message-avatar"><img :src="rabbit" alt="" /></span>
            <span class="message-info">
              <span class="message-title">潇洒哥</span>
              <span class="message-latest"
                >阿米诺斯阿米诺斯阿米诺斯阿米诺斯阿米诺斯阿米诺斯阿米诺斯</span
              >
            </span>
            <span class="latest-time">11:07</span>
          </span>
        </div>
      </el-aside>
      <!-- 右侧聊天窗口 -->
      <el-container>
        <el-header>
          <appframe />
          <span class="chat-target">潇洒哥</span>
        </el-header>
        <el-main>
          <!-- 消息展示 -->
          <div class="message-content"></div>
          <!-- 消息输入 -->
          <div class="message-input">
            <el-input type="textarea" />
          </div>
        </el-main>
      </el-container>
    </el-container>
  </div>
</template>
<script setup>
import appframe from '@renderer/components/appFrame/index.vue'
import rabbit from '@renderer/assets/images/rabbit.png'
import { ref, getCurrentInstance, onMounted, onBeforeUnmount } from 'vue'
import { Search } from '@element-plus/icons-vue'
const { proxy } = getCurrentInstance()
const ipcRenderer = window.electron.ipcRenderer
ipcRenderer.send('login-success')
// 0 消息 1 好友
const activeIcon = ref(0)

const sockets = {
  welcome(data) {
    console.log(data)
  }
}
proxy.$socket.io.emit('send', 'client send some data to node Serve.')
onMounted(() => {
  proxy.$addSockets(sockets, proxy)
})

onBeforeUnmount(() => {
  proxy.$removeSockets(sockets, proxy)
})
</script>
<style lang="less">
.chatroom {
  height: 100vh;
  width: 100vw;
  background-color: #fff;
  .el-container {
    height: 100%;
    width: 100%;
    .el-aside {
      position: relative;
      // background-color: #545c64;
      color: #fff;
      text-align: center;
      height: 100%;
      overflow: hidden;
      &:first-child {
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: center;
        background-color: #333333;
        padding-top: 1rem;
        .avatar {
          display: flex;
          justify-content: center;
          align-items: center;
          border: 3px solid #e7e7e7a4;
          border-radius: 50%;
          width: 40px;
          height: 40px;
          z-index: 5;
          transition: all 0.3s ease;
          img {
            width: 100%;
            height: 100%;
            object-fit: contain;
            border-radius: 50%;
          }
        }
        .content-list {
          margin-top: 1rem;
          height: 6rem;
          display: flex;
          flex-direction: column;
          justify-content: space-around;
          font-size: 1.2rem;
          -webkit-app-region: no-drag;
          span {
            padding: 0.5rem;
            cursor: pointer;
          }
        }
      }
      &:nth-child(2) {
        * {
          -webkit-app-region: no-drag;
        }
        .search-box {
          height: 60px;
          padding: 25px 15px 12px 15px;
        }
        .message-list {
          height: calc(100% - 60px);
          overflow-y: auto;
          &::-webkit-scrollbar {
            position: absolute;
            width: 6px;
            background-color: #f1f1f1;
          }
          &::-webkit-scrollbar-track {
            border-radius: 4px;
            background-color: #f1f1f1;
          }
          &::-webkit-scrollbar-thumb {
            border-radius: 4px;
            background-color: #bebebe86;
          }
          .message-preview {
            position: relative;
            display: flex;
            justify-content: flex-start;
            width: 100%;
            height: 60px;
            background-color: #fff;
            color: #808080;
            font-size: 14px;
            border-bottom: 1px solid #e7e7e7a4;
            padding-left: 0.6rem;
            user-select: none;
            cursor: pointer;
            .message-avatar {
              flex-shrink: 0;
              width: 40px;
              img {
                width: 100%;
                height: 100%;
                object-fit: contain;
              }
            }
            .message-info {
              display: flex;
              flex-direction: column;
              justify-content: space-evenly;
              align-items: flex-start;
              padding: 0.2rem 0.6rem;
              .message-title {
                color: #1f1f1f;
              }
              .message-latest {
                width: 160px;
                overflow: hidden;
                white-space: nowrap;
                text-overflow: ellipsis;
              }
            }
            .latest-time {
              position: absolute;
              right: 0.6rem;
              top: 1.1rem;
              transform: translateY(-50%);
              font-size: 12px;
            }
          }
        }
      }
    }
    .el-header {
      position: relative;
      background-color: #f6f6f6;
      height: 60px;
      border-bottom: 1px solid #dededeb5;
      .chat-target {
        position: absolute;
        left: 0;
        bottom: 50%;
        transform: translate(50%, 50%);
      }
    }
    .el-main {
      -webkit-app-region: no-drag;
      background-color: #f6f6f6;
      line-height: 160px;
      padding: 0;
      .message-content {
        height: 70%;
      }
      .message-input {
        position: relative;
        height: 30%;
        border-top: 1px solid #dadadab3;
        box-shadow: 0 -1px 5px #dadada55 inset;
        .el-textarea {
          position: absolute;
          top: 0;
          left: 0;
          width: 100%;
          height: 100%;
          textarea {
            height: 100%;
            padding: 0.6rem;
            resize: none;
            border: none;
            box-shadow: none;
            outline: none;
            border-radius: 0;
            font-size: 14px;
            &::-webkit-scrollbar {
              position: absolute;
              width: 6px;
              background-color: #f1f1f1;
            }
            &::-webkit-scrollbar-track {
              border-radius: 4px;
              background-color: #f1f1f1;
            }
            &::-webkit-scrollbar-thumb {
              border-radius: 4px;
              background-color: #bebebe86;
            }
          }
        }
      }
    }
  }
}
</style>
