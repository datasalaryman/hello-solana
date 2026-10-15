use std::{path::PathBuf, str::FromStr};

use litesvm::LiteSVM;
use solana_address::Address;
use solana_instruction::{AccountMeta, Instruction};
use solana_keypair::Keypair;
use solana_message::Message;
use solana_signer::Signer;
use solana_transaction::Transaction;

const PROGRAM_ID: &str = "GwSgNvA6PjFt32Bb2dZ5XS94zRPoXn3aTPnwMs1jfjbk";
// Anchor's first eight bytes of sha256("global:hello").
const HELLO_DISCRIMINATOR: [u8; 8] = [149, 118, 59, 220, 196, 127, 161, 179];

fn setup() -> (LiteSVM, Address, Keypair) {
    let mut svm = LiteSVM::new();
    let program_id = Address::from_str(PROGRAM_ID).unwrap();
    let program_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/deploy/hello_solana.so");

    assert!(
        program_path.exists(),
        "program binary not found; run `NO_DNA=1 anchor build` first"
    );
    svm.add_program_from_file(program_id, program_path).unwrap();

    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    (svm, program_id, payer)
}

fn hello_data(payload: &[u8]) -> Vec<u8> {
    let mut data = HELLO_DISCRIMINATOR.to_vec();
    data.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    data.extend_from_slice(payload);
    data
}

fn transaction(
    svm: &LiteSVM,
    program_id: Address,
    payer: &Keypair,
    accounts: Vec<AccountMeta>,
    payload: &[u8],
) -> Transaction {
    let instruction = Instruction {
        program_id,
        accounts,
        data: hello_data(payload),
    };
    let message = Message::new(&[instruction], Some(&payer.pubkey()));

    Transaction::new(&[payer], message, svm.latest_blockhash())
}

#[test]
fn fails_when_an_account_is_passed() {
    let (mut svm, program_id, payer) = setup();
    let accounts = vec![AccountMeta::new_readonly(payer.pubkey(), true)];
    let tx = transaction(&svm, program_id, &payer, accounts, &[]);

    let failure = svm.send_transaction(tx).unwrap_err();
    assert!(failure
        .meta
        .logs
        .iter()
        .any(|log| log.contains("AccountsNotAllowed")));
}

#[test]
fn fails_when_data_is_passed() {
    let (mut svm, program_id, payer) = setup();
    let tx = transaction(&svm, program_id, &payer, vec![], b"unexpected");

    let failure = svm.send_transaction(tx).unwrap_err();
    assert!(failure
        .meta
        .logs
        .iter()
        .any(|log| log.contains("DataNotAllowed")));
}

#[test]
fn succeeds_and_logs_hello_solana() {
    let (mut svm, program_id, payer) = setup();
    let tx = transaction(&svm, program_id, &payer, vec![], &[]);

    let metadata = svm.send_transaction(tx).unwrap();
    assert!(
        metadata
            .logs
            .iter()
            .any(|log| log == "Program log: Hello Solana!"),
        "expected greeting in logs, got: {:?}",
        metadata.logs
    );
}
