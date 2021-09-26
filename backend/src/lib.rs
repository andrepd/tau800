use neon::prelude::*;
use rand::{distributions::Uniform, prelude::*};

const REGISTER_A: usize = 0;
const REGISTER_F: usize = 1;
const REGISTER_BH: usize = 2;
const REGISTER_BL: usize = 3;
const REGISTER_CH: usize = 4;
const REGISTER_CL: usize = 5;
const REGISTER_X: usize = 6;
const REGISTER_SP: usize = 7;
const REGISTER_PC: usize = 8;
const REGISTERS: [usize; 9] = [
    REGISTER_A,
    REGISTER_F,
    REGISTER_BH,
    REGISTER_BL,
    REGISTER_CH,
    REGISTER_CL,
    REGISTER_X,
    REGISTER_SP,
    REGISTER_PC,
];

fn poll(mut cx: FunctionContext) -> JsResult<JsObject> {
    // Dummy example: return random values
    let time_sampler = Uniform::new(0, 60);
    let numbers = JsArray::new(&mut cx, 2);
    let js_number = cx.number(time_sampler.sample(&mut thread_rng()));
    numbers.set(&mut cx, 0, js_number)?;
    let js_number = cx.number(time_sampler.sample(&mut thread_rng()));
    numbers.set(&mut cx, 1, js_number)?;

    let boolean_sampler = rand::distributions::Bernoulli::new(0.5).unwrap();
    let registers = {
        let registers = JsArray::new(&mut cx, 8);
        for i in 0..8 {
            let register = JsArray::new(&mut cx, 6);

            for j in 0..6 {
                let value = boolean_sampler.sample(&mut thread_rng());
                let value = cx.boolean(value);
                register.set(&mut cx, j, value)?;
            }

            registers.set(&mut cx, i, register)?;
        }
        registers
    };

    let stack = Uniform::new(0, 7).sample(&mut thread_rng());
    let stack = cx.number(stack);

    let history = {
        let values = ["aaaaaaa", "bbbbb", "cccccc", "dd", "eeee", "ffffffff"].iter();
        let history = JsArray::new(&mut cx, values.len() as u32);

        for (i, value) in values.enumerate() {
            let value = cx.string(value);
            history.set(&mut cx, i as u32, value)?;
        }

        history
    };

    let response_object = JsObject::new(&mut cx);

    response_object.set(&mut cx, "numbers", numbers)?;
    response_object.set(&mut cx, "registers", registers)?;
    response_object.set(&mut cx, "stack", stack)?;
    response_object.set(&mut cx, "history", history)?;

    Ok(response_object)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("poll", poll)?;
    Ok(())
}
