const renderer = require('./renderer')
const { sleep } = require('./../sleep')
const { REGISTER_A } = require('./registers')

window.addEventListener('DOMContentLoaded', () => {
    renderer.initialize()
    main()
})

async function main() {
    let timer = 0
    let bits
    let bit_remainder
    while (true) {
        renderer.write_clock(
            String(Math.floor(timer/60)).padStart(2, '0'),
            String(timer%60).padStart(2, '0'))

        bits = Array(6)
        bit_remainder = timer
        for (let i = 5; i >= 0; i--) {
            if (2**i <= bit_remainder) {
                bits[i] = true
                bit_remainder -= 2**i
            } else {
                bits[i] = false
            }
        }
        renderer.write_register(REGISTER_A, bits)

        timer++
        await sleep(1000)
    }
}