mod ds;
mod info;

use ds::*;
use info::*;
use clap::Parser;

fn main() {
    let cli = ds_cli::Cli::parse();
    let arg_output = cli.output.as_deref();

    // 指定參數的輸出內容
    if  arg_output == Some("system") {
        info_system::output_msg();
    }else if arg_output == Some("network"){
        info_network::output_msg();
    }else if arg_output == Some("cpu"){
        info_cpu::output_msg();
    }else if arg_output == Some("gpu"){
       info_gpu::output_msg();
    }else if arg_output == Some("memory"){
        info_memory::output_msg();
    }else if arg_output == Some("power"){
        info_power::output_msg();
    }else if arg_output == Some("disk"){
        info_disk::output_msg();
    }else if arg_output == Some("environment"){
        info_env::output_msg();
    }else if arg_output == Some("desktop"){
        info_desktop::output_msg();
    }

    // 如果未指定輸出或輸出`all`,則輸出
    if arg_output == None || arg_output == Some("all"){
        info_system::output_msg();
        info_desktop::output_msg();
        info_env::output_msg();
        info_network::output_msg();
        info_cpu::output_msg();
        info_gpu::output_msg();
        info_memory::output_msg();
        info_power::output_msg();
        info_disk::output_msg();
    }
    
}