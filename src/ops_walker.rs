use crate::{Cell, Instruction};
use crate::buffer::{VecBuffer};

pub fn exec_ops(ops: &Vec<Instruction<u8>>) {
    let mut buffer: VecBuffer<u8> = VecBuffer::default();
    walk_ops(ops, &mut buffer);
    println!("{:?}", buffer);
}

fn walk_ops<T>(ops: &Vec<Instruction<T>>, buffer: &mut VecBuffer<T>)
    where
        T: Cell
{
    for op in ops {
        match op {
            Instruction::NoOp => {},
            Instruction::IncreaseIndex(value) => {buffer.inc_index(*value)},
            Instruction::DecreaseIndex(value) => {buffer.dec_index(*value)},
            Instruction::IncreaseValue(value) => {buffer.inc_value(value)},
            Instruction::DecreaseValue(value) => {buffer.dec_value(value)},
            Instruction::IoRead => {
                buffer.set_value(buffer.index, read())
            },
            Instruction::IoWrite => {
                write(buffer.get_value(buffer.index))
            },
            Instruction::Loop(loop_ops) => {
                let break_value: T = T::default();
                while buffer.get_value(buffer.index) != break_value {
                    walk_ops(loop_ops, buffer)
                }
            }
        }
    }
}

fn read<T>() -> T
    where
        T: Cell
{
    unimplemented!()
    //let mut buffer = String::new();
    //let result = io::stdin().read_line(&mut buffer);
    //buffer.bytes().next().unwrap_or_default().into()
}

fn write<T>(value: T)
    where
        T: Cell
{
    let c: char = value.to_char();
    print!("{}", c)
}