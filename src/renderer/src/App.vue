<script setup></script>

<template>
  <router-view />
</template>

<style lang="less">
html,
body {
  // 使得整个窗口可拖动，但是这样会导致输入框无法输入且按钮无法点击，
  // 所以需要在相应的位置上加上-webkit-app-region: no-drag
  -webkit-app-region: drag;
  &::-webkit-scrollbar {
    display: none;
  }
}
</style>
