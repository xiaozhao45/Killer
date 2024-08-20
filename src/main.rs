use std::{fs, io::{self, BufRead, BufReader, Write}};
use rand::Rng; // random tip text
use colored::*; // terminal coloring
use std::process; // exit program
use std::path::Path;

fn first_run() {
    let workspace_path = "KillerWorkSpace";
    let killer_standard_path = Path::new(workspace_path).join("killer.standard");
    let config_conf_path = Path::new(workspace_path).join("config.conf");

    match fs::create_dir_all(workspace_path) {
        Ok(_) => println!("创建工作空间文件夹成功: {}", workspace_path),
        Err(e) => error_print("0x0x", &format!("创建工作空间文件夹失败: {}", e), "请检查磁盘空间或文件权限"),
    }

    match fs::write(killer_standard_path.clone(), "initialized") { // 注意这里使用了 `.clone()`
        Ok(_) => println!("创建初始化标记文件成功: {:?}", killer_standard_path),
        Err(e) => error_print("0x0x", &format!("创建初始化标记文件失败: {}", e), "请检查磁盘空间或文件权限"),
    }

    match fs::write(config_conf_path.clone(), "configured") {
        Ok(_) => println!("创建配置文件成功: {:?}", config_conf_path),
        Err(e) => error_print("0x0x", &format!("创建配置文件失败: {}", e), "请检查磁盘空间或文件权限"),
    }
}

fn check_initialization() -> bool {
    let workspace_path = Path::new("KillerWorkSpace");
    let killer_standard_path = workspace_path.join("killer.standard");
    let config_conf_path = workspace_path.join("config.conf");

    fs::metadata(killer_standard_path).is_ok() && fs::metadata(config_conf_path).is_ok()
}

// fn translate_and_print(text:&str){
//     if launguage == "zh-CN"{
//         println!("{}",text);
//     } else if launguage == "en-US"{
//         println!("{}",text);
//     } else {
//         println!("{}",text);
//     }
// }

fn tiptext() -> String {


    const TIPTEXT_01: &str = "建议您多开几个Killer程序，这很方便，由于Killer的单个功能使用后会直接退出，而不是返回到主界面";
    const TIPTEXT_02: &str = "非常希望您在Github上反馈错误，在关于中有Github地址";
    const TIPTEXT_03: &str = "Arp攻击属于网络攻击的一种，可能您需要为所有后果负责，请谨慎使用！";
    const TIPTEXT_04: &str = "您可以单独运行Killer4.0的某功能，只需要在Github上下载对应Py文件并配置环境";
    const TIPTEXT_05: &str = "Killer的Github地址：https://github.com/xiaozhao45/Killer";
    const TIPTEXT_06: &str = "这个程序是我6年级的时候的一个小项目，希望您能喜欢！";
    const TIPTEXT_07: &str = "这里是作者的一些提示，您完全可以相信这些提示。";
    const TIPTEXT_08: &str = "Killer 5.0使用了Rust来重写，Killer 4.0则是纯Python开发。";
    const TIPTEXT_09: &str = "至少未来5个版本，Killer将不会开发GUI版本";
    const TIPTEXT_10: &str = "Killer的作者：xiaozhao45，如果您要Fork本项目，您可以修改这些提示信息，也可以留着，但请不要删除。";
    const TIPTEXT_11: &str = "Killer 未来的开发趋势是自动化、多功能、跨平台，将专注于网络相关。";


    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..11);

    let tiptext = match random_index {
        0 => TIPTEXT_01,
        1 => TIPTEXT_02,
        2 => TIPTEXT_03,
        3 => TIPTEXT_04,
        4 => TIPTEXT_05,
        5 => TIPTEXT_06,
        6 => TIPTEXT_07,
        7 => TIPTEXT_08,
        8 => TIPTEXT_09,
        9 => TIPTEXT_10,
        10 => TIPTEXT_11,
        _ => unreachable!(),
    };

    tiptext.to_string()
}




fn error_print(error_code:&str, error_message:&str, try_todo:&str){
    println!();
    println!("          致命错误：{}",error_code);
    println!("  :(      错误信息：{}",error_message);
    println!("          尝试解决：{}",try_todo);
    println!();
    std::process::exit(error_code.parse().unwrap());

}

// fn get_user_input(prompt: &str) -> String {
//     println!("{}", prompt);

//     let stdin = io::stdin();
//     let mut reader = BufReader::new(stdin.lock());

//     let mut user_input = String::new();
//     reader.read_line(&mut user_input).expect("读取输入时发生错误");

//     // 去除末尾的换行符
//     user_input.trim_end().to_string()
// }

/// 从用户获取输入，并返回输入的内容。
/// 从用户获取输入，并返回输入的内容。
fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // 确保提示符立即显示

    let mut buffer = String::new();

    // 从标准输入读取一行
    io::stdin().lock().read_line(&mut buffer).expect("读取输入时发生错误");

    // 去除末尾的换行符
    buffer.trim_end_matches('\n').to_string()
}





