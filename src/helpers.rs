#[macro_export]
macro_rules! print_noline {
    ( $( $x:tt )* ) => {{
        use std::io::Write;
        print!("{}", format_args!($($x)*));
        match std::io::stdout().flush() {
            Ok(_) => (),
            Err(err) => panic!("{}", err)
        }
    }}
}
