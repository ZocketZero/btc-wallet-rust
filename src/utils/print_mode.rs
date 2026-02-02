#[derive(Clone, Debug, clap::ValueEnum)]
pub enum PrintMode {
    A,
    All,
    S,
    Secret,
    P,
    Public,
}

impl std::str::FromStr for PrintMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "a" | "all" => Ok(PrintMode::All),
            "s" | "secret" => Ok(PrintMode::Secret),
            "p" | "public" => Ok(PrintMode::Public),
            _ => Err(format!("Invalid print mode: {}", s)),
        }
    }
}
