use crate::{
    core::Wallet,
    utils::{PrintMode, hash_from_file, read_hash},
};
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use std::{io, path::Path};

/// Generate a Bitcoin wallet from a text seed.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Text to use as a seed for the wallet generation
    pub seed_text: Option<String>,

    /// use file as seed
    #[arg(short, long, value_name = "File")]
    pub file: Option<String>,

    /// generate bitcoin from sha256 hash.
    #[arg(long, value_name = "Sha256")]
    pub hash: Option<String>,

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
        } else if let Some(hash) = &self.hash {
            if let Ok(hash) = read_hash(hash) {
                Wallet::from_hash(&hash, self.compressed).print(self.print.clone(), self.raw);
            } else {
                eprintln!("Invalid hash");
            }
        } else if let Some(path) = &self.file {
            if Path::new(path).exists()
                && let Ok(hash) = hash_from_file(path)
            {
                Wallet::from_hash(&hash, self.compressed).print(self.print.clone(), self.raw);
            } else {
                eprintln!("File not found");
            }
        } else {
            let _ = Args::command().print_help();
        }
    }
}
