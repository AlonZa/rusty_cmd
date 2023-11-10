use rusty_cmd;

#[cfg(test)]
mod tests {
    #[test]
    fn test_change_prompt() {
        let mut cmd: rusty_cmd::Cmdline = rusty_cmd::Cmdline::new();
        cmd.change_prompt("[My New Prompt] # ");
        assert_eq!(cmd.get_prompt().eq("[My New Prompt] # "), true);
    }

    #[test]
    fn test_add_command() {
        let mut cmd: rusty_cmd::Cmdline = rusty_cmd::Cmdline::new();
        cmd.change_prompt("[My New Prompt] # ");
        assert_eq!(cmd.get_prompt().eq("[My New Prompt] # "), true);
    }
}