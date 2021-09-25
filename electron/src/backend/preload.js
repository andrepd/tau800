const { contextBridge, ipcRenderer } = require('electron')
const { sleep } = require('./../sleep')

contextBridge.exposeInMainWorld(
    'electron',
    {}
)

(async () => {
    let poll_response
    while(true) {
        // TODO: Poll
        poll_response = {}

        ipcRenderer.invoke('tau_update', poll_response)
        await sleep(2)
    }
})()