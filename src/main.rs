use parse::parse_body_complete;

mod ast;
mod parse;
mod serialize;

fn main() {
    match parse_body_complete(
        "
(hardcoded_command argument argument)
(hardcoded_command argument)",
    ) {
        Ok(body) => {
            println!("{:?}", body);
            println!("--------");
            println!("{}", body.serialize());
        }
        Err(e) => eprintln!("{:?}", e),
    }
}
