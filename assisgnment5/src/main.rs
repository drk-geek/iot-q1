mod a {
    pub fn func_1(value_1 : f32) -> f32 {
        super::b::func_2(value_1*9.0)
    }
}
mod b {
    pub fn func_2(value_2 : f32) -> f32{      
        super::c::func_3(value_2/5.0)
    }
}
mod c {
    pub fn func_3(value_3: f32) -> f32 {
        value_3 + 32.0
    }
}
mod d {
    pub fn func_4(value_4: f32) -> f32 {       
        super::e::func_5(value_4-32.0)
    }
}
mod e {
    pub fn func_5(value_5: f32) -> f32{       
        super::f::func_6(value_5 * 5.0)
    }
}
mod f {
    pub fn func_6(value_6: f32) -> f32 {
        value_6 / 9.0
    }
}
fn main(){

    let climate = crate::a::func_1(29.0);

    println!("{}", climate);

    println!("{}", crate::d::func_4(climate));
}