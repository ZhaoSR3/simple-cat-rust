use std::{env, fs, io};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() == 1 {
        let mut output: String = String::new();
        io::stdin().read_line(&mut output).expect("读取输入失败！");
        println!("{output}");
    } else {
        for argv in args.iter().skip(1) {
            let file = fs::read_to_string(argv).expect("文件未找到！");
            println!("{file}");
        }
    }
}
