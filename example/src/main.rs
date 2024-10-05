mod ds;
mod info;

fn main() {
    info::info_system::output_msg();
    info::info_network::output_msg();
    info::info_cpu::output_msg();
    info::info_gpu::output_msg();
    info::info_memory::output_msg();
    info::info_power::output_msg();
    info::info_disk::output_msg();
}