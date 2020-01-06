#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{string::String as AllocString, vec::Vec};

use simple_json::{self, json::JsonObject, json::JsonValue};

/*
fn main() {
    struct HTLC {
        address: Vec<u8>,
        topics: Vec<Vec<u8>>,
        data: Vec<u8>,
        block_number: Vec<u8>,
        time_stamp: Vec<u8>,
        transaction_hash: Vec<u8>,
        transaction_index: Vec<u8>,
    }
    let json_str = r#"{"status":"1","message":"OK","result":[{"address":"0x16d5195fe8c6ba98b2f61a9a787bc0bde19e3f6f","topics":["0x924028c31cbef81354a146f585e1c91ea6a9caa2a9880e0e2f195cb8894823aa","0x000000000000000000000000f7fea1722f9b27b0666919a5664bab486a4b18d3","0xc731f90c0df8fd2a27268bb7942ea7a53e0861ddd57227869645e5157f685913","0x952dc77591ca272bcb010e6acce188a078be41ca4598987ef122e28c2ae9d707"],"data":"0x000000000000000000000000cf5becb7245e2e6ee2e092f0bd63f6bd79ef19fe6c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000005dca9f440000000000000000000000000000000000000000000000000000000000674f9800000000000000000000000000000000000000000000000000000000009896800000000000000000000000000000000000000000000000000000000000989680","blockNumber":"0x672888","timeStamp":"0x5dcaa1cb","gasPrice":"0x3b9aca00","gasUsed":"0x43bac","logIndex":"0x7","transactionHash":"0x196ee30fa9076bcb4b1e04a37df215ef754c27db7cdca926395116a2971ab1cf","transactionIndex":"0x39"},{"address":"0x16d5195fe8c6ba98b2f61a9a787bc0bde19e3f6f","topics":["0x924028c31cbef81354a146f585e1c91ea6a9caa2a9880e0e2f195cb8894823aa","0x000000000000000000000000603a2abcbb0414a5c13a8bb22c20daf2f9388ad8","0xef85676f7752cb4d76942df4fff5c46a4e57dec88aa96766ddafe084cbe59421","0xbf19265f61734f9e5483b03aa5b97693dee83c88858a2cda0de6fd55b01624fc"],"data":"0x000000000000000000000000cf5becb7245e2e6ee2e092f0bd63f6bd79ef19fe6c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000005dd7c7610000000000000000000000000000000000000000000000000000000000684a1e00000000000000000000000000000000000000000000000000000000009896800000000000000000000000000000000000000000000000000000000000989680","blockNumber":"0x68230e","timeStamp":"0x5dd7c789","gasPrice":"0x1a13b8600","gasUsed":"0x40114","logIndex":"0x16","transactionHash":"0x42fb1b4b113a0fb9d0b2c8ce6cb888ff37bb70db4b789e300b5ed424413ad589","transactionIndex":"0x1c"}]}"#;

    let json_val: JsonValue = simple_json::parse_json(&json_str).unwrap();

    let key_status = "status";
    let key_message = "message";
    let key_result = "result";
    let key_address = "address";
    let key_topics = "topics";
    let key_data = "data";
    let key_block_number = "blockNumber";
    let key_time_stamp = "timeStamp";
    let key_transaction_hash = "transactionHash";
    let key_transaction_index = "transactionIndex";

    let mut message = AllocString::from("");
    let mut status = AllocString::from("");
    let mut results: Vec<JsonValue> = Vec::new();

    let mut htlcs: Vec<HTLC> = Vec::new();

    //TODO: check json_val.get_object().len() == 3

    json_val
        .get_object()
        .iter()
        .filter(|(k, _)| {
            key_message == k.iter().collect::<AllocString>()
                || key_status == k.iter().collect::<AllocString>()
                || key_result == k.iter().collect::<AllocString>()
        })
        .for_each(|(k, v)| {
            let key = k.iter().collect::<AllocString>();
            if key == key_message {
                message = v.get_string();
            } else if key == key_status {
                status = v.get_string();
            } else if key == key_result {
                results = v.get_array().to_vec();
            }
        });

    println!(
        "message {}, status {}, results len {}",
        message,
        status,
        &results.len()
    );

    if status != AllocString::from("1") || message != AllocString::from("OK") {
        panic!("status got {}, message got {}", status, message);
    }

    for result in results.iter() {
        let mut address: Vec<u8> = Vec::new();
        let mut topics = Vec::new();
        let mut data: Vec<u8> = Vec::new();
        let mut block_number: Vec<u8> = Vec::new();
        let mut time_stamp: Vec<u8> = Vec::new();
        let mut transaction_hash: Vec<u8> = Vec::new();
        let mut transaction_index: Vec<u8> = Vec::new();

        //TODO: check result.get_object().len() == 10

        result
            .get_object()
            .iter()
            .filter(|(k, _)| {
                key_address == k.iter().collect::<AllocString>()
                    || key_topics == k.iter().collect::<AllocString>()
                    || key_data == k.iter().collect::<AllocString>()
                    || key_block_number == k.iter().collect::<AllocString>()
                    || key_time_stamp == k.iter().collect::<AllocString>()
                    || key_transaction_hash == k.iter().collect::<AllocString>()
                    || key_transaction_index == k.iter().collect::<AllocString>()
            })
            .for_each(|(k, v)| {
                let key = k.iter().collect::<AllocString>();
                if key == key_address {
                    address = v.get_string().into_bytes();
                } else if key == key_topics {
                    for i in v.get_array().iter() {
                        topics.push(i.get_string().into_bytes());
                    }
                } else if key == key_data {
                    data = v.get_string().into_bytes();
                } else if key == key_block_number {
                    block_number = v.get_string().into_bytes();
                } else if key == key_time_stamp {
                    time_stamp = v.get_string().into_bytes();
                } else if key == key_transaction_hash {
                    transaction_hash = v.get_string().into_bytes();
                } else if key == key_transaction_index {
                    transaction_index = v.get_string().into_bytes();
                }
            });

        //println!("address {:?}", address);
        //println!("topics {:?}", topics);
        //println!("data {:?}", data);
        //println!("block_number {:?}", block_number);

        let htlc = HTLC {
            address: address,
            topics: topics,
            data: data,
            block_number: block_number,
            time_stamp: time_stamp,
            transaction_hash: transaction_hash,
            transaction_index: transaction_index,
        };

        htlcs.push(htlc);
    }

    println!("htlcs len {}", htlcs.len());
}
*/

