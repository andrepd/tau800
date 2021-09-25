const renderer = require('./renderer')
const { ipcMain } = require('electron')

function onUpdate(update) {
    renderer.write_clock(update.numbers[0], update.numbers[1])
    update.registers.forEach((values, index) => {
        renderer.write_register(REGISTERS[index], values)
    })
    renderer.write_stack(update.stack)
    renderer.report_command_history(update.history)
}

window.addEventListener('DOMContentLoaded', () => {
    renderer.initialize()
    ipcMain.addListener('tau_update', onUpdate)
})
