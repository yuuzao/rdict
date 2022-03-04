use crate::util::{coloring, ColorfulRole};

const VERSION: &str = "Rdict v0.0.1";
const LOGO: &str = r#"
    ⣀⢀⣀⡀   ⢀⡀ ⣀⡀ 
    ⣿⠁⢀⡿  ⣀⢼⡇ ⣭ ⣠⠖⠲⡄⢀⣴⣧ 
    ⣿⠘⢷⡀ ⣾⠁⢸⡇ ⣿ ⣿    ⢸⡇
    ⠛  ⠑ ⠙⠒⠚⠓ ⠛ ⠙⠛⠋⠁ ⠘⠋ "#;

pub fn show_logo() {
    println!("{}", coloring(LOGO, ColorfulRole::Logo));

    println!(
        "{:>4}{}",
        ' ',
        coloring(DictMsg::Version, ColorfulRole::Content)
    );
    println!(
        "{:>4}{}",
        ' ',
        coloring(DictMsg::Intro, ColorfulRole::Content)
    )
}
pub fn wip() {
    show_logo();
    println!("{:>4}{}", ' ', coloring(DictMsg::Wip, ColorfulRole::Other));
}

pub enum DictMsg {
    NotFound,
    Shrug,
    Wip,
    Version,
    Intro,
}

impl From<DictMsg> for &str {
    fn from(msg: DictMsg) -> &'static str {
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
