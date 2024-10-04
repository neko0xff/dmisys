use tabled::Tabled;

#[derive(Tabled)]
pub struct NetworkInfo {
    pub interface: String,
    pub ipv4: String,
    pub ipv6: String,
}

#[derive(Tabled)]
pub struct NetworkSpeed {
    pub interface: String,
    pub tx_speed: String,
    pub rx_speed: String,
}

#[derive(Tabled)]
pub struct MacAddress {
    pub interface: String,
    pub mac: String,
}

#[derive(Tabled)]
pub struct DiskAll {
    pub name: String,
    pub total_space: String,
}

#[derive(Tabled)]
pub struct PowerSupplyInfo {
    pub number: String,
    pub name: String,
    //pub dev_type: String,
    pub type_: String,
    pub online: String,
}

#[derive(Tabled)]
pub struct BatteryInfo{
    pub number: String,
    pub name: String,
    //pub dev_type: String,
    pub type_: String,
    pub status: String,
    pub present: String,
    pub technology: String,
    pub capacity: usize,
    pub capacity_lv: String,
    pub model: String,
    pub manufacturer:String,
    pub serialnum: String
}

#[derive(Tabled)]
pub struct BatteryElectronMeta {
    pub number: String,
    pub name: String,
    pub volt_min: f64,
    pub volt_now: f64,
    pub charge_now: f64,
    pub charge_full_design: f64,
    pub current_now: f64,
    pub cyclecount: usize,
}

#[derive(Tabled)]
pub struct DiskInfo {
    pub name: String,
    pub filesystem: String,
    pub mount_point: String,
    pub total_space: String,
    pub used_space: String,
    pub used_percent: String,
    pub free_space: String,
}

#[derive(Tabled)]
pub struct DiskDetails {
    pub path: String,
    pub status: String,
    pub model: String,
    pub firmware: String,
    pub serial: String,
    pub factor: String,
    pub version: String,
    pub rotation_rate: String,
}