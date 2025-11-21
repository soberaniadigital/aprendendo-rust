use integrate;
fn main(){
    let yan=integrate::prelude::rectangle_rule(|x| (9.8*65.0/12.5)*(1.0-(-12.5/65.0*x as f64).exp()),0.0,10.0,1000 as usize);
    let celso=integrate::prelude::newton_rule(|x| (9.8*85.0/12.5)*(1.0-(-12.5/65.0*x as f64).exp()),0.0,10.0,1000 as usize);
    println!("celso voou {}m em 10seg",celso);
    println!("yan voou {}m em 10seg",yan);
}