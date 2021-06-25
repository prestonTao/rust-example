//配置状态分离

pub struct Struct {
    config: StructConfig,
}

impl Struct {
    pub fn new() -> Self {
        Self::new_with_config(Default::default())
    }
    
    pub fn new_with_config(config: StructConfig) -> Self {
        Self { config }
    }
}

#[derive(Default)]
pub struct StructConfig {
    pub a: bool,
    pub b: u32,
    pub c: String,
}

fn main() {
    let _ = Struct::new();
    
    let _ = Struct::new_with_config(StructConfig {
        a: true,
        ..Default::default()
    });
}