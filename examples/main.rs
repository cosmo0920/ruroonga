extern crate ruroonga;

use ruroonga as groonga;

fn main() {
    // initialize libgroonga and automatically finalize
    groonga::LibGroonga::new().unwrap();

    let ctx = groonga::Context::new().unwrap();
    let mut db = groonga::Database::new(ctx.clone());
    let dbpath = "test.db";
    let _ = db.uses(dbpath);
    let grn_command = "table_create Users TABLE_HASH_KEY ShortText";
    let mut command = groonga::Command::new(ctx.clone());
    let _ = command.execute(grn_command);
    println!("Hello in Ruroonga with Groonga: {}", groonga::Command::groonga_version());
}
