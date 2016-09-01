extern crate ruroonga;

use ruroonga as groonga;

fn main() {
    // initialize libgroonga and automatically finalize
    let libgroonga = groonga::LibGroonga::new();
    // For more safety
    let is_success = match libgroonga {
        Ok(_) => true,
        Err(e) => {
            println!("{}", e);
            false
        }
    };

    assert_eq!(true, is_success);
    let ctx = groonga::Context::new().unwrap();
    let mut db = groonga::Database::new(ctx.clone());
    let dbpath = "test.db";
    let _ = db.uses(dbpath);
    let grn_command = "table_create Users TABLE_HASH_KEY ShortText";
    let mut command = groonga::Command::new(ctx.clone());
    let _ = command.execute(grn_command.clone());
    let dump = "dump";
    let result = command.execute(dump.clone());
    println!("result: {}", result);
    println!("Hello in Ruroonga with Groonga: {}", groonga::Command::groonga_version());
}
