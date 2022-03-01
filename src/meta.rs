use crate::util::colorize;

pub struct Meta;

impl Meta {
    pub fn show_logo() {
        let logo = "
    ⣀⢀⣀⡀   ⢀⡀ ⣀⡀ 
    ⣿⠁⢀⡿  ⣀⢼⡇ ⣭ ⣠⠖⠲⡄⢀⣴⣧ 
    ⣿⠘⢷⡀ ⣾⠁⢸⡇ ⣿ ⣿    ⢸⡇
    ⠛  ⠑ ⠙⠒⠚⠓ ⠛ ⠙⠛⠋⠁ ⠘⠋ ";
        println!("{}", colorize(logo, (0, 221, 192)));

        println!("{:>4}{}", ' ', colorize("Rdict v0.0.1", (92, 184, 92)));
        println!(
            "{:>4}{}",
            ' ',
            colorize(
                "A cli dict tool implemented by Rust with love.",
                (92, 184, 92)
            )
        )
    }
}
