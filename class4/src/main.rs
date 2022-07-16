use q1::ShowLight;
use q2::sum;

mod q1;
mod q2;
mod q3;
fn main() {
    q1::Traffic::Red.show_light();
    q1::Traffic::Yellow.show_light();
    q1::Traffic::Green.show_light();

    println!("{:?}",sum(&[1,2,3]));
    println!("{:?}",sum(&[]));

    let  grph1 = q3::triangle{a:1, h:3};
    let  grph2 = q3::rectangle{a:1, b:3};

    println!("{:?}",grph1.calculate_graph());
    println!("{:?}",grph2.calculate_graph());
}
