const { app, BrowserWindow } = require('electron')
const path = require('path')
const { sleep } = require('./../sleep')
const backend = require('anachronic')

let win;

function createWindow() {
	win = new BrowserWindow({
		width: 800,
		height: 600,
		webPreferences: {
			nodeIntegration: true,
			preload: path.join(__dirname, 'preload.js'),
			nativeWindowOpen: true,
		}
	})

	win.loadFile('main.html')

	win.webContents.on('did-finish-load', () => updateLoop())
}

async function updateLoop() {
	let poll_response
	while (true) {
		poll_response = backend.poll()
		win.webContents.send('tauUpdate', poll_response)
		await sleep(100)
	}
}

app.whenReady().then(createWindow)

app.on('window-all-closed', () => {
	if (process.platform != 'darwin') {
		app.quit()
	}
})

app.on('activate', () => {
	if (BrowserWindow.getAllWindows().length === 0) {
		createWindow()
	}
})