/*
fn main() {
    let json_str = r#"{"data":{"id":"bitcoin","rank":"1","symbol":"BTC","name":"Bitcoin","supply":"18134287.0000000000000000","maxSupply":"21000000.0000000000000000","marketCapUsd":"131299748689.8711721818461171","volumeUsd24Hr":"3289762241.9455566768483118","priceUsd":"7240.4141772914023133","changePercent24Hr":"-0.1429944571894846","vwap24Hr":"7247.6143248016475241"},"timestamp":1577869002663}"#;

    let json_val: JsonValue = simple_json::parse_json(&json_str).unwrap();
    let key_id = "id";
    let key_bitcoin = "bitcoin";
    let key_price_usd = "priceUsd";

    let mut id = AllocString::from("");
    let mut price_usd = AllocString::from("");

    let data = json_val.get_object()[0].1.get_object();

    data.iter()
        .filter(|(k, _)| {
            key_id == k.iter().collect::<AllocString>()
                || key_price_usd == k.iter().collect::<AllocString>()
        })
        .for_each(|(k, v)| {
            let key = k.iter().collect::<AllocString>();
            if key == key_id {
                id = v.get_string();
            } else if key == key_price_usd {
                price_usd = v.get_string();
            }
        });

    if id == key_bitcoin {
        println!("id {:?}, price_usd {:?}", id, price_usd);
    }
}
*/

