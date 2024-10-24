use parser_hiskyi::list_parser;

pub fn main() {
    assert_eq!(list_parser::list("[5,3,5,0,51,763]"), Ok(vec![5, 3, 5, 0, 51, 763]));
    println!("{:?}",list_parser::list("[5,3,5,0,51,763]") );
}