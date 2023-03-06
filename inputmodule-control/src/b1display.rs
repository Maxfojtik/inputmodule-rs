use clap::Parser;

/// B1 Display
#[derive(Parser, Debug)]
#[command(arg_required_else_help = true)]
pub struct B1DisplaySubcommand {
    /// Set sleep status or get, if no value provided
    #[arg(long)]
    pub sleeping: Option<Option<bool>>,

    /// Jump to the bootloader
    #[arg(long)]
    pub bootloader: bool,

    /// Crash the firmware (TESTING ONLY!)
    #[arg(long)]
    pub panic: bool,

    /// Serial device, like /dev/ttyACM0 or COM0
    #[arg(long)]
    pub serial_dev: Option<String>,

    /// Get the device version
    #[arg(short, long)]
    pub version: bool,

    /// Turn display on/off
    // TODO: Allow getting current state
    #[arg(long)]
    pub display_on: Option<bool>,

    /// Invert screen on/off
    // TODO: Allow getting current state
    #[arg(long)]
    pub invert_screen: Option<bool>,
}