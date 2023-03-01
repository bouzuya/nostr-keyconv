use std::str::FromStr;

use anyhow::bail;
use clap::Parser;
use nostr::{
    prelude::{hex::ToHex, FromBech32, ToBech32},
    secp256k1::{SecretKey, XOnlyPublicKey},
    EventId,
};

#[derive(clap::Parser)]
struct Args {
    /// private key or public key or event id
    public_key_or_private_key_or_event_id: String,
    ///
    #[arg(long, value_enum)]
    prefix: Option<Prefix>,
}

#[derive(Clone, clap::ValueEnum)]
enum Prefix {
    Npub,
    Nsec,
    Note,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let s = args.public_key_or_private_key_or_event_id.as_str();
    if let Ok(public_key) = XOnlyPublicKey::from_bech32(s) {
        println!("{}", public_key.to_hex());
        return Ok(());
    }

    if let Ok(private_key) = SecretKey::from_bech32(s) {
        println!("{}", private_key.display_secret());
        return Ok(());
    }

    if let Ok(event_id) = EventId::from_bech32(s) {
        println!("{}", event_id.to_hex());
        return Ok(());
    }

    let bech32 = match args.prefix {
        Some(prefix) => match prefix {
            Prefix::Npub => XOnlyPublicKey::from_str(s)?.to_bech32()?,
            Prefix::Nsec => SecretKey::from_str(s)?.to_bech32()?,
            Prefix::Note => EventId::from_hex(s)?.to_bech32()?,
        },
        None => bail!("prefix is not specified"),
    };
    println!("{bech32}");

    Ok(())
}
