static MEM_SIZE: uint = 5000;

pub struct BrainfuckInterpreter
{
    pointer: uint,
    mem: [u8, ..MEM_SIZE],
}

impl BrainfuckInterpreter
{
    pub fn new() -> BrainfuckInterpreter
    {
        BrainfuckInterpreter {
            pointer: 0,
            mem: [0, ..MEM_SIZE],
        }
    }

    pub fn run(&mut self, program: &str)
    {
        let mut pc: uint = 0;
        let mut loops: ~[uint] = ~[];

        let get_matching_paren = || {
            let mut tmp = pc;

            while tmp < program.len()
            {
                if program[tmp] == ']' as u8
                {
                    return tmp;
                }

                tmp += 1;
            }

            fail!("No matching ']' for index {}", pc);
        };

        while pc < program.len()
        {
            let c = program[pc] as char;
            pc += 1;

            match c
            {
                '>' => {
                    self.pointer += 1;
                    while self.pointer >= MEM_SIZE
                    {
                        self.pointer -= MEM_SIZE;
                    }
                }
                '<' => {
                    self.pointer -= 1;
                    while self.pointer < 0
                    {
                        self.pointer += MEM_SIZE;
                    }
                }
                '+' => {
                    self.mem[self.pointer] += 1;
                }
                '-' => {
                    self.mem[self.pointer] -= 1;
                }
                '.' => {
                    print!("{:c}", self.mem[self.pointer] as char);
                }
                '[' => {
                    if self.mem[self.pointer] == 0
                    {
                        pc = get_matching_paren();
                    }
                    else
                    {
                        loops.push(pc - 1);
                    }
                }
                ']' => {
                    if self.mem[self.pointer] != 0
                    {
                        if loops.len() == 0
                        {
                            fail!("No matching '[' for index {}", pc);
                        }

                        pc = loops.pop();
                    }
                }
                _ => {}
            }
        }
    }
}