/*
fn main() {
    let json_str = r#"{"USD":7208.7}"#;

    let json_val: JsonValue = simple_json::parse_json(&json_str).unwrap();

    let val_f64: f64 = json_val.get_object()[0].1.get_number_f64();
    println!("val_f64 {:?}", val_f64);
}
*/

fn main() {
    let json_str = r#"{"status":{"timestamp":"2020-01-02T06:42:50.600Z","error_code":0,"error_message":null,"elapsed":7,"credit_count":1,"notice":null},"data":{"2275":{"id":2275,"name":"ProChain","symbol":"PRA","slug":"prochain","num_market_pairs":5,"date_added":"2017-12-13T00:00:00.000Z","tags":[],"max_supply":null,"circulating_supply":49562749.45,"total_supply":100000000,"platform":{"id":1027,"name":"Ethereum","symbol":"ETH","slug":"ethereum","token_address":"0x9041fe5b3fdea0f5e4afdc17e75180738d877a01"},"cmc_rank":493,"last_updated":"2020-01-02T06:42:06.000Z","quote":{"USD":{"price":0.0709146544966,"volume_24h":576640.229147533,"percent_change_1h":-0.945668,"percent_change_24h":-5.63962,"percent_change_7d":-0.429565,"market_cap":3514725.253148302,"last_updated":"2020-01-02T06:42:06.000Z"}}}}}"#;

    let json_val = simple_json::parse_json(&json_str);
    if let Err(e) = json_val {
        println!("Err {:?}", e);
        return;
    }

    let json_val: JsonValue = json_val.unwrap();
    let key_error_code = "error_code";
    let key_symbol = "symbol";
    let key_quote = "quote";
    let key_price = "price";

    let mut error_code: i64 = -1;

    if json_val.get_object().len() == 2 {
        let status: &JsonObject = json_val.get_object()[0].1.get_object();

        status
            .iter()
            .filter(|(k, _)| key_error_code == k.iter().collect::<AllocString>())
            .for_each(|(k, v)| {
                let key = k.iter().collect::<AllocString>();
                if key == key_error_code {
                    if let JsonValue::Number(obj) = v {
                        error_code = obj.integer;
                        if error_code != 0 {
                            println!("Err error_code {}", error_code);
                            return;
                        }
                    }
                }
            });

        let data: &JsonObject = json_val.get_object()[1].1.get_object()[0].1.get_object();

        let mut usd_obj: &JsonObject = status;
        let mut found_usd_obj = false;
        let mut price_f64: f64 = 0.0;

        data.iter()
            .filter(|(k, _)| {
                key_symbol == k.iter().collect::<AllocString>()
                    || key_quote == k.iter().collect::<AllocString>()
            })
            .for_each(|(k, v)| {
                let key = k.iter().collect::<AllocString>();
                if key == key_symbol {
                    let value = v.get_string();
                    if value != "PRA" {
                        println!("Err value {}", value);
                        return;
                    }
                } else if key == key_quote {
                    let usd = v.get_object()[0].0.clone();
                    if usd != vec!['U', 'S', 'D'] {
                        println!("Err usd {:?}", usd);
                        return;
                    }

                    usd_obj = v.get_object()[0].1.get_object();
                    found_usd_obj = true;
                }
            });

        if !found_usd_obj {
            println!("Err not_found_usd_obj");
            return;
        }

        usd_obj
            .iter()
            .filter(|(k, _)| key_price == k.iter().collect::<AllocString>())
            .for_each(|(k, v)| {
                let key = k.iter().collect::<AllocString>();
                if key == key_price {
                    price_f64 = v.get_number_f64();
                }
            });

        println!("price_f64: {:?}", price_f64);
    } else {
        println!("Err not valid json len() {:?}", json_val.get_object().len());
    }
}
