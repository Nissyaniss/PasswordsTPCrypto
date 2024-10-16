use std::fs::File;

pub fn main() {
    unimplemented!();
}

pub fn change_master_password(is_first_password: bool) {
    if cfg!(windows) {
        /*if is_first_password {
            File::create("%userprofile%/master_password.txt").unwrap();
        }*/
    } else if cfg!(unix) {
        File::create("~/tmp/masterPassword.txt").unwrap();
    }
}
