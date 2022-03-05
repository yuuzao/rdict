use crate::util::{ColorfulRole as Role, Style};

const VERSION: &str = "Rdict v0.0.1";
const LOGO: &str = r#"
    ⣀⢀⣀⡀   ⢀⡀ ⣀⡀ 
    ⣿⠁⢀⡿  ⣀⢼⡇ ⣭ ⣠⠖⠲⡄⢀⣴⣧ 
    ⣿⠘⢷⡀ ⣾⠁⢸⡇ ⣿ ⣿    ⢸⡇
    ⠛  ⠑ ⠙⠒⠚⠓ ⠛ ⠙⠛⠋⠁ ⠘⠋ "#;

pub fn show_logo() {
    println!("{}", LOGO.coloring(Role::Logo));

    println!(
        "{}{}",
        ' '.align_right(4),
        DictMsg::Version.to_string().coloring(Role::Content)
    );
    println!(
        "{}{}",
        ' '.align_right(4),
        DictMsg::Intro.coloring(Role::Content)
    );
}
pub fn wip() {
    show_logo();
    println!("{}{}", ' '.align_right(4), DictMsg::Wip.coloring(Role::Wip))
}

pub enum DictMsg {
    NotFound,
    Shrug,
    Wip,
    Version,
    Intro,
}

impl ToString for DictMsg {
    fn to_string(&self) -> String {
        let s: &str = self.into();
        s.to_owned()
    }
}

impl From<&DictMsg> for &str {
    fn from(msg: &DictMsg) -> &'static str {
        use DictMsg::*;
        match msg {
            NotFound => "No result found",
            Shrug => r#"¯\_(ツ)_/¯"#,
            Version => VERSION,
            Intro => "A cli dict tool implemented by Rust with love.",
            Wip => "🚧 Method Under Construction 🚧",
        }
    }
}
