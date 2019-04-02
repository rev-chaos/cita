#[macro_use]
extern crate log;
extern crate env_logger;
mod helper;
mod json;

use core_executor::engines::NullEngine;
use core_executor::libexecutor::economical_model::EconomicalModel;
use core_executor::libexecutor::sys_config::BlockSysConfig;
use core_executor::state::ApplyResult;
use core_executor::types::transaction::Transaction;
use evm::cita_types::{H256, U256};
use evm::env_info::EnvInfo;
use helper::{get_temp_state, secret_2_address, string_2_bytes, string_2_h256, string_2_u256};
use hex;
use libproto::blockchain::Transaction as ProtoTransaction;
use std::fs;
use std::ops::Add;
use std::sync::Arc;

fn test_json_file(p: &str) {
    // println!("test_json_file(r\"{}\");", p);
    let f = fs::File::open(p).unwrap();
    let tests = json::Test::load(f).unwrap();
    for (name, test) in tests.into_iter() {
        let data_post_byzantium = test.post.unwrap().constantinople;
        if data_post_byzantium.is_none() {
            continue;
        }

        for (_i, postdata) in data_post_byzantium.unwrap().into_iter().enumerate() {
            // Init state
            let mut state = get_temp_state();
            for (address, account) in test.pre.clone().unwrap() {
                let balance = string_2_u256(account.balance);
                let code = string_2_bytes(account.code);
                let nonce = string_2_u256(account.nonce);
                if code.is_empty() {
                    state.new_contract(&address, balance, nonce);
                } else {
                    state.new_contract(&address, balance, nonce);
                    let _ = state.init_code(&address, code);
                }

                for (k, v) in account.storage {
                    let kk = string_2_h256(k);
                    let vv = string_2_h256(v);
                    let _ = state.set_storage(&address, kk, vv);
                }
            }
            state.commit().unwrap();

            // Set envionment
            let mut env_info = EnvInfo::default();
            env_info.difficulty = string_2_u256(test.env.current_difficulty.clone());
            env_info.number = string_2_u256(test.env.current_number.clone()).low_u64();
            env_info.timestamp = string_2_u256(test.env.current_timestamp.clone()).low_u64();
            env_info.gas_limit = string_2_u256(test.env.current_gas_limit.clone());
            env_info.author = test.env.current_coinbase;
            let previous_hash = string_2_h256(test.env.previous_hash.clone());
            Arc::make_mut(&mut env_info.last_hashes).push(previous_hash);

            let engine = NullEngine::cita();
            let mut config = BlockSysConfig::default();
            config.quota_price = string_2_u256(test.transaction.gas_price.clone());
            config.economical_model = EconomicalModel::Charge;
            config.quota_price = U256::from(1);

            let idx_gas = &postdata.indexes[&String::from("gas")];
            let idx_value = &postdata.indexes[&String::from("value")];
            let idx_data = &postdata.indexes[&String::from("data")];
            // debug!("index gas={}, index value={}, index data{}", idx_gas, idx_value, idx_data);
            let str_gas = test.transaction.gas_limit.clone()[*idx_gas].clone();
            let str_value = test.transaction.value.clone()[*idx_value].clone();
            let str_data = test.transaction.data.clone()[*idx_data].clone();

            let mut proto_tx = ProtoTransaction::new();
            proto_tx.set_data(string_2_bytes(str_data));
            proto_tx.set_value(string_2_bytes(str_value));
            proto_tx.set_nonce(test.transaction.nonce.clone());
            proto_tx.set_quota(string_2_u256(str_gas).low_u64());
            if !test.transaction.to.is_empty() {
                proto_tx.set_to(test.transaction.to.clone());
            }

            let tx = Transaction::create(&proto_tx).unwrap();
            let sender = secret_2_address(&test.transaction.secret_key);
            let signed_transaction = tx.fake_sign(sender);

            // Execute transactions
            let result: ApplyResult = state.apply(&env_info, &engine, &signed_transaction, true, &config);
            match result {
                Ok(outcome) => {
                    debug!("lalalal receipt error: {:?}", outcome.receipt.error);
                }
                _ => panic!("apply_transaction: There must be something wrong!"),
            }

            // check root hash
            state.commit().unwrap();
            let root = state.root();
            assert_eq!(*root, string_2_h256(postdata.hash));
            // if *root != string_2_h256(postdata.hash) {
            //     println!("test_json_file(r\"{}\");", p);
            // } else {
            //     // println!("skip_json_file(r\"{}\");", p);
            // }
        }
    }
}

