use crate::utils::PrintMode;
use clap::CommandFactory;
use clap::Parser;

/// Generate a Bitcoin wallet from a text seed.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Text to use as a seed for the wallet generation
    pub seed_text: Option<String>,

    /// Print only specific value.
    #[arg(short, long, default_value = "all")]
    pub print: PrintMode,

    /// Generate Shell completion.
    #[arg(long)]
    pub completion: Option<clap_complete::Shell>,

    /// Print only raw value.
    #[arg(short, long, default_value_t = false)]
    pub raw: bool,

    /// Generate uncompressed keys (WIF will start with '5' instead of 'K' or 'L')
    #[arg(short, long, default_value_t = false)]
    pub compressed: bool,
}

impl Args {
    pub fn run(&self) {
        if let Some(shell) = self.completion {
            use std::io;

            use clap_complete::generate;

            let mut args = Args::command();
            let bin_name = std::env::current_exe();
            if let Ok(bin_name) = bin_name
                && let Some(bin_name) = bin_name.file_name()
                && let Some(bin_name) = bin_name.to_str()
            {
                generate(shell, &mut args, bin_name, &mut io::stdout());
            }
        } else if let Some(seed_text) = &self.seed_text {
            use crate::core::Wallet;
            let wallet = Wallet::new(seed_text, self.compressed);
            wallet.print(self.print.clone(), self.raw);
        } else {
            let _ = Args::command().print_help();
        }
    }
}
