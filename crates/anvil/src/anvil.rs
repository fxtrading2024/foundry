//! The `anvil` cli
use anvil::cmd::NodeArgs;
use clap::{CommandFactory, Parser, Subcommand};

/// A fast local Ethereum development node.
#[derive(Parser)]
#[clap(name = "anvil", version = anvil::VERSION_MESSAGE, next_display_order = None)]
pub struct Anvil {
    #[clap(flatten)]
    pub node: NodeArgs,

    #[clap(subcommand)]
    pub cmd: Option<AnvilSubcommand>,
}

#[derive(Subcommand)]
pub enum AnvilSubcommand {
    /// Generate shell completions script.
    #[clap(visible_alias = "com")]
    Completions {
        #[clap(value_enum)]
        shell: clap_complete::Shell,
    },

    /// Generate Fig autocompletion spec.
    #[clap(visible_alias = "fig")]
    GenerateFigSpec,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = Anvil::parse();
    app.node.evm_opts.resolve_rpc_alias();

    if let Some(ref cmd) = app.cmd {
        match cmd {
            AnvilSubcommand::Completions { shell } => {
                clap_complete::generate(
                    *shell,
                    &mut Anvil::command(),
                    "anvil",
                    &mut std::io::stdout(),
                );
            }
            AnvilSubcommand::GenerateFigSpec => clap_complete::generate(
                clap_complete_fig::Fig,
                &mut Anvil::command(),
                "anvil",
                &mut std::io::stdout(),
            ),
        }
        return Ok(())
    }

    let _ = fdlimit::raise_fd_limit();
    app.node.run().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        Anvil::command().debug_assert();
    }

    #[test]
    fn can_parse_help() {
        let _: Anvil = Anvil::parse_from(["anvil", "--help"]);
    }

    #[test]
    fn can_parse_completions() {
        let args: Anvil = Anvil::parse_from(["anvil", "completions", "bash"]);
        assert!(matches!(
            args.cmd,
            Some(AnvilSubcommand::Completions { shell: clap_complete::Shell::Bash })
        ));
    }
}
