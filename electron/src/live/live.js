const renderer = require('./renderer')
const { REGISTERS } = require('./registers')

function onUpdate(event, update) {
    renderer.write_clock(update.numbers[0], update.numbers[1])
    update.registers.forEach((value, index) => {
        renderer.write_register(REGISTERS[index], value)
    })
    renderer.write_stack(update.stack)
    renderer.report_command_history(update.history)
}

window.addEventListener('DOMContentLoaded', () => {
    renderer.initialize()
    window.electron.registerTauUpdate(onUpdate)
})
