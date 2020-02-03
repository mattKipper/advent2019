use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum SubProblem {
    One,
    Two
}

/// Advent of Code 2019
#[derive(StructOpt,Debug)]
pub struct Opts {
    /// Path to problem input
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Solve alternate problem
    #[structopt(short, long)]
    alternate: bool,
}


pub fn input() -> Option<(String, SubProblem)>
{
    let args = Opts::from_args();

    let sub = if args.alternate {
        SubProblem::Two
    } 
    else {
        SubProblem::One
    };

    match std::fs::read_to_string(args.input) {
        Ok(s) => Some((s, sub)),
        Err(e) => {
            println!("{}", e);
            None
        }
    }
}

pub mod intcode {

    use std::io::{Read,BufRead,Write,Error,ErrorKind};

    pub struct InputProvider {
        pub inputs: String
    }
    
    impl InputProvider {
        pub fn new(inputs: Vec<i64>) -> InputProvider {
            let str_inputs: Vec<String> = inputs.iter().map(i64::to_string).collect();
            let mut input: String = str_inputs.join("\n");
            input.push('\n');
            InputProvider { inputs: input }
        }
    }

    impl Read for InputProvider {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
            if self.inputs.is_empty() {
                Ok(0)
            }
            else {
                let mut bytes: usize = 0;
                for i in 0..std::cmp::min(buf.len(), self.inputs.len()) {
                    let c = self.inputs.remove(0);
                    buf[i] = c as u8;
                    bytes += 1;
                }
                Ok(bytes)
            }
        }
    }

    impl BufRead for InputProvider {
        fn fill_buf(&mut self) -> Result<&[u8], Error> {
            if self.inputs.is_empty() {
               Err(Error::new(ErrorKind::Other, "Input Exhausted"))
            }
            else {
                Ok(self.inputs.as_bytes())
            }
        }

        fn consume(&mut self, amt: usize) {
            for _ in 0..amt {
                self.inputs.remove(0);
            }
        }
    }

    pub struct OutputCollector {
        pub outputs: Vec<i64>
    }

    impl OutputCollector {
        pub fn new() -> OutputCollector {
            OutputCollector { outputs: Vec::new() }
        }
    }

    impl Write for OutputCollector {
        fn write(&mut self, buf: &[u8]) -> Result<usize,Error> {
            match std::str::from_utf8(buf) {
                Ok(s) => {
                    match s.trim().parse::<i64>() {
                        Ok(n) => { 
                            self.outputs.push(n);
                            Ok(buf.len())
                        }
                        Err(_) => Err(Error::new(ErrorKind::Other, "Parse error"))
                    }
                },
                Err(_) => Err(Error::new(ErrorKind::Other, "Parse error"))
            }
        }

        fn flush(&mut self) -> Result<(),Error> {
            Ok(())
        }
    }

    #[derive(Debug)]
    enum AddressingMode {
        Immediate,
        Position
    }

    #[derive(Debug)]
    struct Parameter {
        value: i64,
        mode: AddressingMode
    }

    #[derive(Debug)]
    enum OutputParameterType {
        Write,
        Jump
    }

    #[derive(Debug)]
    enum Instruction {
        Add(Parameter, Parameter, Parameter),
        Multiply(Parameter, Parameter, Parameter),
        Finish,
        Input(Parameter),
        Output(Parameter),
        JumpNonZero(Parameter, Parameter),
        JumpZero(Parameter, Parameter),
        LessThan(Parameter, Parameter, Parameter),
        Equals(Parameter, Parameter, Parameter)
    }

    impl Instruction {
        fn arg_count(command: &i64) -> usize {
            match command % 100 {
                1 => 3,
                2 => 3,
                3 => 1,
                4 => 1,
                5 => 2,
                6 => 2,
                7 => 3,
                8 => 3,
                99 => 0,
                n => panic!("Unexpected opcode: {}", n)
            }
        }
    }

    fn decode_args<'a, T>(mut program: T, opcode: &i64, count: usize) -> Vec<Parameter>
        where T: Iterator<Item=&'a i64> 
    { 
        let mut args = Vec::new();
        let mut opcode = opcode.clone();

        opcode /= 100;

        for _ in 0..count {
            let mode = match opcode % 10 { 
                0 => AddressingMode::Position,
                1 => AddressingMode::Immediate,
                n => panic!("Unexpected Addressing Mode: {}", n)
            };

            let param = Parameter {
                mode: mode,
                value: program.next().expect("Program ended during argument decode").clone()
            };

            args.push(param);

            opcode /= 10;
        }

        args
    }

    fn decode<'a, T>(mut program: T) -> Option<Instruction> 
        where T: Iterator<Item=&'a i64> 
    { 
        let opcode = program.next()?;
        let mut args = decode_args(program, opcode, Instruction::arg_count(&opcode));
        let mut dargs = args.drain(..);

        match opcode % 100 {
            1=> {
                let a = dargs.next().unwrap();
                let b = dargs.next().unwrap();
                let o = dargs.next().unwrap();
                Some(Instruction::Add(a, b, o))
            },
            2 => {
                let a = dargs.next().unwrap();
                let b = dargs.next().unwrap();
                let o = dargs.next().unwrap();
                Some(Instruction::Multiply(a, b, o))
            },
            3 => Some(Instruction::Input(dargs.next().unwrap())),
            4 => Some(Instruction::Output(dargs.next().unwrap())),
            5 => {
                let i = dargs.next().unwrap();
                let o = dargs.next().unwrap();
                Some(Instruction::JumpNonZero(i, o))
            },
            6 => {
                let i = dargs.next().unwrap();
                let o = dargs.next().unwrap();
                Some(Instruction::JumpZero(i, o))
            },
            7 => {
                let a = dargs.next().unwrap();
                let b = dargs.next().unwrap();
                let o = dargs.next().unwrap();
                Some(Instruction::LessThan(a, b, o))
            },
            8 => {
                let a = dargs.next().unwrap();
                let b = dargs.next().unwrap();
                let o = dargs.next().unwrap();
                Some(Instruction::Equals(a, b, o))
            },
            99 => Some(Instruction::Finish),
            n => panic!("Unexpected command: {}", n)
        }
    }

    fn input(program: &Vec<i64>, param: &Parameter) -> i64 {
        match param.mode {
            AddressingMode::Immediate => param.value,
            AddressingMode::Position => program[param.value as usize]
        }
    }

    fn output_index(program: &Vec<i64>, param: &Parameter, param_type: &OutputParameterType) -> usize {
        let index = param.value as usize;
        match param_type {
            OutputParameterType::Jump => {
                match param.mode {
                    AddressingMode::Immediate => index,
                    AddressingMode::Position => program[index] as usize
                }
            },
            OutputParameterType::Write => index
        }
    }

    fn execute_instruction<I,O>(program: &mut Vec<i64>, instruction: &Instruction, pc: &mut usize, is: &mut I, os: &mut O) where
        I: BufRead, O: Write {
        match instruction {
            Instruction::Add(a,b,o) => {
                let a = input(program, a);
                let b = input(program, b);
                let o = output_index(program, o, &OutputParameterType::Write);
                program[o] = a + b;
                *pc = *pc + 4;
            },
            Instruction::Multiply(a, b, o) => {
                let a = input(program, a);
                let b = input(program, b);
                let o = output_index(program, o, &OutputParameterType::Write);
                program[o] = a * b;
                *pc = *pc + 4;
            },
            Instruction::Input(o) => {
                let mut input_data = String::new();
                is.read_line(&mut input_data).unwrap();
                let o = o.value as usize;
                let v = input_data.trim().parse::<i64>().unwrap();
                program[o] = v;
                *pc = *pc + 2;
            },
            Instruction::Output(i) => {
                write!(os, "{}", input(program, i)).unwrap();
                os.flush().unwrap();
                *pc = *pc + 2;
            },
            Instruction::JumpNonZero(i, o) => {
                if input(program, i) != 0 {
                    *pc = output_index(program, o, &OutputParameterType::Jump);
                }
                else {
                    *pc = *pc + 3;
                }
            },
            Instruction::JumpZero(i, o) => {
                if input(program, i) == 0 {
                    *pc = output_index(program, o, &OutputParameterType::Jump);
                }
                else {
                    *pc = *pc + 3;
                }
            },
            Instruction::LessThan(a, b, o) => {
                let a = input(program, a);
                let b = input(program, b);
                let o = output_index(program, o, &OutputParameterType::Write);

                program[o] = if a < b { 1 } else { 0 };
                *pc = *pc + 4;
            },
            Instruction::Equals(a, b, o) => {
                let a = input(program, a);
                let b = input(program, b);
                let o = output_index(program, o, &OutputParameterType::Write);

                program[o] = if a == b { 1 } else { 0 };
                *pc = *pc + 4;
            },
            Instruction::Finish => {
                *pc = *pc + 1;
            }
        };
    }


    pub fn execute<I,O>(program: &mut Vec<i64>, input: &mut I, output: &mut O) where
        I: BufRead, O: Write
    {
        let mut pc: usize = 0;
        while pc < program.len() {

            match decode(program.iter().skip(pc)) {
                Some(Instruction::Finish) | None => {
                    break
                },
                Some(instruction) => {
                    execute_instruction(program, &instruction, &mut pc, input, output);
                }
            };
        }
    }

    pub fn parse_input(input: String) -> Vec<i64> {
        input.split(',')
             .map(|op| {
                 op.parse::<i64>().unwrap()
             })
             .collect()
    }
}