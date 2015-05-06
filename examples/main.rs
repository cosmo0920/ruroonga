extern crate ruroonga;
use ruroonga::commandapi::*;

fn main() {
    let ctx = groonga_init();
    let dbpath = "test.db";
    let _ = groonga_db_use(ctx, dbpath);
    let command = "table_create Users TABLE_HASH_KEY ShortText";
    let _ = groonga_execute_command(ctx, command);
    println!("Hello in Ruroonga with Groonga: {}", get_groonga_version());
    let _ = groonga_fin(ctx);
}
