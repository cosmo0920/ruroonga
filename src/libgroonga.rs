extern crate libc;
extern crate groonga_sys as ffi;
use std::mem;
use ffi::groonga;
use super::commandapi;

#[derive(Clone, Debug)]
pub struct LibGroonga {
    disposed: bool,
}

impl LibGroonga {
    pub fn new() -> Result<LibGroonga, String> {
        let rc = commandapi::groonga_init();
        if rc != groonga::grn_rc::GRN_SUCCESS {
            return Err("Couldn't initilize Groonga.".to_string());
        }
        Ok(LibGroonga { disposed: false })
    }

    fn close(&mut self) -> Result<(), String> {
        if self.disposed {
            return Ok(());
        }
        let rc = commandapi::groonga_fin();
        if rc != groonga::grn_rc::GRN_SUCCESS {
            return Err("Couldn't finalized Groonga.".to_string());
        }
        self.disposed = true;
        Ok(())
    }
}

impl Drop for LibGroonga {
    fn drop(&mut self) {
        self.close().unwrap();
    }
}

#[derive(Clone, Debug)]
pub struct Context {
    ctx: *mut groonga::grn_ctx,
    disposed: bool,
}

impl Context {
    pub fn new() -> Result<Context, String> {
        let ctx = commandapi::groonga_ctx_open(0);
        if ctx.is_null() {
            return Err("Couldn't create Groonga Context.".to_string());
        }
        Ok(Context {
            ctx: ctx,
            disposed: false,
        })
    }

    pub fn close(&mut self) -> Result<(), String> {
        if self.disposed {
            return Ok(());
        }
        let rc = commandapi::groonga_ctx_close(self.ctx);
        if rc != groonga::grn_rc::GRN_SUCCESS {
            return Err("Couldn't dispose Groonga Context.".to_string());
        }
        unsafe {
            self.ctx = mem::zeroed();
        }
        self.disposed = true;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Database {
    ctx: *mut groonga::grn_ctx,
    disposed: bool,
}

impl Database {
    pub fn new(ctx: Context) -> Database {
        Database {
            ctx: ctx.ctx,
            disposed: false,
        }
    }

    pub fn uses(&mut self, dbpath: &str) {
        let _ = commandapi::groonga_db_use(self.ctx, dbpath);
    }

    fn close(&mut self) {
        if self.disposed {
            return;
        }
        unsafe {
            self.ctx = mem::zeroed();
        }
        self.disposed = true;
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        self.close()
    }
}

#[derive(Clone, Debug)]
pub struct Command {
    ctx: *mut groonga::grn_ctx,
    disposed: bool,
}

impl Command {
    pub fn new(ctx: Context) -> Command {
        Command {
            ctx: ctx.ctx,
            disposed: false,
        }
    }

    pub fn groonga_version<'a>() -> &'a str {
        commandapi::get_groonga_version()
    }

    pub fn execute(&mut self, command: &str) -> String {
        let result = commandapi::groonga_execute_command(self.ctx, command);
        match result {
            Ok(v) => v,
            Err(_) => "".to_string(),
        }
    }

    fn close(&mut self) {
        if self.disposed {
            return;
        }
        unsafe {
            self.ctx = mem::zeroed();
        }
        self.disposed = true;
    }
}

impl Drop for Command {
    fn drop(&mut self) {
        self.close()
    }
}

#[cfg(test)]
mod test {
    extern crate tempdir;
    use super::*;
    use self::tempdir::TempDir;

    fn setup_dbpath() -> String {
        let dbdir = TempDir::new("ruroonga");
        let dbname = "test.db";
        let mut dbpath_buf = dbdir.unwrap().into_path();
        dbpath_buf.push(dbname);
        let dbpath = match dbpath_buf.to_str() {
            Some(v) => v,
            None => panic!("Could not get temporary database path."),
        };

        dbpath.to_owned()
    }

    #[test]
    fn integration_test() {
        let groonga = LibGroonga::new();
        assert!(groonga.is_ok());

        let context = Context::new();
        assert!(context.is_ok());

        let ctx = match context {
            Ok(v) => v,
            Err(_) => panic!("Could not get context."),
        };

        let dbpath = setup_dbpath();
        let _ = Database::new(ctx.clone()).uses(&*dbpath);

        let grn_command = "table_create Users TABLE_HASH_KEY ShortText";
        let mut command = Command::new(ctx.clone());
        let table_create_result = command.execute(grn_command.clone());
        assert_eq!("true", table_create_result);

        let dump = "dump";
        let result = command.execute(dump.clone());
        let dumped_command = "table_create Users TABLE_HASH_KEY ShortText\n";
        assert!(dumped_command == result || format!("{}\u{fffd}\u{7f}", dumped_command) == result);
    }
}
