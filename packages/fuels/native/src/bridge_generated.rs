#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.74.0.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

use crate::model::balance::Balance;
use crate::model::receipt::PanicInstruction;
use crate::model::receipt::Receipt;
use crate::model::receipt::ScriptExecutionResult;
use crate::model::response::TransferResponse;
use crate::model::transaction::TxParameters;

// Section: wire functions

fn wire_new_random__static_method__WalletUnlocked_impl(
    port_: MessagePort,
    provider: impl Wire2Api<Option<Provider>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "new_random__static_method__WalletUnlocked",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_provider = provider.wire2api();
            move |task_callback| Ok(WalletUnlocked::new_random(api_provider))
        },
    )
}
fn wire_new_from_private_key__static_method__WalletUnlocked_impl(
    port_: MessagePort,
    private_key: impl Wire2Api<String> + UnwindSafe,
    provider: impl Wire2Api<Option<Provider>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "new_from_private_key__static_method__WalletUnlocked",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_private_key = private_key.wire2api();
            let api_provider = provider.wire2api();
            move |task_callback| {
                Ok(WalletUnlocked::new_from_private_key(
                    api_private_key,
                    api_provider,
                ))
            }
        },
    )
}
fn wire_new_from_mnemonic_phrase__static_method__WalletUnlocked_impl(
    port_: MessagePort,
    phrase: impl Wire2Api<String> + UnwindSafe,
    provider: impl Wire2Api<Option<Provider>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "new_from_mnemonic_phrase__static_method__WalletUnlocked",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_phrase = phrase.wire2api();
            let api_provider = provider.wire2api();
            move |task_callback| {
                Ok(WalletUnlocked::new_from_mnemonic_phrase(
                    api_phrase,
                    api_provider,
                ))
            }
        },
    )
}
fn wire_new_from_mnemonic_phrase_with_path__static_method__WalletUnlocked_impl(
    port_: MessagePort,
    phrase: impl Wire2Api<String> + UnwindSafe,
    provider: impl Wire2Api<Option<Provider>> + UnwindSafe,
    path: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "new_from_mnemonic_phrase_with_path__static_method__WalletUnlocked",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_phrase = phrase.wire2api();
            let api_provider = provider.wire2api();
            let api_path = path.wire2api();
            move |task_callback| {
                Ok(WalletUnlocked::new_from_mnemonic_phrase_with_path(
                    api_phrase,
                    api_provider,
                    api_path,
                ))
            }
        },
    )
}
fn wire_address__method__WalletUnlocked_impl(
    port_: MessagePort,
    that: impl Wire2Api<WalletUnlocked> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "address__method__WalletUnlocked",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Ok(WalletUnlocked::address(&api_that))
        },
    )
}
fn wire_get_asset_balance__method__WalletUnlocked_impl(
    port_: MessagePort,
    that: impl Wire2Api<WalletUnlocked> + UnwindSafe,
    asset: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_asset_balance__method__WalletUnlocked",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_asset = asset.wire2api();
            move |task_callback| Ok(WalletUnlocked::get_asset_balance(&api_that, api_asset))
        },
    )
}
fn wire_get_balances__method__WalletUnlocked_impl(
    port_: MessagePort,
    that: impl Wire2Api<WalletUnlocked> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_balances__method__WalletUnlocked",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Ok(WalletUnlocked::get_balances(&api_that))
        },
    )
}
fn wire_transfer__method__WalletUnlocked_impl(
    port_: MessagePort,
    that: impl Wire2Api<WalletUnlocked> + UnwindSafe,
    to: impl Wire2Api<Bech32Address> + UnwindSafe,
    amount: impl Wire2Api<u64> + UnwindSafe,
    asset: impl Wire2Api<String> + UnwindSafe,
    tx_parameters: impl Wire2Api<TxParameters> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "transfer__method__WalletUnlocked",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_to = to.wire2api();
            let api_amount = amount.wire2api();
            let api_asset = asset.wire2api();
            let api_tx_parameters = tx_parameters.wire2api();
            move |task_callback| {
                Ok(WalletUnlocked::transfer(
                    &api_that,
                    api_to,
                    api_amount,
                    api_asset,
                    api_tx_parameters,
                ))
            }
        },
    )
}
fn wire_from_bech32_string__static_method__Bech32Address_impl(
    port_: MessagePort,
    s: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "from_bech32_string__static_method__Bech32Address",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_s = s.wire2api();
            move |task_callback| Ok(Bech32Address::from_bech32_string(api_s))
        },
    )
}
fn wire_from_b256_string__static_method__Bech32Address_impl(
    port_: MessagePort,
    s: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "from_b256_string__static_method__Bech32Address",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_s = s.wire2api();
            move |task_callback| Ok(Bech32Address::from_b256_string(api_s))
        },
    )
}
fn wire_to_bech32_string__method__Bech32Address_impl(
    port_: MessagePort,
    that: impl Wire2Api<Bech32Address> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "to_bech32_string__method__Bech32Address",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Ok(Bech32Address::to_bech32_string(&api_that))
        },
    )
}
fn wire_to_b256_string__method__Bech32Address_impl(
    port_: MessagePort,
    that: impl Wire2Api<Bech32Address> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "to_b256_string__method__Bech32Address",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Ok(Bech32Address::to_b256_string(&api_that))
        },
    )
}
fn wire_connect__static_method__Provider_impl(
    port_: MessagePort,
    url: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "connect__static_method__Provider",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_url = url.wire2api();
            move |task_callback| Ok(Provider::connect(api_url))
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<u32> for u32 {
    fn wire2api(self) -> u32 {
        self
    }
}
impl Wire2Api<u64> for u64 {
    fn wire2api(self) -> u64 {
        self
    }
}
impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

// Section: impl IntoDart

impl support::IntoDart for Balance {
    fn into_dart(self) -> support::DartAbi {
        vec![self.asset.into_dart(), self.amount.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Balance {}

impl support::IntoDart for Bech32Address {
    fn into_dart(self) -> support::DartAbi {
        vec![self.native.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Bech32Address {}

impl support::IntoDart for PanicInstruction {
    fn into_dart(self) -> support::DartAbi {
        vec![self.reason.into_dart(), self.instruction.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for PanicInstruction {}

impl support::IntoDart for Provider {
    fn into_dart(self) -> support::DartAbi {
        vec![self.node_url.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Provider {}

impl support::IntoDart for Receipt {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::Call {
                id,
                to,
                amount,
                asset_id,
                gas,
                param1,
                param2,
                pc,
                is_field,
            } => vec![
                0.into_dart(),
                id.into_dart(),
                to.into_dart(),
                amount.into_dart(),
                asset_id.into_dart(),
                gas.into_dart(),
                param1.into_dart(),
                param2.into_dart(),
                pc.into_dart(),
                is_field.into_dart(),
            ],
            Self::ReturnReceipt {
                id,
                val,
                pc,
                is_field,
            } => vec![
                1.into_dart(),
                id.into_dart(),
                val.into_dart(),
                pc.into_dart(),
                is_field.into_dart(),
            ],
            Self::ReturnData {
                id,
                ptr,
                len,
                digest,
                data,
                pc,
                is_field,
            } => vec![
                2.into_dart(),
                id.into_dart(),
                ptr.into_dart(),
                len.into_dart(),
                digest.into_dart(),
                data.into_dart(),
                pc.into_dart(),
                is_field.into_dart(),
            ],
            Self::Panic {
                id,
                reason,
                pc,
                is_field,
                contract_id,
            } => vec![
                3.into_dart(),
                id.into_dart(),
                reason.into_dart(),
                pc.into_dart(),
                is_field.into_dart(),
                contract_id.into_dart(),
            ],
            Self::Revert {
                id,
                ra,
                pc,
                is_field,
            } => vec![
                4.into_dart(),
                id.into_dart(),
                ra.into_dart(),
                pc.into_dart(),
                is_field.into_dart(),
            ],
            Self::Log {
                id,
                ra,
                rb,
                rc,
                rd,
                pc,
                is_field,
            } => vec![
                5.into_dart(),
                id.into_dart(),
                ra.into_dart(),
                rb.into_dart(),
                rc.into_dart(),
                rd.into_dart(),
                pc.into_dart(),
                is_field.into_dart(),
            ],
            Self::LogData {
                id,
                ra,
                rb,
                ptr,
                len,
                digest,
                data,
                pc,
                is_field,
            } => vec![
                6.into_dart(),
                id.into_dart(),
                ra.into_dart(),
                rb.into_dart(),
                ptr.into_dart(),
                len.into_dart(),
                digest.into_dart(),
                data.into_dart(),
                pc.into_dart(),
                is_field.into_dart(),
            ],
            Self::Transfer {
                id,
                to,
                amount,
                asset_id,
                pc,
                is_field,
            } => vec![
                7.into_dart(),
                id.into_dart(),
                to.into_dart(),
                amount.into_dart(),
                asset_id.into_dart(),
                pc.into_dart(),
                is_field.into_dart(),
            ],
            Self::TransferOut {
                id,
                to,
                amount,
                asset_id,
                pc,
                is_field,
            } => vec![
                8.into_dart(),
                id.into_dart(),
                to.into_dart(),
                amount.into_dart(),
                asset_id.into_dart(),
                pc.into_dart(),
                is_field.into_dart(),
            ],
            Self::ScriptResult { result, gas_used } => {
                vec![9.into_dart(), result.into_dart(), gas_used.into_dart()]
            }
            Self::MessageOut {
                sender,
                recipient,
                amount,
                nonce,
                len,
                digest,
                data,
            } => vec![
                10.into_dart(),
                sender.into_dart(),
                recipient.into_dart(),
                amount.into_dart(),
                nonce.into_dart(),
                len.into_dart(),
                digest.into_dart(),
                data.into_dart(),
            ],
            Self::Mint {
                sub_id,
                contract_id,
                val,
                pc,
                is_field,
            } => vec![
                11.into_dart(),
                sub_id.into_dart(),
                contract_id.into_dart(),
                val.into_dart(),
                pc.into_dart(),
                is_field.into_dart(),
            ],
            Self::Burn {
                sub_id,
                contract_id,
                val,
                pc,
                is_field,
            } => vec![
                12.into_dart(),
                sub_id.into_dart(),
                contract_id.into_dart(),
                val.into_dart(),
                pc.into_dart(),
                is_field.into_dart(),
            ],
        }
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Receipt {}
impl support::IntoDart for ScriptExecutionResult {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::Success => vec![0.into_dart()],
            Self::Revert => vec![1.into_dart()],
            Self::Panic => vec![2.into_dart()],
            Self::GenericFailure(field0) => vec![3.into_dart(), field0.into_dart()],
        }
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for ScriptExecutionResult {}
impl support::IntoDart for TransferResponse {
    fn into_dart(self) -> support::DartAbi {
        vec![self.tx_id.into_dart(), self.receipts.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for TransferResponse {}

impl support::IntoDart for WalletUnlocked {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.private_key.into_dart(),
            self.mnemonic_phrase.into_dart(),
            self.provider.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for WalletUnlocked {}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
