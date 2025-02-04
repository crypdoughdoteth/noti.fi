use crate::errors::Errors;
use move_core_types::language_storage::StructTag;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use sui_sdk::{
    rpc_types::{
        SuiData, SuiMoveValue, SuiObjectDataFilter, SuiObjectDataOptions, SuiObjectResponseQuery,
    },
    types::{
        base_types::{ObjectID, SuiAddress},
        id::UID,
    },
    SuiClientBuilder,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct SuilendAccountFields {
    id: UID,
    obligation_id: SuiAddress,
}

pub struct SuilendAccount;

impl SuilendAccount {
    pub async fn get_suilend_accounts(address: SuiAddress) -> Result<(), Errors> {
        let client = SuiClientBuilder::default()
            .build("https://fullnode.mainnet.sui.io:443")
            .await?;

        let query = SuiObjectResponseQuery::new(
            Some(SuiObjectDataFilter::StructType(
                    StructTag::from_str("0xf95b06141ed4a174f239417323bde3f209b972f5930d8521ea38a52aff3a6ddf::lending_market::ObligationOwnerCap<0xf95b06141ed4a174f239417323bde3f209b972f5930d8521ea38a52aff3a6ddf::suilend::MAIN_POOL>").unwrap(),
            )),
            Some(SuiObjectDataOptions {
                show_content: true,
                show_type: true,
                show_owner: false,
                show_previous_transaction: false,
                show_display: false,
                show_bcs: false,
                show_storage_rebate: false,
            })
        );

        let suilend_account = client
            .read_api()
            .get_owned_objects(address, Some(query), None, None)
            .await?
            .data;

        for e in suilend_account.into_iter() {
            let data = e
                .data
                .unwrap()
                .content
                .unwrap()
                .try_as_move()
                .unwrap()
                .fields
                .field_value("obligation_id")
                .unwrap();

            if let SuiMoveValue::Address(obligation_id) = data {
                let obligation = client
                    .read_api()
                    .get_object_with_options(
                        ObjectID::from(obligation_id),
                        SuiObjectDataOptions {
                            show_content: true,
                            show_type: true,
                            show_owner: false,
                            show_previous_transaction: false,
                            show_display: false,
                            show_bcs: false,
                            show_storage_rebate: false,
                        },
                    )
                    .await?;

                let sample = obligation.data.unwrap().content.unwrap().try_into_move().unwrap();
                println!("{sample:#?}");
            }
        }

        Ok(())
    }
}
