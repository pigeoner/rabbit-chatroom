<template>
  <span class="app-frame">
    <span class="minimize" @click="minimizeWindow">-</span>
    <span class="close" @click="closeWindow">×</span>
  </span>
</template>
<script setup>
// 右上角按钮
const ipcRenderer = window.electron.ipcRenderer
// 最小化窗口
const minimizeWindow = () => {
  ipcRenderer.send('minimize', {})
}
// 关闭窗口
const closeWindow = () => {
  console.log('close')
  ipcRenderer.send('close', {})
}
</script>
<style lang="less">
.app-frame {
  position: absolute;
  top: -0.25rem;
  right: 0;
  font-size: 1.5rem;
  font-weight: 500;
  -webkit-app-region: no-drag;
  span {
    box-sizing: content-box;
    padding: 5px;
    cursor: pointer;
    display: inline-block;
    width: 1.2rem;
    height: 1.2rem;
    text-align: center;
    line-height: 1rem;
    vertical-align: middle;
    transition: all 0.3s ease;
    color: #646464;
    &:hover {
      background-color: #dedede85;
    }
  }
  .close {
    &:hover {
      color: #fff;
      background-color: #fe3131cd;
    }
  }
}
</style>
