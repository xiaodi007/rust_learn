

pub trait ShowLight {
    fn show_light(&self);
}
pub enum Traffic {
    Red, Green, Yellow,
}
impl ShowLight for Traffic {
    fn show_light(&self) {
        match self {
            Traffic::Red =>  println!("30 sec"),
            Traffic::Green => println!("50 sec"),
            Traffic::Yellow => println!("3 sec"),
        }
    }
}
