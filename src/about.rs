pub const LIB_NAME: &str = "Graph1";
pub const LIB_VER: &str = "0.0.1";


pub struct AboutApp;

impl AboutApp {
    pub fn name() -> &'static str {
        LIB_NAME
    }

    pub fn ver() -> &'static str {
        LIB_VER
    }

    pub fn full_name() ->  String {
        format!("{} {}", LIB_NAME, LIB_VER)
    }

    pub fn err_prefix() -> String {
        format!(">>> [{}]-[ERR] ", AboutApp::full_name())
    }

    pub fn log_prefix() -> String {
        format!(">>> [{}]-[LOG] ", AboutApp::full_name())
    }
}