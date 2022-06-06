// ga - genetic algorithms

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]
pub enum SiteState {
    Empty,
    Can,
    Wall,
}
#[allow(unused_imports)]
use SiteState::{Empty, Can, Wall};


type Individual = Vec<Vec<SiteState>> = Vec::new();




fn main() {
    let mut matrix: Vec<Vec<SiteState>> = Vec::new();

    // Wall Wall Wall Wall
    // Wall Empt Can  Wall
    // Wall Can  Empt
    let mut row: Vec<SiteState> = Vec::new();
    row.push(Wall);
    row.push(Wall);
    row.push(Wall);
    row.push(Wall);
    matrix.push(row);

    let mut row: Vec<SiteState> = Vec::new();
    row.push(Wall);
    row.push(Empty);
    row.push(Can);
    row.push(Wall);
    matrix.push(row);

    let mut row: Vec<SiteState> = Vec::new();
    row.push(Wall);
    row.push(Can);
    row.push(Empty);
    row.push(Wall);
    matrix.push(row);

    let mut row: Vec<SiteState> = Vec::new();
    row.push(Wall);
    row.push(Wall);
    row.push(Wall);
    row.push(Wall);
    matrix.push(row);





    println!("matrix = {:?}", matrix);
}
