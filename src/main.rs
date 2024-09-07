mod userconfig;

fn main() {
    let userconfig = userconfig::UserConfig::new();
    println!("{:?}", userconfig);
}
