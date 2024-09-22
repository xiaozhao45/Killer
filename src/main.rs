use clap::{Parser};
use std::{fs, io::Write, path::Path, thread};
use std::io::BufRead;
use rand::Rng;
use colored::*;
use std::process;
use std::str::FromStr;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use std::env::consts::OS;


extern crate killer_core;

use killer_core::ArgOptions;
use killer_core::arp_poison;


use pnet::datalink::MacAddr;
use std::net::Ipv4Addr;
use std::process::Command;
use pnet::packet::arp::{self, ArpPacket, MutableArpPacket};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
use pnet::packet::Packet;
use pnet::util::{checksum};





#[derive(Parser)]
#[clap(version = "5.0", author = "xiaozhao45", disable_help_flag = true, disable_version_flag = true)]
struct Args {
    #[clap(short = 'k', long = "kill")]
    kill: bool,

    #[clap(short = 's', long = "scan")]
    scan: bool,

    #[clap(short = 'n', long = "net")]
    net: bool,

    #[clap(short = 'p', long = "port")]
    port: bool,

    #[clap(short = 'a', long = "about")]
    about: bool,

    #[clap(short = 'c', long = "command-list")]
    command_list: bool,

    #[clap(short = 'v', long = "version")]
    version_info: bool,

    #[clap(short = 'h', long = "help")]
    help: bool,
}


fn tiptext() -> String {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..11);

    let tiptext = match random_index {
        0 => "建议您多开几个Killer程序，这很方便，由于Killer的单个功能使用后会直接退出，而不是返回到主界面",
        1 => "非常希望您在Github上反馈错误，在关于中有Github地址",
        2 => "Arp攻击属于网络攻击的一种，可能您需要为所有后果负责，请谨慎使用！",
        3 => "您可以单独运行Killer4.0的某功能，只需要在Github上下载对应Py文件并配置环境",
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

/// 从用户获取输入，并返回输入的内容。
fn input(prompt: &str) -> String {
    print!("{}", prompt);
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

    print!("Done!\n\n");
    print!(r#"
     _  __  _   _   _
    | |/ / (_) | | | |   ___   _ __
    | ' /  | | | | | |  / _ \ | '__|
    | . \  | | | | | | |  __/ | |
    |_|\_\ |_| |_| |_|  \___| |_|

    "#);   // header text
    println!("{}", tiptext());

    println!("{}", r#"
    [K] [Kill]          使用ARP协议来攻击局域网内的主机"#.red());

    println!("{}", r#"
    [S] [Scan]          扫描所有内网的活跃IP
    [N] [Net]           获取本机IP、网关和MAC地址
    [P] [Port]          扫描指定局域网IP的开放端口
    [L] [Language]      切换程序的当前语言
    [A] [About]         关于这个程序&帮助页面
    [C] [CommandList]   全部命令列表

    [E] [Exit]          退出程序
    "#.green());

    // 处理命令行参数
    if args.kill {


    } else if args.scan {
        println!("Killer Pre-view (IP Scan)");
        println!("警告：Pre-view版本暂时不开放功能，仅作为基础框架来编译");
    } else if args.net {
        println!("Killer Pre-view (Network Info)");
        println!("警告：Pre-view版本暂时不开放功能，仅作为基础框架来编译");
    } else if args.port {
        println!("Killer Pre-view (Port Scan)");
        println!("警告：Pre-view版本暂时不开放功能，仅作为基础框架来编译");
    } else if args.about {
        about();
    } else if args.version_info {
        print!("{}", r#"
     _  __  _   _   _
    | |/ / (_) | | | |   ___   _ __
    | ' /  | | | | | |  / _ \ | '__|
    | . \  | | | | | | |  __/ | |
    |_|\_\ |_| |_| |_|  \___| |_|

    *********************
    Killer 5.0"#.red());

        println!("  Pre-view version");
        println!(r#"
    *************************
    "#);
    } else if args.command_list {
        command_list();
    } else if args.help {
        help();
    } else {
        // 如果没有提供命令行参数，则进入交互模式
        loop {
            let choice = input("Killer >>> ").to_uppercase();

            match choice.as_str() {
                "K" => {
                    println!("在使用前确保此程序已授予超级管理员权限(root或administrator)!");
                    println!("配置攻击参数以发送Arp包：");
                    let user_interface = input("输入网络接口名");
                    let user_target_ip = input("输入目标IP：");
                    let user_fake_ip = input("输入网关的IP：");

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
                        println!("正在退出...");
                        std::process::exit(0);
                    }



                }
                "S" => {
                    println!("Killer Pre-view (IP Scan)");
                }
                "N" => {
                    println!("Killer Pre-view (Network Info)");
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
                    println!("感谢使用!");
                    break;
                }
                _ => {
                    println!("\n错误代码：0x04");
                    println!("非致命错误，无需异常退出。请检查输入");
                }
            }
        }
    }
}

fn windows_decision() -> bool{
    if !cfg!(windows) {
        return true
    }
    false

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
    命令列表说明：首个方括号内为别名，第二个方括号内为完整命令名称，可以在终端中作为killer的参数使用，有大小写要求。

    [S] [Scan]          扫描所有内网的活跃IP
    [N] [Net]           获取本机IP、网关和MAC地址
    [P] [Port]          扫描指定局域网IP的开放端口
    [A] [About]         关于这个程序&帮助页面
    [C] [CommandList]   全部命令列表
    [L] [Logfile]       输出日志文件

    [E] [Exit]          退出程序
    "#);
}

fn kill(attacker_ip: Ipv4Addr, attacker_mac: MacAddr, target_ip: Ipv4Addr, target_mac: MacAddr, interface_name: String, interval: u64) {
    if !windows_decision() {



    }
    else {
        //Send ARP packets on Windows
    }
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
            -h， --help 显示此帮助消息
            -v， --version 显示版本号
            -a， --about 显示关于页面
            -c， --command-list 显示命令列表
            -s， --scan 扫描本地网络中所有活动 IP
            -n， --net 获取本地 IP、网关和 MAC 地址
            -p， --port 扫描指定局域网IP的开放端口

    "#);
    }

