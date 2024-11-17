use clap::Parser;
use std::io::Write;
use std::io::BufRead;
use rand::Rng;
use colored::*;
use std::str::FromStr;

use killer_core::ArgOptions;
use killer_core::arp_poison;


use std::net::Ipv4Addr;

const VERSION: &str = "5.0";


#[derive(Parser, Debug)]
#[clap(version = "5.0", author = "xiaozhao45", disable_help_flag = true, disable_version_flag = true)]
struct Args {
    /// 子命令
    #[clap(subcommand)]
    command: Option<TerminalCommand>,
}

#[derive(clap::Subcommand, Debug)]
enum TerminalCommand {
    /// Kill 子命令
    Kill {
        /// 目标 IP
        #[clap(short = 't', long("target"))]
        target: String,
        /// 网关 IP
        #[clap(short = 'g', long("gateway"))]
        gateway: String,
        /// 接口名称
        #[clap(short = 'i', long("interface"))]
        interface: String,
    },
    /// Scan 子命令
    Scan {
        /// 网关 IP
        #[clap(short, long)]
        gateway: String,
    },
    /// Port 扫描子命令
    Port {
        /// 目标 IP
        target: String,
    },
    /// Net 子命令
    Net,
    /// 什么也不做
    None,
}

// #[derive(Parser)]
// #[clap(version = "5.0", author = "xiaozhao45", disable_help_flag = true, disable_version_flag = true)]
// struct Args {
//     #[clap(short = 'k', long = "kill")]
//     kill: bool,
//
//     #[clap(short = 's', long = "scan")]
//     scan: bool,
//
//     #[clap(short = 'n', long = "net")]
//     net: bool,
//
//     #[clap(short = 'p', long = "port")]
//     port: bool,
//
//     #[clap(short = 'a', long = "about")]
//     about: bool,
//
//     #[clap(short = 'c', long = "command-list")]
//     command_list: bool,
//
//     #[clap(short = 'v', long = "version")]
//     version_info: bool,
//
//     #[clap(short = 'h', long = "help")]
//     help: bool,
// }


fn tiptext() -> String {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..11);

    let tiptext = match random_index {
        0 => "建议您多开几个Killer序，这很方便，由于Killer的单个功能使用后会直接退出，而不是返回到主界面",
        1 => "非常希望您在Github上反馈错误，在关于中有Github地址",
        2 => "Arp攻击属于网络攻击的一种，可能您需要为所有后果负责，请谨慎使用！",
        3 => "您可以单独运行Killer4.0的某功能，只需要为Killer提供命令行参数。",
        4 => "Killer的Github地址：https://github.com/xiaozhao45/Killer",
        5 => "这个程序是我6年级的时候的一个小项目，希望您能喜欢！",
        6 => "这里是作者的一些提示，您完全可以相信这些提示。",
        7 => "Killer 5.0使用了Rust来重写，Killer 4.0则是纯Python开发。",
        8 => "至少未来5个版本，Killer将不会开发GUI版本",
        9 => "Killer的作者：xiaozhao45，如果您要Fork本项目，您可以修改这些提示信息，也可以留着，但请不要删除。",
        10 => "Killer 未来的开发趋势是自动化、多功能、跨平台，将专注于网络相关。",
        _ => unreachable!(),
    };

    tiptext.to_string()
}

fn error_print(error_level: u64,error_code: &str, error_message: &str, try_todo: &str) {
    if error_level == 0 {
        println!("{}", "致命错误!错误级别:0；正在退出...".red());
        println!("          错误代码：{}", error_code);
        println!("  :(      错误信息：{}", error_message);
        println!("          尝试解决：{}", try_todo);
        println!("{}", "===========================".red());
        std::process::exit(error_code.parse().unwrap());
    } else if error_level == 1 {
        println!("{}", "错误!错误级别:1；正在退出...".red());
        println!("          错误代码：{}", error_code);
        println!("  :(      错误信息：{}", error_message);
        println!("          尝试解决：{}", try_todo);
        println!("{}", "===========================".red());
        std::process::exit(error_code.parse().unwrap());
    } else if error_level == 2 {
        println!("{}", "警告!错误级别:2；请检查...".yellow());
        println!("          错误代码：{}", error_code);
        println!("  :(      错误信息：{}", error_message);
        println!("          尝试解决：{}", try_todo);
        println!("{}", "===========================".red());
    }
}

