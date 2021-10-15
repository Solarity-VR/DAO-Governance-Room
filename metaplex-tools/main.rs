use solana_client::rpc_request::TokenAccountsFilter;

use {
    clap::{crate_description, crate_name, crate_version, App, Arg, ArgMatches, SubCommand},
    solana_clap_utils::{
        input_parsers::pubkey_of,
        input_validators::{is_url, is_valid_pubkey, is_valid_signer},
    },
    solana_client::rpc_client::RpcClient,
    solana_program::{
        account_info::AccountInfo, borsh::try_from_slice_unchecked, program_pack::Pack,
    },
    solana_sdk::{
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signer},
        system_instruction::create_account,
        transaction::Transaction,
    },
    spl_token::{
        instruction::{initialize_account, initialize_mint, mint_to},
        state::{Account, Mint},
    },
    spl_token_metadata::{
        instruction::{
            create_master_edition, create_metadata_accounts,
            mint_new_edition_from_master_edition_via_token, puff_metadata_account,
            update_metadata_accounts,
        },
        state::{
            get_reservation_list, Data, Edition, Key, MasterEditionV1, MasterEditionV2, Metadata,
            EDITION, MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, MAX_URI_LENGTH, PREFIX,
        },
    },
    std::str::FromStr,
};

const TOKEN_PROGRAM_PUBKEY: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

fn create_metadata_account_call(
    keypair_file: String,
    message: String,
    payer: Keypair,
    client: RpcClient,
) -> (Metadata, Pubkey) {
    let update_authority = read_keypair_file(keypair_file).unwrap();

    let program_key = spl_token_metadata::id();
    let token_key = Pubkey::from_str(TOKEN_PROGRAM_PUBKEY).unwrap();
    let name = message;
    let symbol = "".to_string();
    let uri = "".to_string();
    let create_new_mint = true;
    let mutable = false;
    let new_mint = Keypair::new();
    let mint_key = new_mint.pubkey();

    let metadata_seeds = &[PREFIX.as_bytes(), &program_key.as_ref(), mint_key.as_ref()];
    let (metadata_key, _) = Pubkey::find_program_address(metadata_seeds, &program_key);

    let mut new_mint_instructions = vec![
        create_account(
            &payer.pubkey(),
            &mint_key,
            client
                .get_minimum_balance_for_rent_exemption(Mint::LEN)
                .unwrap(),
            Mint::LEN as u64,
            &token_key,
        ),
        initialize_mint(
            &token_key,
            &mint_key,
            &payer.pubkey(),
            Some(&payer.pubkey()),
            0,
        )
        .unwrap(),
    ];

    let new_metadata_instruction = create_metadata_accounts(
        program_key,
        metadata_key,
        mint_key,
        payer.pubkey(),
        payer.pubkey(),
        update_authority.pubkey(),
        name,
        symbol,
        uri,
        None,
        0,
        update_authority.pubkey() != payer.pubkey(),
        mutable,
    );

    let mut instructions = vec![];

    if create_new_mint {
        instructions.append(&mut new_mint_instructions)
    }

    instructions.push(new_metadata_instruction);

    let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));
    let recent_blockhash = client.get_recent_blockhash().unwrap().0;
    let mut signers = vec![&payer];
    if create_new_mint {
        signers.push(&new_mint);
    }
    if update_authority.pubkey() != payer.pubkey() {
        signers.push(&update_authority)
    }
    transaction.sign(&signers, recent_blockhash);
    client.send_and_confirm_transaction(&transaction).unwrap();
    let account = client.get_account(&metadata_key).unwrap();
    let metadata: Metadata = try_from_slice_unchecked(&account.data).unwrap();
    (metadata, metadata_key)
}

fn main() {
    // Add this file to /metaplex/rust/token-metadata/test/src
    // replace the original file with this one 
    // spl_token_metadata is one dependency, 
    // so the rust code in the repo has to be included 

    // keypair is the file location of keypair
    // client is conected to devnet
    // message can be any text 

    // metaplex has three entries for text: name, symbol, uri;
    // for now, message is assigned to name variable
    // the goal is to have an Arweave link assigned to uri
    // and name and symbol variables for name and symbol. 

    let keypair = "/Users/lin/Desktop/solana/my-keypair.json";
    let payer = read_keypair_file(keypair).unwrap();
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let message = "type any message".to_string();

    let (metadata, metadata_key) = create_metadata_account_call(keypair.to_string(), message, payer, client);
    println!( "Create metadata account with mint {:?} and key {:?} and name of {:?} and symbol of {:?}",
        metadata.mint, metadata_key, metadata.data.name, metadata.data.symbol
    );
       
}