fn main(){
    let mut program_language = "zh-CN";
    if !check_initialization() {
        first_run();
    }
    print!("Done!\n\n");   // print message
    print!(r#"
     _  __  _   _   _
    | |/ / (_) | | | |   ___   _ __
    | ' /  | | | | | |  / _ \ | '__|
    | . \  | | | | | | |  __/ | |
    |_|\_\ |_| |_| |_|  \___| |_|

    "#);   // header text



    println!("{}", tiptext());


    print!("{}", r#"

    [K] 使用Arp攻击一个内网的计算机
    "#.red());

    println!("{}", r#"
    [S] 扫描所有内网的活跃IP
    [N] 获取本机IP、网关和MAC地址
    [P] 扫描指定局域网IP的开放端口
    [A] 关于这个程序&帮助页面
    [C] 全部命令列表
    [E] 退出程序
    "#.green());

    loop {
        let choice = input("Killer >>> ");

        match choice.as_str() {
            "K" => {
                kill()
            },
            "S" => {
                scan()
            },
            "N" => {
                net()
            },
            "P" => {
                port()
            },
            "A" => {
                about()
            },
            "C" => {
                command_list()
            },
            "E" => {
                println!("感谢使用!");
                break;
            },
            _ => {

                println!("\n错误代码：0x04");
                println!("非致命错误，无需异常退出。请检查输入")
            }
        }

    }
}

//
//     //while loop
//     while active {
//         let mut command = input("Killer >>> ");
//
//         if command == "K" {
//             println!("Killer Pre-view")
//
//         } else if command == "S" {
//             println!("Killer Pre-view")
//         } else if command == "N" {
//             println!("Killer Pre-view")
//
//
//         //     println!(r#"
//         //     ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//         //    ┃┃       NetWork Info
//         //    ┃┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//         //    ┃┃ 网络状态 ：{}┃ 位置 ：{}
//         //    ┃┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//         //    ┃┃ 网关地址 []
//         //    ┃┃ 本机地址 []
//         //    ┃┃ 物理地址 []
//         //    ┃┣──────────────────────────━━
//         //    ┃┃
//         //    ┃┃
//         //    ┃┃
//         //    ┃┗━ ━━━━━━━━━━━━━━━┳━━━━━━━━
//         //    ┗━ ━━━━━━━━━━━━━━━━┛
//
//         //     "#)
//         } else if command == "P" {
//             println!("Killer Pre-view")
//         } else if command == "A" {

//
//
//         } else if command == "C" {
//             println!("Killer Pre-view");
//             println!("当前语言：{}",program_language);
//         } else if command == "E" {
//             println!( "正在退出...");
//             process::exit(0);
//         } else {
//             println!("未知命令，请重新输入！");
//         }
//     }
// }


// Anybody Here？



//*****************************//
//功能函数 - Kill;Scan;Net;Port //
//*****************************//
//Last Update: 2024/07/12
//***********//

fn kill() {
    println!("Killer Pre-view")
}

fn scan() {
    println!("Killer Pre-view")
}

fn net() {
    println!("Killer Pre-view")
}

fn port() {
    println!("Killer Pre-view")
}

fn about() {
    println!(r#"
    关于程序：
             Killer v5.0
             [RustVersion]
             在这个版本，Killer使用Rust重写，旨在高性能和跨平台。
             **********************
    作者信息：
             2024年，由xiaozhao45编写而成。
             开源于Github，完全自由、免费使用。
             https://github.com/xiaozhao45/Killer
    免责声明：
             *****************************
             自你运行此软件，你将承担所有风险。
             作者不对进行非法用途产生的任何后果负责。
             ************************
    错误代码：
             0x00：未连接到互联网
             0x01：无法连接到目标主机
             0x02：被用户中断操作
             0x03：参数错误
             0x04：未知的命令，检查输入
             0x0x：未知错误，请携带日志提交issues
    帮助信息：
             1.在Killer主程序目录下会创建KillerWorkSpace文件夹，这保证了Killer不会让运行目录变得乱糟糟。
             2.同样在安装目录下的config目录下有Killer的配置文件，即程序设置。
             3.未知的命令会尝试在系统终端执行，我的意思是，Killer内置终端，同时是命令行工具。
             4.在任意终端中,输入'killer'可以运行Killer，同时可以添加参数来运行指定功能。

    "#);
}

fn command_list() {
    println!(r#"
    说明：首个方括号内为别名，第二个方括号内为完整命令名称，可以在终端中作为killer的参数使用，均无大小写要求。

    [S] [Scan]          扫描所有内网的活跃IP
    [N] [Net]           获取本机IP、网关和MAC地址
    [P] [Port]          扫描指定局域网IP的开放端口
    [A] [About]         关于这个程序&帮助页面
    [C] [CommandList]   全部命令列表
    [L] [Logfile]       输出日志文件

    [E] [Exit]          退出程序
    "#)

}