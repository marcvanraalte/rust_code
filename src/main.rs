#![allow(non_snake_case)]
use std::error::Error;
use csv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Point {
    x1: f32,
    x2: f32,
    kind: String,
    err: f32,
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn read_from_file(path: &str, v: &mut Vec<Point>) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    //let headers = reader.headers()?;
    //println!("{:?}", headers);
    for result in reader.deserialize() {
        let record: Point = result?;
        v.push(record);
    }
    Ok(())
}


fn main() {
    let mut v: Vec<Point> = Vec::new();

    if let Err(e) = read_from_file("../out.csv",&mut v){
        eprintln!("{}", e);
    }

    let [mut nI, mut nO, mut nB1, mut nB2, mut nB3, mut nB4, mut nBc] = [0; 7];

    for i in &v {
        if i.kind == String::from("I") { nI+=1;}
        if i.kind == String::from("O") { nO+=1;}
        if i.kind == String::from("B1") { nB1+=1;}
        if i.kind == String::from("B2") { nB2+=1;}
        if i.kind == String::from("B3") { nB3+=1;}
        if i.kind == String::from("B4") { nB4+=1;}
        if i.kind == String::from("Bc") { nBc+=1;}
    }
    println!("total number of points processed {}",nI+nO+nB1+nB2+nB3+nB4+nBc);

    let npoints =[ nI,  nO,  nB1,  nB2,  nB3,  nB4,  nBc];
    println!("{:?}",npoints);
    print_type_of(&npoints);
}