pub fn test_json_path(p: &str) {
    // println!("==== > run tests in {}", p);
    let info = fs::metadata(p).unwrap();
    if info.is_dir() {
        // println!("\nprintln!(\" ========> \" run {})", p);
        println!("* [ ] {}", p);
    }
    if info.is_dir() {
        for entry in fs::read_dir(p).unwrap() {
            let entry = entry.unwrap();
            let p = entry.path();
            test_json_path(p.to_str().unwrap());
        }
    } else {
        test_json_file(p);
    }
}

pub fn skip_path(_name: &str) {}

pub fn skip_json_file(_name: &str) {}

fn main() {
    env_logger::init();
    // passed:
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stMemoryTest");
    // test_json_path("./tests/jsondata/GeneralStateTests/stRefundTest");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stShift");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stHomesteadSpecific");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stExample");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stExtCodeHash");   // 因为我们没有 EXTCODEHASH 指令
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stLogTests");      // sstore
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stCodeCopyTest");  // sstore

    // 3
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stSystemOperationsTest");             // 有问题, 要 skip
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stRecursiveCreate");                  // 有问题
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stZeroKnowledge");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stRandom2");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stTransitionTest");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stZeroCallsTest");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stBugs");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stNonZeroCallsTest");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stCallDelegateCodesHomestead");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stAttackTest");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stStackTests");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stSolidityTest");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stQuadraticComplexityTest");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stPreCompiledContracts2");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stInitCodeTest");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stSpecialTest");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stCodeSizeLimit");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stReturnDataTest");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stTransactionTest");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stRevertTest");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stMemoryStressTest");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stCallDelegateCodesCallCodeHomestead");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stMemExpandingEIP150Calls");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stRandom");                           // 有问题 要 skip
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stCallCodes");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stBadOpcode");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stCreateTest");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stWalletTest");
    // test_json_path(r"./tests/jsondata/GeneralStateTests/stDelegatecallTestHomestead");

    // test_json_file(r"./tests/jsondata/GeneralStateTests/stLogTests/log4_logMemsizeTooHigh.json");        // sstore
    // test_json_file(r"./tests/jsondata/GeneralStateTests/stLogTests/logInOOG_Call.json");
    // test_json_file(r"./tests/jsondata/GeneralStateTests/stLogTests/log0_logMemStartTooHigh.json");

    // test_json_file(r"./tests/jsondata/GeneralStateTests/stCodeCopyTest/ExtCodeCopyTargetRangeLongerThanCodeTests.json");
    // test_json_file(r"./tests/jsondata/GeneralStateTests/stCodeCopyTest/ExtCodeCopyTests.json");

    // test_json_file(r"./tests/jsondata/GeneralStateTests/stDelegatecallTestHomestead/callcodeOutput3Fail.json");
    // test_json_file(r"./tests/jsondata/GeneralStateTests/stDelegatecallTestHomestead/callOutput3Fail.json");
    // test_json_file(r"./tests/jsondata/GeneralStateTests/stDelegatecallTestHomestead/callOutput3Fail.json");
    // test_json_file(r"./tests/jsondata/GeneralStateTests/stDelegatecallTestHomestead/callcodeOutput3partial.json");
    // test_json_file(r"./tests/jsondata/GeneralStateTests/stDelegatecallTestHomestead/callcodeOutput2.json");
    // test_json_file(r"./tests/jsondata/GeneralStateTests/stDelegatecallTestHomestead/callcodeOutput1.json");
    // test_json_file(r"./tests/jsondata/GeneralStateTests/stDelegatecallTestHomestead/Call1024OOG.json");
    // test_json_file(r"./tests/jsondata/GeneralStateTests/stDelegatecallTestHomestead/CallLoseGasOOG.json");

    test_json_path(r"./tests/jsondata/GeneralStateTests/stRecursiveCreate/recursiveCreateReturnValue.json");
    // test_json_file(r"./tests/jsondata/GeneralStateTests/stSystemOperationsTest/CallToNameRegistratorNotMuchMemory0.json");
}