fn info_print(level: u64) {
    match level {
        0 => println!(r#"
    Done!


     _  __  _   _   _
    | |/ / (_) | | | |   ___   _ __
    | ' /  | | | | | |  / _ \ | '__|
    | . \  | | | | | | |  __/ | |
    |_|\_\ |_| |_| |_|  \___| |_|

    {}

    {}

    [S] [Scan]          扫描所有内网的活跃IP
    [N] [Net]           获取本机IP、网关和MAC地址
    [P] [Port]          扫描指定局域网IP的开放端口
    [L] [Language]      切换程序的当前语言
    [A] [About]         关于这个程序&帮助页面
    [C] [CommandList]   全部命令列表

    {}

        "#, tiptext().green(), "[K] [Kill]          使用ARP协议来攻击局域网内的主机".red(), "[E] [Exit]          退出程序".blue()),
        1 => println!(r#"
     _  __  _   _   _
    | |/ / (_) | | | |   ___   _ __
    | ' /  | | | | | |  / _ \ | '__|
    | . \  | | | | | | |  __/ | |
    |_|\_\ |_| |_| |_|  \___| |_|

    Killer {}
        "#, VERSION),
        _ => error_print(1, "0x00", "未知错误", "请联系开发者！"),
    }
}

/// 从用户获取输入，并返回输入的内容。
fn input(prompt: &str) -> String {
    print!("{}", prompt.blue());
    std::io::stdout().flush().unwrap(); // 确保提示符立即显示

    let mut buffer = String::new();

    // 从标准输入读取一行
    std::io::stdin().lock().read_line(&mut buffer).expect("读取输入时发生错误");

    // 去除末尾的换行符
    buffer.trim_end_matches('\n').to_string()
}



fn main() {
    let args = Args::parse();

    // if !check_initialization() {
    //     first_run();
    // }
    //Nerver check,These code will be cleaned at Release build.



    // 处理命令行参数
    match args.command {
        Some(command) => {
            match command {
                TerminalCommand::Kill { target, gateway, interface } => {
                    println!("Executing Kill with target: {}, gateway: {}, interface: {}", target, gateway, interface);
                },
                TerminalCommand::Scan { gateway } => {
                    println!("Executing Scan with gateway: {}", gateway);
                },
                TerminalCommand::Port { target } => {
                    println!("Executing Port with target: {}", target);
                },
                TerminalCommand::Net => {
                    println!("Executing Net command");
                },
                _ => {
                    interaction_mode()
                }
            }
        },
        None => {
            interaction_mode()
        },
    }

}

// fn windows_decision() -> bool{
//     if !cfg!(windows) {
//         return true
//     }
//     false
// }


fn interaction_mode() {
    info_print(0);
    loop {
        let choice = input("Killer >>> ").to_uppercase();

        match choice.as_str() {
            "K" => {
                #[cfg(target_os = "linux")]
                {
                    println!("{}", "在使用前确保此程序已授予超级管理员权限(root或administrator)!".red());
                    println!("{}", "配置攻击参数以发送Arp包：".green());
                    let user_interface = input("输入网络接口名 >>> ");
                    let user_target_ip = input("输入目标IP >>> ");
                    let user_fake_ip = input("输入网关的IP >>> ");

                    println!("{}", "最后警告：此程序的开发者不承担用户（你）使用此程序所带来的后果，如果不同意这项规定请不要使用此程序，是否了解？".red());
                    println!(r#"
    [Y] 是的，继续使用，并且我同意遵守此程序使用协议。
    [N] 不同意，退出程序，并且停止使用此程序。
    "#);

                    if ["y", "yes", "Y", "YES"].contains(&input(">>> ").to_lowercase().as_str()) {
                        let options = ArgOptions {
                            interface: user_interface.to_string(),
                            target_ip: Ipv4Addr::from_str(&*user_target_ip).unwrap(),
                            gateway_ip: Ipv4Addr::from_str(&*user_fake_ip).unwrap(),
                            ip_forward: true,
                            log_traffic: false,
                        };
                        arp_poison(options);
                    } else {
                        exit(0, "用户不同意条款。")
                    }
                }
                #[cfg(target_os = "windows")]
                {

                }


            }
            "S" => {
                // let gateway_ip = input("输入网关IP >>> ");
                // scan(gateway_ip.parse().unwrap());
            }
            "N" => {

            }
            "P" => {
                println!("Killer Pre-view (Port Scan)");
            }
            "A" => {
                about();
            }
            "C" => {
                command_list();
            }
            "E" => {
                exit(0, "用户选择退出程序");
                break
            }
            "H" => {
                help();
            }
            _ => {
                error_print(2, "0x00", "无效的命令", "请输入正确的命令！")
            }
        }
    }
}

fn about() {
    println!(r#"
    Killer v5.0   (杀手 v5.0)
    这是 Killer 的 Rust 版本，旨在成为一个高性能和跨平台的程序。

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



    "#);
}

fn command_list() {
    println!(r#"
    命令列表说明：首个方括号内为别名，第二个方括号内为完整命令名称，可以在终端中作为killer的参数使用，只接受小写。

    [K] [Kill]          使用ARP协议来攻击局域网内的主机
    
    [S] [Scan]          扫描所有内网的活跃IP
    [N] [Net]           获取本机IP、网关和MAC地址
    [P] [Port]          扫描指定局域网IP的开放端口
    [A] [About]         关于这个程序&帮助页面
    [C] [CommandList]   全部命令列表
    [L] [Logfile]       输出日志文件

    [E] [Exit]          退出程序
    "#);
}

fn help() {
        println!(r#"
    Killer v5.0   (杀手 v5.0)
    这是 Killer 的 Rust 版本，旨在成为一个高性能和跨平台的程序。

用法：
        killer [选项]
            或
        ./killer [选项]

        以上用法的"killer"请替换为Killer可执行文件的文件名。
        若Killer没有在环境变量中，那么你需要在Killer可执行文件的目录中执行。
        没有参数时，进入交互模式。
        通过Killer-Toolkit安装的用户，可直接在任意终端中运行killer或Killer命令。

选项：
            -k | --kill      进行Arp攻击
                -i | --interface  网络接口名
                -t | --target     目标IP
                -g | --gateway    网关IP
            -------------------
            -s | --scan      扫描所有内网的活跃IP
                -g | --gateway    网关IP
            -------------------
            -n | --net       获取本机IP、网关和MAC地址
            -p | --port      扫描指定局域网IP的开放端口
                -t | --target     目标IP
            =================
            -h | --help      显示帮助信息
            -a | --about     关于这个程序&帮助页面
            -c | --command   全部命令列表

    "#);
    }

fn exit(exit_code : i32, exit_message: &str) {
    
    println!("{}", "正在退出...".blue());
    println!("          退出代码：{}", exit_code);
    println!("  :)      提示信息：{}", exit_message);
    println!("          感谢使用，再见！");
    println!("{}", "===========================

    ".red());
    std::process::exit(exit_code);
}

// fn scan(gateway_ip: Ipv4Addr) {

// }