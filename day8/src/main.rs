type OP<'a> = (Cmd, i32);
type OPS<'a> = Vec<OP<'a>>;
enum Cmd {
    Nop,
    Acc,
    Jmp,
}

fn main() {
    let mut ops = include_str!("input.txt")
        .lines()
        .map(get_tuples)
        .collect::<Vec<OP>>();
    // part 1
    execute(&ops);
    // part 2
    for i in 0..ops.len() {
        match ops[i].0 {
            Cmd::Jmp => {
                ops[i].0 = Cmd::Nop;
                execute(&mut ops);
                ops[i].0 = Cmd::Jmp;
            }
            Cmd::Nop => {
                ops[i].0 = Cmd::Jmp;
                execute(&mut ops);
                ops[i].0 = Cmd::Nop;
            }
            Cmd::Acc => {
                continue;
            }
        }
    }
}

fn get_tuples(op: &str) -> (Cmd, i32) {
    let args: Vec<&str> = op.split(' ').collect();
    (
        match args[0] {
            "nop" => Cmd::Nop,
            "acc" => Cmd::Acc,
            "jmp" => Cmd::Jmp,
            _ => panic!("unknown operation"),
        },
        args[1].parse().unwrap(),
    )
}

fn execute(ops: &OPS) {
    let mut acc: i32 = 0;
    let mut pc: i32 = 0;
    let mut visited = vec![false; ops.len()];

    loop {
        if pc as usize >= ops.len() {
            println!("PC out of range, acc is : {}", acc);
            break;
        }
        if visited[pc as usize] {
            println!("Infinate loop detected, acc is {}", acc);
            break;
        }
        visited[pc as usize] = true;
        match ops[pc as usize].0 {
            Cmd::Nop => pc += 1,
            Cmd::Acc => {
                acc += ops[pc as usize].1;
                pc += 1
            }
            Cmd::Jmp => pc += ops[pc as usize].1,
        }
    }
}
