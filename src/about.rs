pub const APP_NAME: &str = "Graph1";
pub const APP_VER: &str = "0.0.1";


pub struct AboutApp;

impl AboutApp {
    pub fn name() -> &'static str {
        APP_NAME
    }

    pub fn ver() -> &'static str {
        APP_VER
    }

    pub fn full_name() ->  String {
        format!("{} {}",APP_NAME, APP_VER)
    }

    pub fn err_prefix() -> String {
        format!(">>> [{}]-[ERR] ", AboutApp::full_name())
    }

    pub fn log_prefix() -> String {
        format!(">>> [{}]-[LOG] ", AboutApp::full_name())
    }
}