/*
 * Copyright 2019 Cargill Incorporated
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * -----------------------------------------------------------------------------
 */

#[cfg(feature = "contract-address")]
pub mod address;
#[cfg(feature = "contract-archive")]
pub mod archive;
#[cfg(feature = "contract-context")]
pub mod context;
#[cfg(feature = "contract-engine")]
pub mod engine;

use crate::contract::engine::SmartContractEngine;
use crate::handler::{ApplyError, TransactionContext};
use crate::protocol::transaction::TransactionPair;

pub trait SmartContract: Send {
    fn apply(
        &self,
        transaction: &TransactionPair,
        context: &mut dyn TransactionContext,
    ) -> Result<(), ApplyError>;
}

impl SmartContractEngine for dyn SmartContract {
    fn apply_smart_contract(
        &self,
        transaction: &TransactionPair,
        context: &mut dyn TransactionContext,
    ) -> Result<(), ApplyError> {
        SmartContract::apply(self, transaction, context)
    }
}
