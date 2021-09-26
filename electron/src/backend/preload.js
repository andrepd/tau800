const { contextBridge, ipcRenderer } = require('electron')

contextBridge.exposeInMainWorld(
    'electron', {
        registerTauUpdate: (callback) => ipcRenderer.on('tauUpdate', callback)
    }
)