
use std::io::{stdout, Write};// idk
use rand::Rng; // random tip text
use colored::*; // terminal coloring
//use pnet::*;     // pnet
use std::process; // exit program




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



fn main(){
    let mut stdout = stdout();
    write!(stdout, "\x1b[2J\x1b[1;1H").unwrap();  // clear screen
    print!("Done!\n\n");   // print message
    print!(r#"
     _  __  _   _   _               
    | |/ / (_) | | | |   ___   _ __ 
    | ' /  | | | | | |  / _ \ | '__|
    | . \  | | | | | | |  __/ | |   
    |_|\_\ |_| |_| |_|  \___| |_|   
                                    
    "#);   // header text
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

    println!("{}", tiptext);

    print!("{}", r#"
    
    [K] 使用Arp攻击一个内网的计算机
    "#.red());

    println!("{}", r#"
    [S] 扫描所有内网的活跃IP
    [N] 获取本机IP、网关和MAC地址
    [P] 扫描指定局域网IP的开放端口
    [A] 关于这个程序&帮助页面
    [T] 此程序的设置选项
    [E] 退出程序
    "#.green());

    let loop_conlrol = true;
    let user_input = String::new();

    //while loop
    while loop_conlrol == true {
        if user_input == "K" {
            
            
        } else if user_input == "S" {

        } else if user_input == "N" {
            
            println!(r#"
            ┏━━━━━━━━━━━━━━━━━━━━━━━━━━┓
            ┃       NetWork Info       ┃
            ┣━━━━━━━━━━━━━━━━━━━━━━━━━━┫
            ┃ 网络状态 ：[]┃ 位置 ：[]   ┃
            ┣━━━━━━━━━━━━━━━━━━━━━━━━━━┫
            ┃ 网关地址 []
            ┃ 本机地址 []
            ┃ 物理地址 []
            ┣──────────────────────────┫
            ┃ ：物理地址是MAC地址        ┃
            ┃ ：只是因为MAC地址不全是半角 ┃
            ┃ ：字符, 所以会无法垂直对齐  ┃
            ┗━━━━━━━━━━━━━━━━━━━━━━━━━━┛
            

            "#)
        } else if user_input == "P" {
            
        } else if user_input == "A" {
            println!(r#"
            关于程序：
                                Killer v5.0 [RustVersion]
                                在这个版本，Killer使用Rust重写，旨在高性能和跨平台。
                                **********************
            作者信息：
                                2024年，由xiaozhao45编写而成。
                                开源于Github，完全自由使用。
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
                                0x0x：未知错误，请携带日志提交issues
            其他信息：
                                1.在Killer的安装目录下有killer.log日志文件。
                                2.同样在安装目录下的config目录下有Killer的配置文件。
            
            "#);


        } else if user_input == "T" {
            
        } else if user_input == "E" {
            println!( "正在退出...");
            process::exit(0); 
        } else {
            println!("无效的输入，请重新输入。");
        }
    } 
}


// Anybody Here？



//*****************************//
//功能函数 - Kill;Scan;Net;Port //
//*****************************//
//Last Update: 2024/07/12
//***********//

