import { app, shell, BrowserWindow, ipcMain } from 'electron'
import { join } from 'path'
import { electronApp, optimizer, is } from '@electron-toolkit/utils'
import icon from '../../resources/icon.png?asset'

const windows = new Set<BrowserWindow>()

function createWindow(): void {
  // Create the browser window.
  const mainWindow = new BrowserWindow({
    width: 450,
    height: 650,
    show: false,
    // titleBarStyle: 'hidden',
    // titleBarOverlay: {
    //   color: '#eef2ff',
    //   symbolColor: '#74b1be'
    //   // height: 60
    // },
    autoHideMenuBar: true,
    maximizable: false,
    ...(process.platform === 'linux' ? { icon } : {}),
    webPreferences: {
      preload: join(__dirname, '../preload/index.js'),
      sandbox: false
    }
  })

  const child = new BrowserWindow({
    // parent: mainWindow,
    autoHideMenuBar: true,
    // fullscreen: true,
    show: false,
    ...(process.platform === 'linux' ? { icon } : {})
    // webPreferences: {
    //   preload: join(__dirname, '../preload/index.js'),
    //   sandbox: false
    // }
  })
  child.loadFile(join(__dirname, '../renderer/index.html'))

  mainWindow.on('ready-to-show', () => {
    mainWindow.show()
    windows.add(mainWindow)
  })

  mainWindow.on('closed', () => windows.delete(mainWindow))
  child.on('closed', () => windows.delete(child))

  ipcMain.handle('add-new-window', () => {
    if (child) {
      child.show()
      windows.add(child)
    }
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
