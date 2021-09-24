const { app, BrowserWindow, ipcMain } = require('electron')
const path = require('path')

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