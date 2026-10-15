struct Triangle{
    side1:u64,
    side2:u64,
    side3:u64,

}

impl Triangle{

    fn build(sides:[u64;3])->Option<Triangle>{
    
    if sides[0] > 0 && sides[1] > 0 && sides[2] >0 && sides[0]+sides[1] >= sides[2] && sides[0]+sides[2]>= sides[1] && sides[2]+sides[1] >= sides[0]{
        let tringle = Triangle{
            side1: sides[0],
            side2: sides[1],
            side3: sides[2],
        };
        Some(tringle)
    }
    else{
        None
    }

    }
    fn is_equilateral(&self)->bool{

        if self.side1 == self.side2 && self.side2 == self.side3 && self.side1 == self.side3{
            true
        }
        else{
            false
        }

    }
    fn is_isosceles(&self)->bool{
        if self.side1 == self.side2 || self.side2 == self.side3 || self.side1 == self.side3{
            true
        }
        else{
            false
        }

    }
    fn is_scalene(&self)->bool{
        if self.side1 != self.side2 && self.side2 != self.side3 && self.side1 != self.side3{
            true
        }
        else{
            false
        }

    }
}
fn main(){
    let tringle = Triangle::build([1,1,3]);

    match tringle{
        Some(tringle)=>{
            let a = tringle.is_scalene();
            println!("its a scalene:{a}");
            let b = tringle.is_equilateral();
            println!("its an equilateral:{b}");
            let c = tringle.is_isosceles();
            println!("its an isosceles{c}");

        }
        None => {
            println!("Error: The provided sides do not make a valid triangle.");
        }

    }
    

    
}