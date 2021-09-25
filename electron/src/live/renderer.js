let initialized = false
let clock_left
let clock_right
let stack_boxes
let register_labels
let register_boxes
let cmd_displays

function initialize() {
    if (document == undefined) {
        throw Error('Tried to initialize renderer before DOM was loaded.')
    }

    clock_left = document.getElementById('clock-left')
    clock_right = document.getElementById('clock-right')

    stack_boxes = document.querySelectorAll('.stack-box > g > rect')
    register_labels = document.querySelectorAll('.register > .label')
    register_boxes = Array.from(document.getElementsByClassName('register'))
        .map((register_group) =>
            Array.from(register_group.getElementsByClassName('register-box')).map(
                (element) => element.querySelector('rect'))
        )
    console.log(register_boxes)

    cmd_displays = document.getElementsByClassName('command-display')

    initialized = true
}

function write_clock(hours, minutes) {
    if (!initialized) {
        throw Error('Need to initialize the renderer first.')
    }

    clock_left.innerHTML = hours
    clock_right.innerHTML = minutes
}

function report_command_history(commands) {
    commands.forEach((value, index) => { cmd_displays[index] = value })
}

function write_register(register, values) {
    let register_group = register_boxes[register]

    register_group.forEach((box, index) => {
        if (values[index]) {
            box.style.fill = '#ffffff'
        } else {
            box.style.fill = 'none'
        }
    })
}

module.exports = {
    initialize,
    write_clock,
    report_command_history,
    write_register,
}