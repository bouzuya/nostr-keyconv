use assert_cmd::Command;

#[test]
fn npub_to_hex() -> anyhow::Result<()> {
    Command::cargo_bin("nostr-keyconv")?
        .arg("npub10elfcs4fr0l0r8af98jlmgdh9c8tcxjvz9qkw038js35mp4dma8qzvjptg")
        .assert()
        .stdout("7e7e9c42a91bfef19fa929e5fda1b72e0ebc1a4c1141673e2794234d86addf4e\n")
        .success();
    Ok(())
}

#[test]
fn nsec_to_hex() -> anyhow::Result<()> {
    Command::cargo_bin("nostr-keyconv")?
        .arg("nsec1vl029mgpspedva04g90vltkh6fvh240zqtv9k0t9af8935ke9laqsnlfe5")
        .assert()
        .stdout("67dea2ed018072d675f5415ecfaed7d2597555e202d85b3d65ea4e58d2d92ffa\n")
        .success();
    Ok(())
}

#[test]
fn note_to_hex() -> anyhow::Result<()> {
    Command::cargo_bin("nostr-keyconv")?
        .arg("note1paq6fdap00vkkxch74hxkkhjldvjtkwa6u23as2cpc92h5ghxnmqf7eyg4")
        .assert()
        .stdout("0f41a4b7a17bd96b1b17f56e6b5af2fb5925d9ddd7151ec1580e0aabd11734f6\n")
        .success();
    Ok(())
}

#[test]
fn hex_to_npub() -> anyhow::Result<()> {
    Command::cargo_bin("nostr-keyconv")?
        .arg("--prefix=npub")
        .arg("3bf0c63fcb93463407af97a5e5ee64fa883d107ef9e558472c4eb9aaaefa459d")
        .assert()
        .stdout("npub180cvv07tjdrrgpa0j7j7tmnyl2yr6yr7l8j4s3evf6u64th6gkwsyjh6w6\n")
        .success();
    Ok(())
}

#[test]
fn hex_to_nsec() -> anyhow::Result<()> {
    Command::cargo_bin("nostr-keyconv")?
        .arg("--prefix=nsec")
        .arg("67dea2ed018072d675f5415ecfaed7d2597555e202d85b3d65ea4e58d2d92ffa")
        .assert()
        .stdout("nsec1vl029mgpspedva04g90vltkh6fvh240zqtv9k0t9af8935ke9laqsnlfe5\n")
        .success();
    Ok(())
}

#[test]
fn hex_to_note() -> anyhow::Result<()> {
    Command::cargo_bin("nostr-keyconv")?
        .arg("--prefix=note")
        .arg("0f41a4b7a17bd96b1b17f56e6b5af2fb5925d9ddd7151ec1580e0aabd11734f6")
        .assert()
        .stdout("note1paq6fdap00vkkxch74hxkkhjldvjtkwa6u23as2cpc92h5ghxnmqf7eyg4\n")
        .success();
    Ok(())
}
