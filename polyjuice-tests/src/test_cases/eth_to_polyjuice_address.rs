//! Test EthToPolyjuiceAddress
//!   See ./evm-contracts/EthToPolyjuiceAddress.sol

use crate::helper::{
    account_id_to_eth_address, build_eth_l2_script, deploy, new_account_script, new_block_info,
    setup, PolyjuiceArgsBuilder, CKB_SUDT_ACCOUNT_ID, ETH_ACCOUNT_LOCK_CODE_HASH,
};
use gw_common::state::State;
use gw_generator::traits::StateExt;
use gw_store::chain_view::ChainView;
use gw_types::{bytes::Bytes, packed::RawL2Transaction, prelude::*};

const INIT_CODE: &str = include_str!("./evm-contracts/EthToPolyjuiceAddress.bin");

#[test]
fn test_to_polyjuice_address() {
    let (store, mut state, generator, creator_account_id) = setup();

    let from_args = [1u8; 20];
    let from_script = build_eth_l2_script(from_args.clone());
    println!("from_script: {}", hex::encode(from_script.as_slice()));
    let from_id = state.create_account_from_script(from_script).unwrap();
    state
        .mint_sudt(CKB_SUDT_ACCOUNT_ID, from_id, 200000)
        .unwrap();

    let from_balance1 = state
        .get_sudt_balance(CKB_SUDT_ACCOUNT_ID, from_id)
        .unwrap();
    println!("balance of {} = {}", from_id, from_balance1);

    let _run_result = deploy(
        &generator,
        &store,
        &mut state,
        creator_account_id,
        from_id,
        INIT_CODE,
        122000,
        0,
        0,
    );

    let contract_account_script =
        new_account_script(&mut state, creator_account_id, from_id, false);
    let new_account_id = state
        .get_account_id_by_script_hash(&contract_account_script.hash().into())
        .unwrap()
        .unwrap();
    {
        // EthToPolyjuiceAddress.calcAddr(lock_code_hash, eth_addr);
        let block_info = new_block_info(0, 2, 0);
        let input = hex::decode(format!(
            "ce99c3ef{}000000000000000000000000{}",
            hex::encode(ETH_ACCOUNT_LOCK_CODE_HASH),
            hex::encode(&from_args)
        ))
        .unwrap();
        let args = PolyjuiceArgsBuilder::default()
            .gas_limit(21000)
            .gas_price(1)
            .value(0)
            .input(&input)
            .build();
        let raw_tx = RawL2Transaction::new_builder()
            .from_id(from_id.pack())
            .to_id(new_account_id.pack())
            .args(Bytes::from(args).pack())
            .build();
        let db = store.begin_transaction();
        let tip_block_hash = store.get_tip_block_hash().unwrap();
        let run_result = generator
            .execute_transaction(
                &ChainView::new(&db, tip_block_hash),
                &state,
                &block_info,
                &raw_tx,
            )
            .expect("construct");
        state.apply_run_result(&run_result).expect("update state");
        let from_addr = account_id_to_eth_address(&state, from_id, true);
        assert_eq!(run_result.return_data, from_addr);
        // println!("result {:?}", run_result);
    }
}
