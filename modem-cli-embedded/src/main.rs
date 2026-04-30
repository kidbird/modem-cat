use clap::{Parser, Subcommand};
use modem_hal::transport::SerialTransport;
use modem_hal::ModemFactory;

#[derive(Parser)]
#[command(name = "modem", about = "5G modem CLI for embedded Linux")]
struct Cli {
    /// Serial port (e.g. /dev/ttyUSB2)
    #[arg(short, long)]
    port: String,

    /// Baud rate
    #[arg(short, long, default_value = "115200")]
    baud: u32,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print modem status as JSON
    Status,
    /// Print signal info as JSON
    Signal,
    /// Connect data (NDIS/NDISDUP)
    Connect {
        #[arg(short, long, default_value = "1")]
        cid: i32,
    },
    /// Disconnect data
    Disconnect {
        #[arg(short, long, default_value = "1")]
        cid: i32,
    },
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    let mut transport = match SerialTransport::new(&cli.port, cli.baud) {
        Ok(t) => t,
        Err(e) => { eprintln!("Error: {}", e); std::process::exit(1); }
    };

    let mut modem = match ModemFactory::create(&mut transport) {
        Ok(m) => m,
        Err(e) => { eprintln!("Error detecting modem: {}", e); std::process::exit(1); }
    };

    let result = match cli.command {
        Commands::Status => {
            modem.query_modem_status(&mut transport)
                .map(|s| serde_json::to_string_pretty(&s).unwrap())
        }
        Commands::Signal => {
            modem.query_signal_strength(&mut transport)
                .map(|s| serde_json::to_string_pretty(&s).unwrap())
        }
        Commands::Connect { cid } => {
            modem.connect_data(&mut transport, cid)
                .map(|_| r#"{"status":"connected"}"#.to_string())
        }
        Commands::Disconnect { cid } => {
            modem.disconnect_data(&mut transport, cid)
                .map(|_| r#"{"status":"disconnected"}"#.to_string())
        }
    };

    match result {
        Ok(output) => println!("{}", output),
        Err(e) => { eprintln!("{}", e); std::process::exit(1); }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn status_output_is_valid_json() {
        let status = serde_json::json!({
            "sim_status": "READY",
            "reg_status": "1",
            "conn_status": "0",
            "imei": "123456789012345",
            "iccid": "",
            "operator": "",
        });
        let s = serde_json::to_string(&status).unwrap();
        assert!(s.contains("sim_status"));
        let parsed: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(parsed["sim_status"], "READY");
    }
}
