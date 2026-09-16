// Based on the existing maginet-deploy Electron shell.
const { app, BrowserWindow, shell } = require('electron');
const path = require('node:path');

function openExternal(url) {
    if (url.startsWith('https://') || url.startsWith('http://')) {
        shell.openExternal(url);
    }
}

function createWindow() {
    const window = new BrowserWindow({
        useContentSize: true,
        autoHideMenuBar: true,
        width: 800,
        height: 584,
        minWidth: 800,
        minHeight: 584,
        center: true,
        title: app.getName(),
        show: false,
        icon: path.join(__dirname, 'static/png/appicon.ico'),
        webPreferences: {
            contextIsolation: true,
            nodeIntegration: false,
            sandbox: true,
        },
    });
    window.setMenu(null);
    window.webContents.setWindowOpenHandler(({ url }) => {
        openExternal(url);
        return { action: 'deny' };
    });
    window.webContents.on('will-navigate', (event, url) => {
        event.preventDefault();
        openExternal(url);
    });
    window.loadFile(path.join(__dirname, 'index.html'));
    window.once('ready-to-show', () => window.show());
}

app.whenReady().then(() => {
    createWindow();
    app.on('activate', () => {
        if (BrowserWindow.getAllWindows().length === 0) createWindow();
    });
});
app.on('window-all-closed', () => app.quit());
