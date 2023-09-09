import { app, shell, BrowserWindow, ipcMain, Menu } from 'electron'
import { join } from 'path'
import { electronApp, optimizer, is } from '@electron-toolkit/utils'
import icon from '../../resources/icon.png?asset'
import login from './login'
import store from './utils/store'

let mainWindow = null

function createWindow() {
  // Create the browser window.
  mainWindow = new BrowserWindow({
    width: 440,
    height: 340,
    show: false,
    autoHideMenuBar: true,
    transparent: true, // 窗口透明
    ...(process.platform === 'linux' ? { icon } : {}),
    webPreferences: {
      preload: join(__dirname, '../preload/index.js'),
      sandbox: false
    },
    frame: false,
    resizable: false,
    titleBarStyle: 'hidden'
  })

  mainWindow.on('ready-to-show', () => {
    mainWindow.show()
  })

  mainWindow.webContents.setWindowOpenHandler((details) => {
    shell.openExternal(details.url)
    return { action: 'deny' }
  })

  // HMR for renderer base on electron-vite cli.
  // Load the remote URL for development or the local html file for production.
  if (is.dev && process.env['ELECTRON_RENDERER_URL']) {
    mainWindow.loadURL(process.env['ELECTRON_RENDERER_URL'])
  } else {
    mainWindow.loadFile(join(__dirname, '../renderer/index.html'))
  }
}

// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.
app.whenReady().then(() => {
  // Set app user model id for windows
  electronApp.setAppUserModelId('com.electron')

  // Default open or close DevTools by F12 in development
  // and ignore CommandOrControl + R in production.
  // see https://github.com/alex8088/electron-toolkit/tree/master/packages/utils
  app.on('browser-window-created', (_, window) => {
    optimizer.watchWindowShortcuts(window)
  })
  store.clear()
  createWindow()

  app.on('activate', function () {
    // On macOS it's common to re-create a window in the app when the
    // dock icon is clicked and there are no other windows open.
    if (BrowserWindow.getAllWindows().length === 0) createWindow()
  })
})

// Quit when all windows are closed, except on macOS. There, it's common
// for applications and their menu bar to stay active until the user quits
// explicitly with Cmd + Q.
app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit()
  }
})

// In this file you can include the rest of your app"s specific main process
// code. You can also put them in separate files and require them here.
ipcMain.on('minimize', () => {
  BrowserWindow.getFocusedWindow()?.minimize()
})

ipcMain.on('close', () => {
  BrowserWindow.getFocusedWindow()?.close()
})

ipcMain.handle('login', async (event, userInfo) => {
  const result = await login(userInfo)
  console.log(result)
  return result
})

ipcMain.handle('get-token', async () => {
  const token = await store.get('token')
  return token
})

ipcMain.on('after-login', () => {
  // mainWindow.setOpacity(1) // 设置窗口初始透明度为 0
  // mainWindow.animate({ opacity: 0 }, 1000) // 使用 CSS 动画将窗口透明度渐变为 1，持续时间为 1000 毫秒
  setTimeout(() => {
    mainWindow.setSize(0, 0, true)
  }, 400)
})

ipcMain.on('login-success', () => {
  mainWindow.setSize(1080, 640, true)
  mainWindow.center()
  mainWindow.setResizable(true)
})
