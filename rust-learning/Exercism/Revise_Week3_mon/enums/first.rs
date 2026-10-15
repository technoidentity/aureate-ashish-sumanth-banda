enum TrafficLight{
    Red,
    Yellow,
    Green,
}
impl TrafficLight{
    fn can_drive(&self)->bool{
        match self{
            TrafficLight :: Green => true,
            TrafficLight :: Yellow => false,
            TrafficLight :: Red => false,
        }
    }
}
fn main(){
    let red_light = TrafficLight::Red;
    match red_light{
        TrafficLight :: Red => println!("Stop"),
        TrafficLight :: Yellow => println!("Wait"),
        TrafficLight :: Green => println!("Go"),
    }
    let r = red_light.can_drive();
    println!("Vahicle can move ?{}",r);

}