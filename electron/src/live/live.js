const renderer = require('./renderer')
const { sleep } = require('./../sleep')

window.addEventListener('DOMContentLoaded', () => {
    renderer.initialize()
    main()
})

async function main() {
    let timer = 0
    while (true) {
        renderer.write_clock(
            String(Math.floor(timer/60)).padStart(2, '0'),
            String(timer%60).padStart(2, '0'))

        timer++
        await sleep(1000)
    }
}