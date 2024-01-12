use std::env;

pub fn get_cli_arg_by_name(arg_name: &str) -> Option<String> {
    let mut args = env::args();
    while let Some(arg) = args.next() {
        if arg == arg_name {
            return args.next();
        }
    }
    None
}
