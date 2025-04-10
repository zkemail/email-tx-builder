pub use user_overridable_dkim_registry::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod user_overridable_dkim_registry {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("REACTIVATE_PREFIX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("REACTIVATE_PREFIX"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("REVOKE_PREFIX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("REVOKE_PREFIX"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SET_PREFIX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("SET_PREFIX"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UPGRADE_INTERFACE_VERSION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UPGRADE_INTERFACE_VERSION",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("changeMainAuthorizer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "changeMainAuthorizer",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newMainAuthorizer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("computeSignedMsg"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("computeSignedMsg"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("prefix"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("domainName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("publicKeyHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dkimPublicKeyHashes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "dkimPublicKeyHashes",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("enabledTimeOfDKIMPublicKeyHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "enabledTimeOfDKIMPublicKeyHash",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_initialOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_mainAuthorizer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_setTimestampDelay",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isDKIMPublicKeyHashValid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isDKIMPublicKeyHashValid",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("domainName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("publicKeyHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("authorizer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isDKIMPublicKeyHashValid",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("domainName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("publicKeyHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mainAuthorizer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mainAuthorizer"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("proxiableUUID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proxiableUUID"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("reactivateDKIMPublicKeyHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "reactivateDKIMPublicKeyHash",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("domainName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("publicKeyHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("authorizer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("reactivatedDKIMPublicKeyHashes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "reactivatedDKIMPublicKeyHashes",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("revokeDKIMPublicKeyHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "revokeDKIMPublicKeyHash",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("domainName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("publicKeyHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("authorizer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("revokedDKIMPublicKeyHashes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "revokedDKIMPublicKeyHashes",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setDKIMPublicKeyHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setDKIMPublicKeyHash",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("domainName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("publicKeyHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("authorizer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setDKIMPublicKeyHashes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setDKIMPublicKeyHashes",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("domainNames"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("publicKeyHashes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("authorizers"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signatures"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setTimestampDelay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setTimestampDelay"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("upgradeToAndCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upgradeToAndCall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newImplementation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DKIMPublicKeyHashReactivated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DKIMPublicKeyHashReactivated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("publicKeyHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("authorizer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DKIMPublicKeyHashRegistered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DKIMPublicKeyHashRegistered",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("domainName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("publicKeyHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("authorizer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DKIMPublicKeyHashRevoked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DKIMPublicKeyHashRevoked",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("publicKeyHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("authorizer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Initialized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MainAuthorizerChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MainAuthorizerChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newMainAuthorizer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Upgraded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Upgraded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("implementation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ECDSAInvalidSignature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ECDSAInvalidSignature",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ECDSAInvalidSignatureLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ECDSAInvalidSignatureLength",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("length"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ECDSAInvalidSignatureS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ECDSAInvalidSignatureS",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1967InvalidImplementation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC1967InvalidImplementation",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("implementation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1967NonPayable"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ERC1967NonPayable"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FailedCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("FailedCall"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidInitialization"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidInitialization",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotInitializing"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotInitializing"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnableInvalidOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnableInvalidOwner",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnableUnauthorizedAccount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnableUnauthorizedAccount",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StringsInsufficientHexLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StringsInsufficientHexLength",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("length"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UUPSUnauthorizedCallContext"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UUPSUnauthorizedCallContext",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UUPSUnsupportedProxiableUUID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UUPSUnsupportedProxiableUUID",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("slot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static USEROVERRIDABLEDKIMREGISTRY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0\x80`@R4`)W0`\x80Ra-<\x90\x81a\0.\x829`\x80Q\x81\x81\x81a\x0B\x80\x01Ra\x0CC\x01R\xF3[_\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x07\xF1\xEA\xF5\x14a\x17\xA8W\x80c\x0BU\xB3|\x14a\x17\x8FW\x80c\x17\x94\xBB<\x14a\x15>W\x80c\"Z\x08\xD4\x14a\x15#W\x80c2\xE1\xE1\x94\x14a\x11:W\x80cK\xCB\xBE\x96\x14a\x10\xD7W\x80cL\x93\x06\x07\x14a\x0F\0W\x80cO\x1E\xF2\x86\x14a\x0B\xF8W\x80cR\xD1\x90-\x14a\x0BYW\x80cWI\0\xDD\x14a\x0B\x02W\x80caJD\x85\x14a\n\xE8W\x80cqP\x18\xA6\x14a\n,W\x80c}F6H\x14a\t\xFAW\x80c\x7F\x8E)\xBA\x14a\t\xD0W\x80c\x7F\xF1\x03\xDA\x14a\x06\tW\x80c\x81.\x12\xCE\x14a\x05\xECW\x80c\x82\xBF\xF8\xCD\x14a\x03\xD5W\x80c\x8D\xA5\xCB[\x14a\x03\x83W\x80c\xAD<\xB1\xCC\x14a\x03$W\x80c\xD5\x07\xC3 \x14a\x02\xF1W\x80c\xE3\x08\xDE\x0C\x14a\x02\x89W\x80c\xE7\xA7\x97z\x14a\x01\xA7W\x80c\xF0\xBF\xB1\x97\x14a\x01PWc\xF2\xFD\xE3\x8B\x14a\x01!W_\x80\xFD[4a\x01LW` `\x03\x196\x01\x12a\x01LWa\x01Ja\x01=a\x19-V[a\x01Ea*\tV[a\"\xF3V[\0[_\x80\xFD[4a\x01LW`@`\x03\x196\x01\x12a\x01LWa\x01ia\x19PV[`\x045_R`\x03` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x01LW`@`\x03\x196\x01\x12a\x01LW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01LWa\x01\xD8\x906\x90`\x04\x01a\x18\xE9V[`@Q\x7F\x8D\xA5\xCB[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x813Z\xFA\x90\x81\x15a\x02~W_\x91a\x02.W[` a\x02$\x83`$5\x86a\x1C\x1DV[`@Q\x90\x15\x15\x81R\xF3[\x90P` \x81=` \x11a\x02vW[\x81a\x02I` \x93\x83a\x17\xC3V[\x81\x01\x03\x12a\x01LWQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x01LW` a\x02\x15V[=\x91Pa\x02<V[`@Q=_\x82>=\x90\xFD[4a\x01LW` \x80a\x02\x9A6a\x19sV[\x93\x91`@\x93\x91\x93Q\x92\x81\x84\x92Q\x91\x82\x91\x01\x83^\x81\x01`\x02\x81R\x03\x01\x90 \x90_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x01LW_`\x03\x196\x01\x12a\x01LWa\x03 a\x03\x0Ca\x1A\x8DV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x18\xA6V[\x03\x90\xF3[4a\x01LW_`\x03\x196\x01\x12a\x01LWa\x03 `@Qa\x03E`@\x82a\x17\xC3V[`\x05\x81R\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x18\xA6V[4a\x01LW_`\x03\x196\x01\x12a\x01LW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x16`@Q\x90\x81R\xF3[4a\x01LWa\x03\xE36a\x1A\x01V[\x90a\x03\xF0\x84Q\x15\x15a\x1A\xC8V[a\x03\xFB\x83\x15\x15a\x1B-V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x94a\x04\x1F\x82\x15\x15a\x1B\x92V[\x84_R`\x03` R`@_ \x82_R` Ra\x04B`\xFF`@_ T\x16\x15a\x1F$V[\x813\x03a\x04\xB1W[P\x83\x80_R`\x03` R`@_ \x82_R` R`@_ `\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90U\x7F5P6\xB8\xAD\x96>\x18^\t\xF0t\xE8VU\x96H:\0\x12\xCB\xE6 \xF5\x07\xC0\xF3IP\xA2\xF0\xB3_\x80\xA3\0[a\x04\xD0a\x04\xD5\x91a\x04\xC0a\x1A\x8DV[\x90a\x04\xCA\x88a(KV[\x91a\x1EEV[a%\xE6V[\x91;\x15a\x05\xB1WPa\x05\x18\x91` \x91`@Q\x93\x84\x92\x83\x92\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01a\x1D\xC9V[\x03\x81\x86Z\xFA\x80\x15a\x02~W\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91a\x05y\x93_\x91a\x05\x82W[P\x16\x14a\x1D\xE0V[\x82\x80\x83\x81a\x04JV[a\x05\xA4\x91P` =` \x11a\x05\xAAW[a\x05\x9C\x81\x83a\x17\xC3V[\x81\x01\x90a\x1D\x91V[\x86a\x05qV[P=a\x05\x92V[\x90a\x05\xE0a\x05\xD7a\x05\xE7\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93a*\xCCV[\x90\x92\x91\x92a+\x06V[\x16\x14a\x1D,V[a\x05yV[4a\x01LW_`\x03\x196\x01\x12a\x01LW` `\x01T`@Q\x90\x81R\xF3[4a\x01LW`\x80`\x03\x196\x01\x12a\x01LW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01LW6`#\x82\x01\x12\x15a\x01LW\x80`\x04\x015a\x06E\x81a\x1AuV[\x91a\x06S`@Q\x93\x84a\x17\xC3V[\x81\x83R`$` \x84\x01\x92`\x05\x1B\x82\x01\x01\x906\x82\x11a\x01LW`$\x81\x01\x92[\x82\x84\x10a\t\xA0W\x84`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01LW6`#\x82\x01\x12\x15a\x01LW\x80`\x04\x015\x90a\x06\xA5\x82a\x1AuV[\x91a\x06\xB3`@Q\x93\x84a\x17\xC3V[\x80\x83R`$` \x84\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x01LW`$\x01\x90[\x82\x82\x10a\t\x90WPPP`D5\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x01LW6`#\x84\x01\x12\x15a\x01LW\x82`\x04\x015a\x07\x06\x81a\x1AuV[\x93a\x07\x14`@Q\x95\x86a\x17\xC3V[\x81\x85R`$` \x86\x01\x92`\x05\x1B\x82\x01\x01\x906\x82\x11a\x01LW`$\x01\x91[\x81\x83\x10a\tcWPPP`d5\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x01LW6`#\x84\x01\x12\x15a\x01LW\x82`\x04\x015a\x07g\x81a\x1AuV[\x93a\x07u`@Q\x95\x86a\x17\xC3V[\x81\x85R`$` \x86\x01\x92`\x05\x1B\x82\x01\x01\x906\x82\x11a\x01LW`$\x81\x01\x92[\x82\x84\x10a\t3WPPPP\x81Q\x81Q\x03a\x08\xD5W\x81Q\x84Q\x03a\x08wW\x81Q\x83Q\x03a\x08\x19W_[\x82Q\x81\x10\x15a\x01JW\x80a\x08\x13a\x07\xD4`\x01\x93\x86a\"\xB2V[Qa\x07\xDF\x83\x86a\"\xB2V[Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x07\xFF\x85\x8Ba\"\xB2V[Q\x16\x90a\x08\x0C\x85\x8Aa\"\xB2V[Q\x92a\x1F\xAFV[\x01a\x07\xBBV[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Finvalid signatures length\0\0\0\0\0\0\0`D\x82\x01R\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Finvalid authorizers length\0\0\0\0\0\0`D\x82\x01R\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Finvalid publicKeyHashes length\0\0`D\x82\x01R\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01LW` \x91a\tX\x83\x92`$6\x91\x87\x01\x01a\x18\xE9V[\x81R\x01\x93\x01\x92a\x07\x93V[\x825s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x01LW\x81R` \x92\x83\x01\x92\x01a\x071V[\x815\x81R` \x91\x82\x01\x91\x01a\x06\xD0V[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01LW` \x91a\t\xC5\x83\x92`$6\x91\x87\x01\x01a\x18\xE9V[\x81R\x01\x93\x01\x92a\x06qV[4a\x01LW` `\x03\x196\x01\x12a\x01LW`\x045_R`\x05` R` `@_ T`@Q\x90\x81R\xF3[4a\x01LW_`\x03\x196\x01\x12a\x01LW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x16`@Q\x90\x81R\xF3[4a\x01LW_`\x03\x196\x01\x12a\x01LWa\nDa*\tV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x01LWa\x01Ja\n\xF96a\x1A\x01V[\x92\x91\x90\x91a\x1F\xAFV[4a\x01LW`@`\x03\x196\x01\x12a\x01LWa\x0B\x1Ba\x19PV[`\x045_R`\x04` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x01LW_`\x03\x196\x01\x12a\x01LWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x0B\xD0W` `@Q\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81R\xF3[\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`@`\x03\x196\x01\x12a\x01LWa\x0C\x0Ca\x19-V[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01LWa\x0C,\x906\x90`\x04\x01a\x18\xE9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x800\x14\x90\x81\x15a\x0E\xBEW[Pa\x0B\xD0Wa\x0C{a*\tV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x91`@Q\x7FR\xD1\x90-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x87Z\xFA_\x91\x81a\x0E\x8AW[Pa\x0C\xFBW\x83\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x80\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x85\x92\x03a\x0E_WP\x81;\x15a\x0E4W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16\x17\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCU\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;_\x80\xA2\x81Q\x15a\x0E\x03W_\x80\x83` a\x01J\x95Q\x91\x01\x84Z\xF4=\x15a\r\xFBW=\x91a\r\xDF\x83a\x181V[\x92a\r\xED`@Q\x94\x85a\x17\xC3V[\x83R=_` \x85\x01>a,mV[``\x91a,mV[PP4a\x0E\x0CW\0[\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x90\x91P` \x81=` \x11a\x0E\xB6W[\x81a\x0E\xA6` \x93\x83a\x17\xC3V[\x81\x01\x03\x12a\x01LWQ\x90\x85a\x0C\xCAV[=\x91Pa\x0E\x99V[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16\x14\x15\x83a\x0CnV[4a\x01LW` `\x03\x196\x01\x12a\x01LWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x0F.a\x19-V[a\x0F6a*\tV[\x16\x80\x15a\x10SW_Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x82\x14a\x0F\xA9W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x91\x16\x17_U\x7F;\xB1\x96\x11\xD1\x15f1\xA8\\Y\xDD\xFEvhT\x1A/\0\xE6\xBA+~q\xCB\x0C`\xEC\xE0\xD5\xE5[_\x80\xA2\0[`\xA4`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FnewMainAuthorizer address cannot`D\x82\x01R\x7F be the same as the current main`d\x82\x01R\x7FAuthorizer\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FnewMainAuthorizer address cannot`D\x82\x01R\x7F be zero\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[4a\x01LW```\x03\x196\x01\x12a\x01LW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01LWa\x11\x08\x906\x90`\x04\x01a\x18\xE9V[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01LWa\x03 \x91a\x11/a\x03\x0C\x926\x90`\x04\x01a\x18\xE9V[a\x04\xCA`D5a(KV[4a\x01LWa\x11H6a\x1A\x01V[\x90a\x11U\x84Q\x15\x15a\x1A\xC8V[a\x11`\x83\x15\x15a\x1B-V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x93a\x11\x83\x85\x15\x15a\x1B\x92V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x16\x85\x14a\x14\x9FW\x83_R`\x04` R`@_ \x85_R` R`\xFF`@_ T\x16a\x14\x1BW`\x01a\x11\xCB\x83\x86a#\xE0V[\x03a\x13\xBDW`\x02a\x11\xDD\x83\x86\x84a$\xF3V[\x10a\x139W\x84\x90\x813\x03a\x12SW[P\x83\x80_R`\x04` R`@_ \x82_R` R`@_ `\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90U\x7F2\x89\x9A\x1E\xA4\xD8\xE4\x91|k=l\x1Ci\xFDLp\x949C\xB0'\xFE\x9D\x83+ R\xE7\xEF\xF8\xD6_\x80\xA3\0[a\x04\xD0a\x12b\x91a\x04\xC0a\x19\xC6V[\x91;\x15a\x13\x0EWPa\x12\xA5\x91` \x91`@Q\x93\x84\x92\x83\x92\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01a\x1D\xC9V[\x03\x81\x86Z\xFA\x80\x15a\x02~W\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91a\x13\x05\x93_\x91a\x05\x82WP\x16\x14a\x1D\xE0V[\x82\x80\x83\x81a\x11\xECV[\x90a\x05\xE0a\x05\xD7a\x134\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93a*\xCCV[a\x13\x05V[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7Fset threshold must be larger tha`D\x82\x01R\x7Fn two\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Frevoke threshold must be one\0\0\0\0`D\x82\x01R\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7Fpublic key hash is already react`D\x82\x01R\x7Fivated\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FmainAuthorizer cannot reactivate`D\x82\x01R\x7F the public key hash\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[4a\x01LW_`\x03\x196\x01\x12a\x01LWa\x03 a\x03\x0Ca\x19\xC6V[4a\x01LW```\x03\x196\x01\x12a\x01LWa\x15Wa\x19-V[a\x15_a\x19PV[\x90\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x91`\xFF\x83`@\x1C\x16\x15\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x15\x90\x81a\x17\x87W[`\x01\x14\x90\x81a\x17}W[\x15\x90\x81a\x17tW[Pa\x17LWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x81\x85`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0a\x160\x95\x16\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0Ua\x16\xF7W[Pa\x16(a*uV[a\x01Ea*uV[\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_T\x16\x17_U`D5`\x01Ua\x16dW\0[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x16\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2` `@Q`\x01\x81R\xA1\0[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x16h\x01\0\0\0\0\0\0\0\x01\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U\x85a\x16\x1FV[\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x90P\x15\x85a\x15\xB2V[0;\x15\x91Pa\x15\xAAV[\x85\x91Pa\x15\xA0V[4a\x01LW` a\x02$a\x17\xA26a\x19sV[\x91a\x1C\x1DV[4a\x01LW_`\x03\x196\x01\x12a\x01LWa\x03 a\x03\x0Ca\x18kV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x18\x04W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x18\x04W`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[`@Q\x90a\x18z`@\x83a\x17\xC3V[`\x04\x82R\x7FSET:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01RV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x81`\x1F\x82\x01\x12\x15a\x01LW` \x815\x91\x01a\x19\x03\x82a\x181V[\x92a\x19\x11`@Q\x94\x85a\x17\xC3V[\x82\x84R\x82\x82\x01\x11a\x01LW\x81_\x92` \x92\x83\x86\x017\x83\x01\x01R\x90V[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01LWV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01LWV[```\x03\x19\x82\x01\x12a\x01LW`\x045\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01LWa\x19\x9E\x91`\x04\x01a\x18\xE9V[\x90`$5\x90`D5s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x01LW\x90V[`@Q\x90a\x19\xD5`@\x83a\x17\xC3V[`\x0B\x82R\x7FREACTIVATE:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01RV[\x90`\x80`\x03\x19\x83\x01\x12a\x01LW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01LW\x82a\x1A-\x91`\x04\x01a\x18\xE9V[\x91`$5\x91`D5s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x01LW\x91`d5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01LWa\x1Ar\x91`\x04\x01a\x18\xE9V[\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x18\x04W`\x05\x1B` \x01\x90V[`@Q\x90a\x1A\x9C`@\x83a\x17\xC3V[`\x07\x82R\x7FREVOKE:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01RV[\x15a\x1A\xCFWV[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Fdomain name cannot be zero\0\0\0\0\0\0`D\x82\x01R\xFD[\x15a\x1B4WV[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fpublic key hash cannot be zero\0\0`D\x82\x01R\xFD[\x15a\x1B\x99WV[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7Fauthorizer address cannot be zer`D\x82\x01R\x7Fo\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[\x91\x90a\x1C+\x83Q\x15\x15a\x1A\xC8V[a\x1C6\x81\x15\x15a\x1B-V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x1CX\x81\x15\x15a\x1B\x92V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x16\x14a\x1C\xA8W\x81\x81a\x1C\x86`\x01\x94a\x1C\x8C\x94a#\xE0V[\x94a$\xF3V[\x91\x10a\x1C\x97WP_\x90V[`\x02\x11\x15a\x1C\xA3W_\x90V[`\x01\x90V[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Fauthorizer cannot be mainAuthori`D\x82\x01R\x7Fzer\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[\x15a\x1D3WV[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Finvalid ecdsa signature\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x90\x81` \x91\x03\x12a\x01LWQ\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x03a\x01LW\x90V[`@\x90a\x1Ar\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x18\xA6V[\x15a\x1D\xE7WV[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Finvalid eip1271 signature\0\0\0\0\0\0\0`D\x82\x01R\xFD[`\x11`\x01\x91`\x07` \x80\x97\x96\x81\x80a\x1F\"\x98`@Q\x9B\x87\x8D\x98Q\x91\x82\x91\x01\x84\x8A\x01^\x87\x01\x90\x7Fdomain=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x83\x01R\x80Q\x92\x83\x91\x01`'\x83\x01^\x01\x01\x90\x7F;public_key_hash=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x83\x01R\x80Q\x92\x83\x91\x01`\x18\x83\x01^\x01\x01\x7F;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x82\x01R\x03\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x81\x01\x85R\x01\x83a\x17\xC3V[V[\x15a\x1F+WV[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Fpublic key hash is already revok`D\x82\x01R\x7Fed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[\x91\x90\x91a\x1F\xBE\x81Q\x15\x15a\x1A\xC8V[a\x1F\xC9\x83\x15\x15a\x1B-V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x93a\x1F\xEC\x85\x15\x15a\x1B\x92V[`@Q\x92\x82Q\x93` \x81\x81\x86\x01\x96\x80\x88\x83^\x81\x01`\x02\x81R\x03\x01\x90 \x85_R` R`@_ \x86_R` R`\xFF`@_ T\x16a\"TW\x84_R`\x03` R`@_ \x86_R` Ra G`\xFF`@_ T\x16\x15a\x1F$V[\x853\x03a!GW[PP`@Q` \x81\x83Q\x80\x86\x83^\x81\x01`\x02\x81R\x03\x01\x90 \x83_R` R`@_ \x84_R` R`@_ `\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x16\x84\x14a \xFCW[`@Q\x91\x82\x91Q\x80\x91\x83^\x81\x01_\x81R\x03\x90 \x7F}a~\xDC\x9D\n\xDE/\xB7hC\xEF_sr\xBD'0\xE9\0\xFA\x12\xE6t\xBE\xCA\xA8\xAD\x01\xEA\xB6\xCB_\x80\xA4V[`\x01TB\x01\x80B\x11a!\x1AW\x83_R`\x05` R`@_ Ua \xC3V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x85a!`a\x04\xD0a!Va\x18kV[\x86a\x04\xCA\x8Aa(KV[\x91;\x15a\")WPa!\xA3\x91` \x91`@Q\x93\x84\x92\x83\x92\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01a\x1D\xC9V[\x03\x81\x88Z\xFA\x80\x15a\x02~W\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91a\"\x03\x93_\x91a\"\nWP\x16\x14a\x1D\xE0V[_\x80a OV[a\"#\x91P` =` \x11a\x05\xAAWa\x05\x9C\x81\x83a\x17\xC3V[_a\x05qV[\x90a\x05\xE0a\x05\xD7a\"O\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93a*\xCCV[a\"\x03V[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fpublic key hash is already set\0\0`D\x82\x01R\xFD[\x80Q\x82\x10\x15a\"\xC6W` \x91`\x05\x1B\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15a#\xB4Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[_\x91\x81_R`\x03` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80_T\x16\x16_R` R`\x01`\xFF`@_ T\x16\x15\x15\x14a$\xEAW[\x81_R`\x03` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_R` R`\x01`\xFF`@_ T\x16\x15\x15\x14a$\xD8W[`\x01\x83\x14\x91\x82a$\x9AW[PPa$mW\x90V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x01\x90\x81\x11a!\x1AW\x90V[\x90\x91P_R`\x04` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R`\x01`\xFF`@_ T\x16\x15\x15\x14_\x80a$dV[\x91`\x02\x81\x01\x80\x91\x11a!\x1AW\x91a$YV[`\x01\x92Pa$\x1FV[\x91\x90_\x92`@Q\x90\x80Q\x91` \x81\x81\x84\x01\x94\x80\x86\x83^\x81\x01`\x02\x81R\x03\x01\x90 \x83_R` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80_T\x16\x16_R` R`\x01`\xFF`@_ T\x16\x15\x15\x14a%\xAEW[` \x90`@Q\x92\x83\x91Q\x80\x91\x83^\x81\x01`\x02\x81R\x03\x01\x90 \x90_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R`\x01`\xFF`@_ T\x16\x15\x15\x14a%\xA0W\x90V[`\x02\x81\x01\x80\x91\x11a!\x1AW\x90V[\x93\x82_R`\x05` R`@_ TB\x10_\x14a%\xDBW`\x01\x81\x01\x80\x91\x11a!\x1AW` \x90\x94[\x90Pa%NV[P` `\x02\x94a%\xD4V[\x80Q\x90_\x82z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x81\x10\x15a(#W[\x80m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0`\n\x92\x10\x15a(\x08W[f#\x86\xF2o\xC1\0\0\x81\x10\x15a'\xF4W[c\x05\xF5\xE1\0\x81\x10\x15a'\xE3W[a'\x10\x81\x10\x15a'\xD4W[`d\x81\x10\x15a'\xC6W[\x10\x15a'\xBEW[`\x01\x81\x01\x92`\n\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`!a&\xAEa&\x98\x88a\x181V[\x97a&\xA6`@Q\x99\x8Aa\x17\xC3V[\x80\x89Ra\x181V[\x94\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0` \x89\x01\x96\x016\x877\x87\x01\x01[\x01\x91\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x82\x06\x1A\x83S\x04\x90\x81\x15a'8W`\n\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90a&\xDDV[PPa'\xB8\x90`:` `@Q\x94\x85\x93\x82\x85\x01\x97\x7F\x19Ethereum Signed Message:\n\0\0\0\0\0\0\x89RQ\x80\x91\x85\x87\x01^\x84\x01\x90\x83\x82\x01_\x81R\x81Q\x93\x84\x92\x01\x90^\x01\x01_\x81R\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x83R\x82a\x17\xC3V[Q\x90 \x90V[`\x01\x01a&bV[`d`\x02\x91\x04\x92\x01\x91a&[V[a'\x10`\x04\x91\x04\x92\x01\x91a&QV[c\x05\xF5\xE1\0`\x08\x91\x04\x92\x01\x91a&FV[f#\x86\xF2o\xC1\0\0`\x10\x91\x04\x92\x01\x91a&9V[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0` \x91\x04\x92\x01\x91a&)V[P`@\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x04a&\x0FV[`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x82\x81`\x07\x1B\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x90\x81`\x06\x1B\x1C\x90c\xFF\xFF\xFF\xFF\x82\x11\x91\x82`\x05\x1B\x1C\x92`\xFFa\xFF\xFF\x85\x11\x94\x85`\x04\x1B\x1C\x11\x93\x85\x1B\x92`\x02\x1B\x91`\x03\x1B\x90`\x04\x1B\x01\x01\x01\x01\x01\x81\x81`\x01\x1B\x92\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x83\x03a!\x1AW`\x02\x84\x01\x93\x84\x81\x11a!\x1AWa)\x05a(\xEF\x86a\x181V[\x95a(\xFD`@Q\x97\x88a\x17\xC3V[\x80\x87Ra\x181V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0` \x87\x01\x91\x016\x827\x85Q\x15a\"\xC6W`0\x90S\x84Q`\x01\x10\x15a\"\xC6W`x`!\x86\x01S`\x01\x81\x01\x80\x91\x11a!\x1AW[`\x01\x81\x11a)\x97WPa)iWPP\x90V[\x7F\xE2.'\xEB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x90`\x0F\x81\x16`\x10\x81\x10\x15a\"\xC6W\x85Q\x83\x10\x15a\"\xC6W\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x1A\x85\x83\x01` \x01S`\x04\x1C\x90\x80\x15a!\x1AW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01a)WV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x163\x03a*IWV[\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15a*\xA4WV[\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x81Q\x91\x90`A\x83\x03a*\xFCWa*\xF5\x92P` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90a+\xDEV[\x91\x92\x90\x91\x90V[PP_\x91`\x02\x91\x90V[`\x04\x81\x10\x15a+\xB1W\x80a+\x18WPPV[`\x01\x81\x03a+HW\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`\x02\x81\x03a+|WP\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[`\x03\x14a+\x86WPV[\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11a,bW\x91` \x93`\x80\x92`\xFF_\x95`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\x02~W_Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15a,XW\x90_\x90_\x90V[P_\x90`\x01\x90_\x90V[PPP_\x91`\x03\x91\x90V[\x90a,\xAAWP\x80Q\x15a,\x82W\x80Q\x90` \x01\xFD[\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x81Q\x15\x80a,\xFDW[a,\xBBWP\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$_\xFD[P\x80;\x15a,\xB3V\xFE\xA2dipfsX\"\x12 \xD6*\xD40L\xD0\xA8q\x11\x16\xA6\xFBn]\xA3\x1C\xF4\x9B\x02\x83MbTX\x13@\x88\x05^\x16\x07\x02dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static USEROVERRIDABLEDKIMREGISTRY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x07\xF1\xEA\xF5\x14a\x17\xA8W\x80c\x0BU\xB3|\x14a\x17\x8FW\x80c\x17\x94\xBB<\x14a\x15>W\x80c\"Z\x08\xD4\x14a\x15#W\x80c2\xE1\xE1\x94\x14a\x11:W\x80cK\xCB\xBE\x96\x14a\x10\xD7W\x80cL\x93\x06\x07\x14a\x0F\0W\x80cO\x1E\xF2\x86\x14a\x0B\xF8W\x80cR\xD1\x90-\x14a\x0BYW\x80cWI\0\xDD\x14a\x0B\x02W\x80caJD\x85\x14a\n\xE8W\x80cqP\x18\xA6\x14a\n,W\x80c}F6H\x14a\t\xFAW\x80c\x7F\x8E)\xBA\x14a\t\xD0W\x80c\x7F\xF1\x03\xDA\x14a\x06\tW\x80c\x81.\x12\xCE\x14a\x05\xECW\x80c\x82\xBF\xF8\xCD\x14a\x03\xD5W\x80c\x8D\xA5\xCB[\x14a\x03\x83W\x80c\xAD<\xB1\xCC\x14a\x03$W\x80c\xD5\x07\xC3 \x14a\x02\xF1W\x80c\xE3\x08\xDE\x0C\x14a\x02\x89W\x80c\xE7\xA7\x97z\x14a\x01\xA7W\x80c\xF0\xBF\xB1\x97\x14a\x01PWc\xF2\xFD\xE3\x8B\x14a\x01!W_\x80\xFD[4a\x01LW` `\x03\x196\x01\x12a\x01LWa\x01Ja\x01=a\x19-V[a\x01Ea*\tV[a\"\xF3V[\0[_\x80\xFD[4a\x01LW`@`\x03\x196\x01\x12a\x01LWa\x01ia\x19PV[`\x045_R`\x03` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x01LW`@`\x03\x196\x01\x12a\x01LW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01LWa\x01\xD8\x906\x90`\x04\x01a\x18\xE9V[`@Q\x7F\x8D\xA5\xCB[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x813Z\xFA\x90\x81\x15a\x02~W_\x91a\x02.W[` a\x02$\x83`$5\x86a\x1C\x1DV[`@Q\x90\x15\x15\x81R\xF3[\x90P` \x81=` \x11a\x02vW[\x81a\x02I` \x93\x83a\x17\xC3V[\x81\x01\x03\x12a\x01LWQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x01LW` a\x02\x15V[=\x91Pa\x02<V[`@Q=_\x82>=\x90\xFD[4a\x01LW` \x80a\x02\x9A6a\x19sV[\x93\x91`@\x93\x91\x93Q\x92\x81\x84\x92Q\x91\x82\x91\x01\x83^\x81\x01`\x02\x81R\x03\x01\x90 \x90_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x01LW_`\x03\x196\x01\x12a\x01LWa\x03 a\x03\x0Ca\x1A\x8DV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x18\xA6V[\x03\x90\xF3[4a\x01LW_`\x03\x196\x01\x12a\x01LWa\x03 `@Qa\x03E`@\x82a\x17\xC3V[`\x05\x81R\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x18\xA6V[4a\x01LW_`\x03\x196\x01\x12a\x01LW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x16`@Q\x90\x81R\xF3[4a\x01LWa\x03\xE36a\x1A\x01V[\x90a\x03\xF0\x84Q\x15\x15a\x1A\xC8V[a\x03\xFB\x83\x15\x15a\x1B-V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x94a\x04\x1F\x82\x15\x15a\x1B\x92V[\x84_R`\x03` R`@_ \x82_R` Ra\x04B`\xFF`@_ T\x16\x15a\x1F$V[\x813\x03a\x04\xB1W[P\x83\x80_R`\x03` R`@_ \x82_R` R`@_ `\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90U\x7F5P6\xB8\xAD\x96>\x18^\t\xF0t\xE8VU\x96H:\0\x12\xCB\xE6 \xF5\x07\xC0\xF3IP\xA2\xF0\xB3_\x80\xA3\0[a\x04\xD0a\x04\xD5\x91a\x04\xC0a\x1A\x8DV[\x90a\x04\xCA\x88a(KV[\x91a\x1EEV[a%\xE6V[\x91;\x15a\x05\xB1WPa\x05\x18\x91` \x91`@Q\x93\x84\x92\x83\x92\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01a\x1D\xC9V[\x03\x81\x86Z\xFA\x80\x15a\x02~W\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91a\x05y\x93_\x91a\x05\x82W[P\x16\x14a\x1D\xE0V[\x82\x80\x83\x81a\x04JV[a\x05\xA4\x91P` =` \x11a\x05\xAAW[a\x05\x9C\x81\x83a\x17\xC3V[\x81\x01\x90a\x1D\x91V[\x86a\x05qV[P=a\x05\x92V[\x90a\x05\xE0a\x05\xD7a\x05\xE7\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93a*\xCCV[\x90\x92\x91\x92a+\x06V[\x16\x14a\x1D,V[a\x05yV[4a\x01LW_`\x03\x196\x01\x12a\x01LW` `\x01T`@Q\x90\x81R\xF3[4a\x01LW`\x80`\x03\x196\x01\x12a\x01LW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01LW6`#\x82\x01\x12\x15a\x01LW\x80`\x04\x015a\x06E\x81a\x1AuV[\x91a\x06S`@Q\x93\x84a\x17\xC3V[\x81\x83R`$` \x84\x01\x92`\x05\x1B\x82\x01\x01\x906\x82\x11a\x01LW`$\x81\x01\x92[\x82\x84\x10a\t\xA0W\x84`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01LW6`#\x82\x01\x12\x15a\x01LW\x80`\x04\x015\x90a\x06\xA5\x82a\x1AuV[\x91a\x06\xB3`@Q\x93\x84a\x17\xC3V[\x80\x83R`$` \x84\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x01LW`$\x01\x90[\x82\x82\x10a\t\x90WPPP`D5\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x01LW6`#\x84\x01\x12\x15a\x01LW\x82`\x04\x015a\x07\x06\x81a\x1AuV[\x93a\x07\x14`@Q\x95\x86a\x17\xC3V[\x81\x85R`$` \x86\x01\x92`\x05\x1B\x82\x01\x01\x906\x82\x11a\x01LW`$\x01\x91[\x81\x83\x10a\tcWPPP`d5\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x01LW6`#\x84\x01\x12\x15a\x01LW\x82`\x04\x015a\x07g\x81a\x1AuV[\x93a\x07u`@Q\x95\x86a\x17\xC3V[\x81\x85R`$` \x86\x01\x92`\x05\x1B\x82\x01\x01\x906\x82\x11a\x01LW`$\x81\x01\x92[\x82\x84\x10a\t3WPPPP\x81Q\x81Q\x03a\x08\xD5W\x81Q\x84Q\x03a\x08wW\x81Q\x83Q\x03a\x08\x19W_[\x82Q\x81\x10\x15a\x01JW\x80a\x08\x13a\x07\xD4`\x01\x93\x86a\"\xB2V[Qa\x07\xDF\x83\x86a\"\xB2V[Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x07\xFF\x85\x8Ba\"\xB2V[Q\x16\x90a\x08\x0C\x85\x8Aa\"\xB2V[Q\x92a\x1F\xAFV[\x01a\x07\xBBV[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Finvalid signatures length\0\0\0\0\0\0\0`D\x82\x01R\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Finvalid authorizers length\0\0\0\0\0\0`D\x82\x01R\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Finvalid publicKeyHashes length\0\0`D\x82\x01R\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01LW` \x91a\tX\x83\x92`$6\x91\x87\x01\x01a\x18\xE9V[\x81R\x01\x93\x01\x92a\x07\x93V[\x825s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x01LW\x81R` \x92\x83\x01\x92\x01a\x071V[\x815\x81R` \x91\x82\x01\x91\x01a\x06\xD0V[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01LW` \x91a\t\xC5\x83\x92`$6\x91\x87\x01\x01a\x18\xE9V[\x81R\x01\x93\x01\x92a\x06qV[4a\x01LW` `\x03\x196\x01\x12a\x01LW`\x045_R`\x05` R` `@_ T`@Q\x90\x81R\xF3[4a\x01LW_`\x03\x196\x01\x12a\x01LW` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x16`@Q\x90\x81R\xF3[4a\x01LW_`\x03\x196\x01\x12a\x01LWa\nDa*\tV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x01LWa\x01Ja\n\xF96a\x1A\x01V[\x92\x91\x90\x91a\x1F\xAFV[4a\x01LW`@`\x03\x196\x01\x12a\x01LWa\x0B\x1Ba\x19PV[`\x045_R`\x04` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x01LW_`\x03\x196\x01\x12a\x01LWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x0B\xD0W` `@Q\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81R\xF3[\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`@`\x03\x196\x01\x12a\x01LWa\x0C\x0Ca\x19-V[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01LWa\x0C,\x906\x90`\x04\x01a\x18\xE9V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x800\x14\x90\x81\x15a\x0E\xBEW[Pa\x0B\xD0Wa\x0C{a*\tV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x91`@Q\x7FR\xD1\x90-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` \x81`\x04\x81\x87Z\xFA_\x91\x81a\x0E\x8AW[Pa\x0C\xFBW\x83\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x80\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x85\x92\x03a\x0E_WP\x81;\x15a\x0E4W\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16\x17\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCU\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;_\x80\xA2\x81Q\x15a\x0E\x03W_\x80\x83` a\x01J\x95Q\x91\x01\x84Z\xF4=\x15a\r\xFBW=\x91a\r\xDF\x83a\x181V[\x92a\r\xED`@Q\x94\x85a\x17\xC3V[\x83R=_` \x85\x01>a,mV[``\x91a,mV[PP4a\x0E\x0CW\0[\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x90\x91P` \x81=` \x11a\x0E\xB6W[\x81a\x0E\xA6` \x93\x83a\x17\xC3V[\x81\x01\x03\x12a\x01LWQ\x90\x85a\x0C\xCAV[=\x91Pa\x0E\x99V[\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16\x14\x15\x83a\x0CnV[4a\x01LW` `\x03\x196\x01\x12a\x01LWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa\x0F.a\x19-V[a\x0F6a*\tV[\x16\x80\x15a\x10SW_Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x82\x14a\x0F\xA9W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x91\x16\x17_U\x7F;\xB1\x96\x11\xD1\x15f1\xA8\\Y\xDD\xFEvhT\x1A/\0\xE6\xBA+~q\xCB\x0C`\xEC\xE0\xD5\xE5[_\x80\xA2\0[`\xA4`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FnewMainAuthorizer address cannot`D\x82\x01R\x7F be the same as the current main`d\x82\x01R\x7FAuthorizer\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FnewMainAuthorizer address cannot`D\x82\x01R\x7F be zero\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[4a\x01LW```\x03\x196\x01\x12a\x01LW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01LWa\x11\x08\x906\x90`\x04\x01a\x18\xE9V[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01LWa\x03 \x91a\x11/a\x03\x0C\x926\x90`\x04\x01a\x18\xE9V[a\x04\xCA`D5a(KV[4a\x01LWa\x11H6a\x1A\x01V[\x90a\x11U\x84Q\x15\x15a\x1A\xC8V[a\x11`\x83\x15\x15a\x1B-V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x93a\x11\x83\x85\x15\x15a\x1B\x92V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x16\x85\x14a\x14\x9FW\x83_R`\x04` R`@_ \x85_R` R`\xFF`@_ T\x16a\x14\x1BW`\x01a\x11\xCB\x83\x86a#\xE0V[\x03a\x13\xBDW`\x02a\x11\xDD\x83\x86\x84a$\xF3V[\x10a\x139W\x84\x90\x813\x03a\x12SW[P\x83\x80_R`\x04` R`@_ \x82_R` R`@_ `\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90U\x7F2\x89\x9A\x1E\xA4\xD8\xE4\x91|k=l\x1Ci\xFDLp\x949C\xB0'\xFE\x9D\x83+ R\xE7\xEF\xF8\xD6_\x80\xA3\0[a\x04\xD0a\x12b\x91a\x04\xC0a\x19\xC6V[\x91;\x15a\x13\x0EWPa\x12\xA5\x91` \x91`@Q\x93\x84\x92\x83\x92\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01a\x1D\xC9V[\x03\x81\x86Z\xFA\x80\x15a\x02~W\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91a\x13\x05\x93_\x91a\x05\x82WP\x16\x14a\x1D\xE0V[\x82\x80\x83\x81a\x11\xECV[\x90a\x05\xE0a\x05\xD7a\x134\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93a*\xCCV[a\x13\x05V[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7Fset threshold must be larger tha`D\x82\x01R\x7Fn two\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Frevoke threshold must be one\0\0\0\0`D\x82\x01R\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7Fpublic key hash is already react`D\x82\x01R\x7Fivated\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FmainAuthorizer cannot reactivate`D\x82\x01R\x7F the public key hash\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[4a\x01LW_`\x03\x196\x01\x12a\x01LWa\x03 a\x03\x0Ca\x19\xC6V[4a\x01LW```\x03\x196\x01\x12a\x01LWa\x15Wa\x19-V[a\x15_a\x19PV[\x90\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x91`\xFF\x83`@\x1C\x16\x15\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x15\x90\x81a\x17\x87W[`\x01\x14\x90\x81a\x17}W[\x15\x90\x81a\x17tW[Pa\x17LWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x81\x85`\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0a\x160\x95\x16\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0Ua\x16\xF7W[Pa\x16(a*uV[a\x01Ea*uV[\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_T\x16\x17_U`D5`\x01Ua\x16dW\0[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T\x16\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2` `@Q`\x01\x81R\xA1\0[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x16h\x01\0\0\0\0\0\0\0\x01\x17\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0U\x85a\x16\x1FV[\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x90P\x15\x85a\x15\xB2V[0;\x15\x91Pa\x15\xAAV[\x85\x91Pa\x15\xA0V[4a\x01LW` a\x02$a\x17\xA26a\x19sV[\x91a\x1C\x1DV[4a\x01LW_`\x03\x196\x01\x12a\x01LWa\x03 a\x03\x0Ca\x18kV[\x90`\x1F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x18\x04W`@RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x18\x04W`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[`@Q\x90a\x18z`@\x83a\x17\xC3V[`\x04\x82R\x7FSET:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01RV[\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x81`\x1F\x82\x01\x12\x15a\x01LW` \x815\x91\x01a\x19\x03\x82a\x181V[\x92a\x19\x11`@Q\x94\x85a\x17\xC3V[\x82\x84R\x82\x82\x01\x11a\x01LW\x81_\x92` \x92\x83\x86\x017\x83\x01\x01R\x90V[`\x045\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01LWV[`$5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01LWV[```\x03\x19\x82\x01\x12a\x01LW`\x045\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01LWa\x19\x9E\x91`\x04\x01a\x18\xE9V[\x90`$5\x90`D5s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x01LW\x90V[`@Q\x90a\x19\xD5`@\x83a\x17\xC3V[`\x0B\x82R\x7FREACTIVATE:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01RV[\x90`\x80`\x03\x19\x83\x01\x12a\x01LW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01LW\x82a\x1A-\x91`\x04\x01a\x18\xE9V[\x91`$5\x91`D5s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x01LW\x91`d5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01LWa\x1Ar\x91`\x04\x01a\x18\xE9V[\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x18\x04W`\x05\x1B` \x01\x90V[`@Q\x90a\x1A\x9C`@\x83a\x17\xC3V[`\x07\x82R\x7FREVOKE:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01RV[\x15a\x1A\xCFWV[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Fdomain name cannot be zero\0\0\0\0\0\0`D\x82\x01R\xFD[\x15a\x1B4WV[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fpublic key hash cannot be zero\0\0`D\x82\x01R\xFD[\x15a\x1B\x99WV[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7Fauthorizer address cannot be zer`D\x82\x01R\x7Fo\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[\x91\x90a\x1C+\x83Q\x15\x15a\x1A\xC8V[a\x1C6\x81\x15\x15a\x1B-V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a\x1CX\x81\x15\x15a\x1B\x92V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x16\x14a\x1C\xA8W\x81\x81a\x1C\x86`\x01\x94a\x1C\x8C\x94a#\xE0V[\x94a$\xF3V[\x91\x10a\x1C\x97WP_\x90V[`\x02\x11\x15a\x1C\xA3W_\x90V[`\x01\x90V[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Fauthorizer cannot be mainAuthori`D\x82\x01R\x7Fzer\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[\x15a\x1D3WV[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Finvalid ecdsa signature\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x90\x81` \x91\x03\x12a\x01LWQ\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x03a\x01LW\x90V[`@\x90a\x1Ar\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x18\xA6V[\x15a\x1D\xE7WV[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7Finvalid eip1271 signature\0\0\0\0\0\0\0`D\x82\x01R\xFD[`\x11`\x01\x91`\x07` \x80\x97\x96\x81\x80a\x1F\"\x98`@Q\x9B\x87\x8D\x98Q\x91\x82\x91\x01\x84\x8A\x01^\x87\x01\x90\x7Fdomain=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x83\x01R\x80Q\x92\x83\x91\x01`'\x83\x01^\x01\x01\x90\x7F;public_key_hash=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x83\x01R\x80Q\x92\x83\x91\x01`\x18\x83\x01^\x01\x01\x7F;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x82\x01R\x03\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x81\x01\x85R\x01\x83a\x17\xC3V[V[\x15a\x1F+WV[`\x84`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Fpublic key hash is already revok`D\x82\x01R\x7Fed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R\xFD[\x91\x90\x91a\x1F\xBE\x81Q\x15\x15a\x1A\xC8V[a\x1F\xC9\x83\x15\x15a\x1B-V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x93a\x1F\xEC\x85\x15\x15a\x1B\x92V[`@Q\x92\x82Q\x93` \x81\x81\x86\x01\x96\x80\x88\x83^\x81\x01`\x02\x81R\x03\x01\x90 \x85_R` R`@_ \x86_R` R`\xFF`@_ T\x16a\"TW\x84_R`\x03` R`@_ \x86_R` Ra G`\xFF`@_ T\x16\x15a\x1F$V[\x853\x03a!GW[PP`@Q` \x81\x83Q\x80\x86\x83^\x81\x01`\x02\x81R\x03\x01\x90 \x83_R` R`@_ \x84_R` R`@_ `\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_T\x16\x84\x14a \xFCW[`@Q\x91\x82\x91Q\x80\x91\x83^\x81\x01_\x81R\x03\x90 \x7F}a~\xDC\x9D\n\xDE/\xB7hC\xEF_sr\xBD'0\xE9\0\xFA\x12\xE6t\xBE\xCA\xA8\xAD\x01\xEA\xB6\xCB_\x80\xA4V[`\x01TB\x01\x80B\x11a!\x1AW\x83_R`\x05` R`@_ Ua \xC3V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x85a!`a\x04\xD0a!Va\x18kV[\x86a\x04\xCA\x8Aa(KV[\x91;\x15a\")WPa!\xA3\x91` \x91`@Q\x93\x84\x92\x83\x92\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01a\x1D\xC9V[\x03\x81\x88Z\xFA\x80\x15a\x02~W\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\x16&\xBA~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91a\"\x03\x93_\x91a\"\nWP\x16\x14a\x1D\xE0V[_\x80a OV[a\"#\x91P` =` \x11a\x05\xAAWa\x05\x9C\x81\x83a\x17\xC3V[_a\x05qV[\x90a\x05\xE0a\x05\xD7a\"O\x94s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93a*\xCCV[a\"\x03V[`d`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fpublic key hash is already set\0\0`D\x82\x01R\xFD[\x80Q\x82\x10\x15a\"\xC6W` \x91`\x05\x1B\x01\x01\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15a#\xB4Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x17\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0U\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R_`\x04R`$_\xFD[_\x91\x81_R`\x03` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80_T\x16\x16_R` R`\x01`\xFF`@_ T\x16\x15\x15\x14a$\xEAW[\x81_R`\x03` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_R` R`\x01`\xFF`@_ T\x16\x15\x15\x14a$\xD8W[`\x01\x83\x14\x91\x82a$\x9AW[PPa$mW\x90V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x01\x90\x81\x11a!\x1AW\x90V[\x90\x91P_R`\x04` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R`\x01`\xFF`@_ T\x16\x15\x15\x14_\x80a$dV[\x91`\x02\x81\x01\x80\x91\x11a!\x1AW\x91a$YV[`\x01\x92Pa$\x1FV[\x91\x90_\x92`@Q\x90\x80Q\x91` \x81\x81\x84\x01\x94\x80\x86\x83^\x81\x01`\x02\x81R\x03\x01\x90 \x83_R` R`@_ s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80_T\x16\x16_R` R`\x01`\xFF`@_ T\x16\x15\x15\x14a%\xAEW[` \x90`@Q\x92\x83\x91Q\x80\x91\x83^\x81\x01`\x02\x81R\x03\x01\x90 \x90_R` Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R`\x01`\xFF`@_ T\x16\x15\x15\x14a%\xA0W\x90V[`\x02\x81\x01\x80\x91\x11a!\x1AW\x90V[\x93\x82_R`\x05` R`@_ TB\x10_\x14a%\xDBW`\x01\x81\x01\x80\x91\x11a!\x1AW` \x90\x94[\x90Pa%NV[P` `\x02\x94a%\xD4V[\x80Q\x90_\x82z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x81\x10\x15a(#W[\x80m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0`\n\x92\x10\x15a(\x08W[f#\x86\xF2o\xC1\0\0\x81\x10\x15a'\xF4W[c\x05\xF5\xE1\0\x81\x10\x15a'\xE3W[a'\x10\x81\x10\x15a'\xD4W[`d\x81\x10\x15a'\xC6W[\x10\x15a'\xBEW[`\x01\x81\x01\x92`\n\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`!a&\xAEa&\x98\x88a\x181V[\x97a&\xA6`@Q\x99\x8Aa\x17\xC3V[\x80\x89Ra\x181V[\x94\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0` \x89\x01\x96\x016\x877\x87\x01\x01[\x01\x91\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x82\x06\x1A\x83S\x04\x90\x81\x15a'8W`\n\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90a&\xDDV[PPa'\xB8\x90`:` `@Q\x94\x85\x93\x82\x85\x01\x97\x7F\x19Ethereum Signed Message:\n\0\0\0\0\0\0\x89RQ\x80\x91\x85\x87\x01^\x84\x01\x90\x83\x82\x01_\x81R\x81Q\x93\x84\x92\x01\x90^\x01\x01_\x81R\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x01\x83R\x82a\x17\xC3V[Q\x90 \x90V[`\x01\x01a&bV[`d`\x02\x91\x04\x92\x01\x91a&[V[a'\x10`\x04\x91\x04\x92\x01\x91a&QV[c\x05\xF5\xE1\0`\x08\x91\x04\x92\x01\x91a&FV[f#\x86\xF2o\xC1\0\0`\x10\x91\x04\x92\x01\x91a&9V[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0` \x91\x04\x92\x01\x91a&)V[P`@\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x04a&\x0FV[`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x82\x81`\x07\x1B\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x90\x81`\x06\x1B\x1C\x90c\xFF\xFF\xFF\xFF\x82\x11\x91\x82`\x05\x1B\x1C\x92`\xFFa\xFF\xFF\x85\x11\x94\x85`\x04\x1B\x1C\x11\x93\x85\x1B\x92`\x02\x1B\x91`\x03\x1B\x90`\x04\x1B\x01\x01\x01\x01\x01\x81\x81`\x01\x1B\x92\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x83\x03a!\x1AW`\x02\x84\x01\x93\x84\x81\x11a!\x1AWa)\x05a(\xEF\x86a\x181V[\x95a(\xFD`@Q\x97\x88a\x17\xC3V[\x80\x87Ra\x181V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0` \x87\x01\x91\x016\x827\x85Q\x15a\"\xC6W`0\x90S\x84Q`\x01\x10\x15a\"\xC6W`x`!\x86\x01S`\x01\x81\x01\x80\x91\x11a!\x1AW[`\x01\x81\x11a)\x97WPa)iWPP\x90V[\x7F\xE2.'\xEB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$R`D_\xFD[\x90`\x0F\x81\x16`\x10\x81\x10\x15a\"\xC6W\x85Q\x83\x10\x15a\"\xC6W\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x1A\x85\x83\x01` \x01S`\x04\x1C\x90\x80\x15a!\x1AW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01a)WV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T\x163\x03a*IWV[\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R3`\x04R`$_\xFD[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15a*\xA4WV[\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x81Q\x91\x90`A\x83\x03a*\xFCWa*\xF5\x92P` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90a+\xDEV[\x91\x92\x90\x91\x90V[PP_\x91`\x02\x91\x90V[`\x04\x81\x10\x15a+\xB1W\x80a+\x18WPPV[`\x01\x81\x03a+HW\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[`\x02\x81\x03a+|WP\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[`\x03\x14a+\x86WPV[\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11a,bW\x91` \x93`\x80\x92`\xFF_\x95`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\x02~W_Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15a,XW\x90_\x90_\x90V[P_\x90`\x01\x90_\x90V[PPP_\x91`\x03\x91\x90V[\x90a,\xAAWP\x80Q\x15a,\x82W\x80Q\x90` \x01\xFD[\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x81Q\x15\x80a,\xFDW[a,\xBBWP\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R\x16`\x04R`$_\xFD[P\x80;\x15a,\xB3V\xFE\xA2dipfsX\"\x12 \xD6*\xD40L\xD0\xA8q\x11\x16\xA6\xFBn]\xA3\x1C\xF4\x9B\x02\x83MbTX\x13@\x88\x05^\x16\x07\x02dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static USEROVERRIDABLEDKIMREGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct UserOverridableDKIMRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for UserOverridableDKIMRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for UserOverridableDKIMRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for UserOverridableDKIMRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for UserOverridableDKIMRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(UserOverridableDKIMRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> UserOverridableDKIMRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    USEROVERRIDABLEDKIMREGISTRY_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                USEROVERRIDABLEDKIMREGISTRY_ABI.clone(),
                USEROVERRIDABLEDKIMREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `REACTIVATE_PREFIX` (0x225a08d4) function
        pub fn reactivate_prefix(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([34, 90, 8, 212], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `REVOKE_PREFIX` (0xd507c320) function
        pub fn revoke_prefix(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([213, 7, 195, 32], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `SET_PREFIX` (0x07f1eaf5) function
        pub fn set_prefix(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([7, 241, 234, 245], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `UPGRADE_INTERFACE_VERSION` (0xad3cb1cc) function
        pub fn upgrade_interface_version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([173, 60, 177, 204], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeMainAuthorizer` (0x4c930607) function
        pub fn change_main_authorizer(
            &self,
            new_main_authorizer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([76, 147, 6, 7], new_main_authorizer)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeSignedMsg` (0x4bcbbe96) function
        pub fn compute_signed_msg(
            &self,
            prefix: ::std::string::String,
            domain_name: ::std::string::String,
            public_key_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([75, 203, 190, 150], (prefix, domain_name, public_key_hash))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dkimPublicKeyHashes` (0xe308de0c) function
        pub fn dkim_public_key_hashes(
            &self,
            p0: ::std::string::String,
            p1: [u8; 32],
            p2: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([227, 8, 222, 12], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enabledTimeOfDKIMPublicKeyHash` (0x7f8e29ba) function
        pub fn enabled_time_of_dkim_public_key_hash(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([127, 142, 41, 186], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x1794bb3c) function
        pub fn initialize(
            &self,
            initial_owner: ::ethers::core::types::Address,
            main_authorizer: ::ethers::core::types::Address,
            set_timestamp_delay: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [23, 148, 187, 60],
                    (initial_owner, main_authorizer, set_timestamp_delay),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isDKIMPublicKeyHashValid` (0x0b55b37c) function
        pub fn is_dkim_public_key_hash_valid_with_domain_name_and_public_key_hash(
            &self,
            domain_name: ::std::string::String,
            public_key_hash: [u8; 32],
            authorizer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [11, 85, 179, 124],
                    (domain_name, public_key_hash, authorizer),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isDKIMPublicKeyHashValid` (0xe7a7977a) function
        pub fn is_dkim_public_key_hash_valid(
            &self,
            domain_name: ::std::string::String,
            public_key_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([231, 167, 151, 122], (domain_name, public_key_hash))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mainAuthorizer` (0x7d463648) function
        pub fn main_authorizer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([125, 70, 54, 72], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proxiableUUID` (0x52d1902d) function
        pub fn proxiable_uuid(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([82, 209, 144, 45], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reactivateDKIMPublicKeyHash` (0x32e1e194) function
        pub fn reactivate_dkim_public_key_hash(
            &self,
            domain_name: ::std::string::String,
            public_key_hash: [u8; 32],
            authorizer: ::ethers::core::types::Address,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [50, 225, 225, 148],
                    (domain_name, public_key_hash, authorizer, signature),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reactivatedDKIMPublicKeyHashes` (0x574900dd) function
        pub fn reactivated_dkim_public_key_hashes(
            &self,
            p0: [u8; 32],
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([87, 73, 0, 221], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeDKIMPublicKeyHash` (0x82bff8cd) function
        pub fn revoke_dkim_public_key_hash(
            &self,
            domain_name: ::std::string::String,
            public_key_hash: [u8; 32],
            authorizer: ::ethers::core::types::Address,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [130, 191, 248, 205],
                    (domain_name, public_key_hash, authorizer, signature),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokedDKIMPublicKeyHashes` (0xf0bfb197) function
        pub fn revoked_dkim_public_key_hashes(
            &self,
            p0: [u8; 32],
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([240, 191, 177, 151], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDKIMPublicKeyHash` (0x614a4485) function
        pub fn set_dkim_public_key_hash(
            &self,
            domain_name: ::std::string::String,
            public_key_hash: [u8; 32],
            authorizer: ::ethers::core::types::Address,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [97, 74, 68, 133],
                    (domain_name, public_key_hash, authorizer, signature),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDKIMPublicKeyHashes` (0x7ff103da) function
        pub fn set_dkim_public_key_hashes(
            &self,
            domain_names: ::std::vec::Vec<::std::string::String>,
            public_key_hashes: ::std::vec::Vec<[u8; 32]>,
            authorizers: ::std::vec::Vec<::ethers::core::types::Address>,
            signatures: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [127, 241, 3, 218],
                    (domain_names, public_key_hashes, authorizers, signatures),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTimestampDelay` (0x812e12ce) function
        pub fn set_timestamp_delay(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([129, 46, 18, 206], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgradeToAndCall` (0x4f1ef286) function
        pub fn upgrade_to_and_call(
            &self,
            new_implementation: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 30, 242, 134], (new_implementation, data))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DKIMPublicKeyHashReactivated` event
        pub fn dkim_public_key_hash_reactivated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DkimpublicKeyHashReactivatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DKIMPublicKeyHashRegistered` event
        pub fn dkim_public_key_hash_registered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DkimpublicKeyHashRegisteredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DKIMPublicKeyHashRevoked` event
        pub fn dkim_public_key_hash_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DkimpublicKeyHashRevokedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MainAuthorizerChanged` event
        pub fn main_authorizer_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MainAuthorizerChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Upgraded` event
        pub fn upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpgradedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UserOverridableDKIMRegistryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for UserOverridableDKIMRegistry<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AddressEmptyCode` with signature `AddressEmptyCode(address)` and selector `0x9996b315`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "AddressEmptyCode", abi = "AddressEmptyCode(address)")]
    pub struct AddressEmptyCode {
        pub target: ::ethers::core::types::Address,
    }
    ///Custom Error type `ECDSAInvalidSignature` with signature `ECDSAInvalidSignature()` and selector `0xf645eedf`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ECDSAInvalidSignature", abi = "ECDSAInvalidSignature()")]
    pub struct ECDSAInvalidSignature;
    ///Custom Error type `ECDSAInvalidSignatureLength` with signature `ECDSAInvalidSignatureLength(uint256)` and selector `0xfce698f7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "ECDSAInvalidSignatureLength",
        abi = "ECDSAInvalidSignatureLength(uint256)"
    )]
    pub struct ECDSAInvalidSignatureLength {
        pub length: ::ethers::core::types::U256,
    }
    ///Custom Error type `ECDSAInvalidSignatureS` with signature `ECDSAInvalidSignatureS(bytes32)` and selector `0xd78bce0c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ECDSAInvalidSignatureS", abi = "ECDSAInvalidSignatureS(bytes32)")]
    pub struct ECDSAInvalidSignatureS {
        pub s: [u8; 32],
    }
    ///Custom Error type `ERC1967InvalidImplementation` with signature `ERC1967InvalidImplementation(address)` and selector `0x4c9c8ce3`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "ERC1967InvalidImplementation",
        abi = "ERC1967InvalidImplementation(address)"
    )]
    pub struct ERC1967InvalidImplementation {
        pub implementation: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC1967NonPayable` with signature `ERC1967NonPayable()` and selector `0xb398979f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ERC1967NonPayable", abi = "ERC1967NonPayable()")]
    pub struct ERC1967NonPayable;
    ///Custom Error type `FailedCall` with signature `FailedCall()` and selector `0xd6bda275`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "FailedCall", abi = "FailedCall()")]
    pub struct FailedCall;
    ///Custom Error type `InvalidInitialization` with signature `InvalidInitialization()` and selector `0xf92ee8a9`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidInitialization", abi = "InvalidInitialization()")]
    pub struct InvalidInitialization;
    ///Custom Error type `NotInitializing` with signature `NotInitializing()` and selector `0xd7e6bcf8`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NotInitializing", abi = "NotInitializing()")]
    pub struct NotInitializing;
    ///Custom Error type `OwnableInvalidOwner` with signature `OwnableInvalidOwner(address)` and selector `0x1e4fbdf7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OwnableInvalidOwner", abi = "OwnableInvalidOwner(address)")]
    pub struct OwnableInvalidOwner {
        pub owner: ::ethers::core::types::Address,
    }
    ///Custom Error type `OwnableUnauthorizedAccount` with signature `OwnableUnauthorizedAccount(address)` and selector `0x118cdaa7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "OwnableUnauthorizedAccount",
        abi = "OwnableUnauthorizedAccount(address)"
    )]
    pub struct OwnableUnauthorizedAccount {
        pub account: ::ethers::core::types::Address,
    }
    ///Custom Error type `StringsInsufficientHexLength` with signature `StringsInsufficientHexLength(uint256,uint256)` and selector `0xe22e27eb`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "StringsInsufficientHexLength",
        abi = "StringsInsufficientHexLength(uint256,uint256)"
    )]
    pub struct StringsInsufficientHexLength {
        pub value: ::ethers::core::types::U256,
        pub length: ::ethers::core::types::U256,
    }
    ///Custom Error type `UUPSUnauthorizedCallContext` with signature `UUPSUnauthorizedCallContext()` and selector `0xe07c8dba`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "UUPSUnauthorizedCallContext",
        abi = "UUPSUnauthorizedCallContext()"
    )]
    pub struct UUPSUnauthorizedCallContext;
    ///Custom Error type `UUPSUnsupportedProxiableUUID` with signature `UUPSUnsupportedProxiableUUID(bytes32)` and selector `0xaa1d49a4`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "UUPSUnsupportedProxiableUUID",
        abi = "UUPSUnsupportedProxiableUUID(bytes32)"
    )]
    pub struct UUPSUnsupportedProxiableUUID {
        pub slot: [u8; 32],
    }
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum UserOverridableDKIMRegistryErrors {
        AddressEmptyCode(AddressEmptyCode),
        ECDSAInvalidSignature(ECDSAInvalidSignature),
        ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength),
        ECDSAInvalidSignatureS(ECDSAInvalidSignatureS),
        ERC1967InvalidImplementation(ERC1967InvalidImplementation),
        ERC1967NonPayable(ERC1967NonPayable),
        FailedCall(FailedCall),
        InvalidInitialization(InvalidInitialization),
        NotInitializing(NotInitializing),
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        StringsInsufficientHexLength(StringsInsufficientHexLength),
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for UserOverridableDKIMRegistryErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AddressEmptyCode as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressEmptyCode(decoded));
            }
            if let Ok(decoded) = <ECDSAInvalidSignature as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ECDSAInvalidSignature(decoded));
            }
            if let Ok(decoded) = <ECDSAInvalidSignatureLength as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ECDSAInvalidSignatureLength(decoded));
            }
            if let Ok(decoded) = <ECDSAInvalidSignatureS as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ECDSAInvalidSignatureS(decoded));
            }
            if let Ok(decoded) = <ERC1967InvalidImplementation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC1967InvalidImplementation(decoded));
            }
            if let Ok(decoded) = <ERC1967NonPayable as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC1967NonPayable(decoded));
            }
            if let Ok(decoded) = <FailedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedCall(decoded));
            }
            if let Ok(decoded) = <InvalidInitialization as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidInitialization(decoded));
            }
            if let Ok(decoded) = <NotInitializing as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotInitializing(decoded));
            }
            if let Ok(decoded) = <OwnableInvalidOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnableInvalidOwner(decoded));
            }
            if let Ok(decoded) = <OwnableUnauthorizedAccount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnableUnauthorizedAccount(decoded));
            }
            if let Ok(decoded) = <StringsInsufficientHexLength as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StringsInsufficientHexLength(decoded));
            }
            if let Ok(decoded) = <UUPSUnauthorizedCallContext as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UUPSUnauthorizedCallContext(decoded));
            }
            if let Ok(decoded) = <UUPSUnsupportedProxiableUUID as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UUPSUnsupportedProxiableUUID(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for UserOverridableDKIMRegistryErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AddressEmptyCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ECDSAInvalidSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ECDSAInvalidSignatureLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ECDSAInvalidSignatureS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1967InvalidImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1967NonPayable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidInitialization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotInitializing(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableInvalidOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StringsInsufficientHexLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UUPSUnauthorizedCallContext(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UUPSUnsupportedProxiableUUID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for UserOverridableDKIMRegistryErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AddressEmptyCode as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ECDSAInvalidSignature as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ECDSAInvalidSignatureLength as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ECDSAInvalidSignatureS as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC1967InvalidImplementation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC1967NonPayable as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FailedCall as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidInitialization as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotInitializing as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OwnableInvalidOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OwnableUnauthorizedAccount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <StringsInsufficientHexLength as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UUPSUnauthorizedCallContext as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UUPSUnsupportedProxiableUUID as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for UserOverridableDKIMRegistryErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::ECDSAInvalidSignature(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ECDSAInvalidSignatureLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ECDSAInvalidSignatureS(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC1967InvalidImplementation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC1967NonPayable(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidInitialization(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotInitializing(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnableInvalidOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StringsInsufficientHexLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UUPSUnauthorizedCallContext(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UUPSUnsupportedProxiableUUID(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String>
    for UserOverridableDKIMRegistryErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for UserOverridableDKIMRegistryErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignature>
    for UserOverridableDKIMRegistryErrors {
        fn from(value: ECDSAInvalidSignature) -> Self {
            Self::ECDSAInvalidSignature(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignatureLength>
    for UserOverridableDKIMRegistryErrors {
        fn from(value: ECDSAInvalidSignatureLength) -> Self {
            Self::ECDSAInvalidSignatureLength(value)
        }
    }
    impl ::core::convert::From<ECDSAInvalidSignatureS>
    for UserOverridableDKIMRegistryErrors {
        fn from(value: ECDSAInvalidSignatureS) -> Self {
            Self::ECDSAInvalidSignatureS(value)
        }
    }
    impl ::core::convert::From<ERC1967InvalidImplementation>
    for UserOverridableDKIMRegistryErrors {
        fn from(value: ERC1967InvalidImplementation) -> Self {
            Self::ERC1967InvalidImplementation(value)
        }
    }
    impl ::core::convert::From<ERC1967NonPayable> for UserOverridableDKIMRegistryErrors {
        fn from(value: ERC1967NonPayable) -> Self {
            Self::ERC1967NonPayable(value)
        }
    }
    impl ::core::convert::From<FailedCall> for UserOverridableDKIMRegistryErrors {
        fn from(value: FailedCall) -> Self {
            Self::FailedCall(value)
        }
    }
    impl ::core::convert::From<InvalidInitialization>
    for UserOverridableDKIMRegistryErrors {
        fn from(value: InvalidInitialization) -> Self {
            Self::InvalidInitialization(value)
        }
    }
    impl ::core::convert::From<NotInitializing> for UserOverridableDKIMRegistryErrors {
        fn from(value: NotInitializing) -> Self {
            Self::NotInitializing(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner>
    for UserOverridableDKIMRegistryErrors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount>
    for UserOverridableDKIMRegistryErrors {
        fn from(value: OwnableUnauthorizedAccount) -> Self {
            Self::OwnableUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<StringsInsufficientHexLength>
    for UserOverridableDKIMRegistryErrors {
        fn from(value: StringsInsufficientHexLength) -> Self {
            Self::StringsInsufficientHexLength(value)
        }
    }
    impl ::core::convert::From<UUPSUnauthorizedCallContext>
    for UserOverridableDKIMRegistryErrors {
        fn from(value: UUPSUnauthorizedCallContext) -> Self {
            Self::UUPSUnauthorizedCallContext(value)
        }
    }
    impl ::core::convert::From<UUPSUnsupportedProxiableUUID>
    for UserOverridableDKIMRegistryErrors {
        fn from(value: UUPSUnsupportedProxiableUUID) -> Self {
            Self::UUPSUnsupportedProxiableUUID(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "DKIMPublicKeyHashReactivated",
        abi = "DKIMPublicKeyHashReactivated(bytes32,address)"
    )]
    pub struct DkimpublicKeyHashReactivatedFilter {
        #[ethevent(indexed)]
        pub public_key_hash: [u8; 32],
        #[ethevent(indexed)]
        pub authorizer: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "DKIMPublicKeyHashRegistered",
        abi = "DKIMPublicKeyHashRegistered(string,bytes32,address)"
    )]
    pub struct DkimpublicKeyHashRegisteredFilter {
        #[ethevent(indexed)]
        pub domain_name: ::ethers::core::types::H256,
        #[ethevent(indexed)]
        pub public_key_hash: [u8; 32],
        #[ethevent(indexed)]
        pub authorizer: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "DKIMPublicKeyHashRevoked",
        abi = "DKIMPublicKeyHashRevoked(bytes32,address)"
    )]
    pub struct DkimpublicKeyHashRevokedFilter {
        #[ethevent(indexed)]
        pub public_key_hash: [u8; 32],
        #[ethevent(indexed)]
        pub authorizer: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Initialized", abi = "Initialized(uint64)")]
    pub struct InitializedFilter {
        pub version: u64,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "MainAuthorizerChanged", abi = "MainAuthorizerChanged(address)")]
    pub struct MainAuthorizerChangedFilter {
        #[ethevent(indexed)]
        pub new_main_authorizer: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum UserOverridableDKIMRegistryEvents {
        DkimpublicKeyHashReactivatedFilter(DkimpublicKeyHashReactivatedFilter),
        DkimpublicKeyHashRegisteredFilter(DkimpublicKeyHashRegisteredFilter),
        DkimpublicKeyHashRevokedFilter(DkimpublicKeyHashRevokedFilter),
        InitializedFilter(InitializedFilter),
        MainAuthorizerChangedFilter(MainAuthorizerChangedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for UserOverridableDKIMRegistryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DkimpublicKeyHashReactivatedFilter::decode_log(log) {
                return Ok(
                    UserOverridableDKIMRegistryEvents::DkimpublicKeyHashReactivatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = DkimpublicKeyHashRegisteredFilter::decode_log(log) {
                return Ok(
                    UserOverridableDKIMRegistryEvents::DkimpublicKeyHashRegisteredFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = DkimpublicKeyHashRevokedFilter::decode_log(log) {
                return Ok(
                    UserOverridableDKIMRegistryEvents::DkimpublicKeyHashRevokedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(UserOverridableDKIMRegistryEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = MainAuthorizerChangedFilter::decode_log(log) {
                return Ok(
                    UserOverridableDKIMRegistryEvents::MainAuthorizerChangedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(
                    UserOverridableDKIMRegistryEvents::OwnershipTransferredFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(UserOverridableDKIMRegistryEvents::UpgradedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for UserOverridableDKIMRegistryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DkimpublicKeyHashReactivatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DkimpublicKeyHashRegisteredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DkimpublicKeyHashRevokedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MainAuthorizerChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DkimpublicKeyHashReactivatedFilter>
    for UserOverridableDKIMRegistryEvents {
        fn from(value: DkimpublicKeyHashReactivatedFilter) -> Self {
            Self::DkimpublicKeyHashReactivatedFilter(value)
        }
    }
    impl ::core::convert::From<DkimpublicKeyHashRegisteredFilter>
    for UserOverridableDKIMRegistryEvents {
        fn from(value: DkimpublicKeyHashRegisteredFilter) -> Self {
            Self::DkimpublicKeyHashRegisteredFilter(value)
        }
    }
    impl ::core::convert::From<DkimpublicKeyHashRevokedFilter>
    for UserOverridableDKIMRegistryEvents {
        fn from(value: DkimpublicKeyHashRevokedFilter) -> Self {
            Self::DkimpublicKeyHashRevokedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for UserOverridableDKIMRegistryEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<MainAuthorizerChangedFilter>
    for UserOverridableDKIMRegistryEvents {
        fn from(value: MainAuthorizerChangedFilter) -> Self {
            Self::MainAuthorizerChangedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter>
    for UserOverridableDKIMRegistryEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for UserOverridableDKIMRegistryEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
    ///Container type for all input parameters for the `REACTIVATE_PREFIX` function with signature `REACTIVATE_PREFIX()` and selector `0x225a08d4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "REACTIVATE_PREFIX", abi = "REACTIVATE_PREFIX()")]
    pub struct ReactivatePrefixCall;
    ///Container type for all input parameters for the `REVOKE_PREFIX` function with signature `REVOKE_PREFIX()` and selector `0xd507c320`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "REVOKE_PREFIX", abi = "REVOKE_PREFIX()")]
    pub struct RevokePrefixCall;
    ///Container type for all input parameters for the `SET_PREFIX` function with signature `SET_PREFIX()` and selector `0x07f1eaf5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "SET_PREFIX", abi = "SET_PREFIX()")]
    pub struct SetPrefixCall;
    ///Container type for all input parameters for the `UPGRADE_INTERFACE_VERSION` function with signature `UPGRADE_INTERFACE_VERSION()` and selector `0xad3cb1cc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "UPGRADE_INTERFACE_VERSION", abi = "UPGRADE_INTERFACE_VERSION()")]
    pub struct UpgradeInterfaceVersionCall;
    ///Container type for all input parameters for the `changeMainAuthorizer` function with signature `changeMainAuthorizer(address)` and selector `0x4c930607`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "changeMainAuthorizer", abi = "changeMainAuthorizer(address)")]
    pub struct ChangeMainAuthorizerCall {
        pub new_main_authorizer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `computeSignedMsg` function with signature `computeSignedMsg(string,string,bytes32)` and selector `0x4bcbbe96`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "computeSignedMsg",
        abi = "computeSignedMsg(string,string,bytes32)"
    )]
    pub struct ComputeSignedMsgCall {
        pub prefix: ::std::string::String,
        pub domain_name: ::std::string::String,
        pub public_key_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `dkimPublicKeyHashes` function with signature `dkimPublicKeyHashes(string,bytes32,address)` and selector `0xe308de0c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "dkimPublicKeyHashes",
        abi = "dkimPublicKeyHashes(string,bytes32,address)"
    )]
    pub struct DkimPublicKeyHashesCall(
        pub ::std::string::String,
        pub [u8; 32],
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `enabledTimeOfDKIMPublicKeyHash` function with signature `enabledTimeOfDKIMPublicKeyHash(bytes32)` and selector `0x7f8e29ba`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "enabledTimeOfDKIMPublicKeyHash",
        abi = "enabledTimeOfDKIMPublicKeyHash(bytes32)"
    )]
    pub struct EnabledTimeOfDKIMPublicKeyHashCall(pub [u8; 32]);
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,uint256)` and selector `0x1794bb3c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "initialize", abi = "initialize(address,address,uint256)")]
    pub struct InitializeCall {
        pub initial_owner: ::ethers::core::types::Address,
        pub main_authorizer: ::ethers::core::types::Address,
        pub set_timestamp_delay: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isDKIMPublicKeyHashValid` function with signature `isDKIMPublicKeyHashValid(string,bytes32,address)` and selector `0x0b55b37c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "isDKIMPublicKeyHashValid",
        abi = "isDKIMPublicKeyHashValid(string,bytes32,address)"
    )]
    pub struct IsDkimPublicKeyHashValidWithDomainNameAndPublicKeyHashCall {
        pub domain_name: ::std::string::String,
        pub public_key_hash: [u8; 32],
        pub authorizer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isDKIMPublicKeyHashValid` function with signature `isDKIMPublicKeyHashValid(string,bytes32)` and selector `0xe7a7977a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "isDKIMPublicKeyHashValid",
        abi = "isDKIMPublicKeyHashValid(string,bytes32)"
    )]
    pub struct IsDKIMPublicKeyHashValidCall {
        pub domain_name: ::std::string::String,
        pub public_key_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `mainAuthorizer` function with signature `mainAuthorizer()` and selector `0x7d463648`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "mainAuthorizer", abi = "mainAuthorizer()")]
    pub struct MainAuthorizerCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `proxiableUUID` function with signature `proxiableUUID()` and selector `0x52d1902d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "proxiableUUID", abi = "proxiableUUID()")]
    pub struct ProxiableUUIDCall;
    ///Container type for all input parameters for the `reactivateDKIMPublicKeyHash` function with signature `reactivateDKIMPublicKeyHash(string,bytes32,address,bytes)` and selector `0x32e1e194`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "reactivateDKIMPublicKeyHash",
        abi = "reactivateDKIMPublicKeyHash(string,bytes32,address,bytes)"
    )]
    pub struct ReactivateDKIMPublicKeyHashCall {
        pub domain_name: ::std::string::String,
        pub public_key_hash: [u8; 32],
        pub authorizer: ::ethers::core::types::Address,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `reactivatedDKIMPublicKeyHashes` function with signature `reactivatedDKIMPublicKeyHashes(bytes32,address)` and selector `0x574900dd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "reactivatedDKIMPublicKeyHashes",
        abi = "reactivatedDKIMPublicKeyHashes(bytes32,address)"
    )]
    pub struct ReactivatedDKIMPublicKeyHashesCall(
        pub [u8; 32],
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `revokeDKIMPublicKeyHash` function with signature `revokeDKIMPublicKeyHash(string,bytes32,address,bytes)` and selector `0x82bff8cd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "revokeDKIMPublicKeyHash",
        abi = "revokeDKIMPublicKeyHash(string,bytes32,address,bytes)"
    )]
    pub struct RevokeDKIMPublicKeyHashCall {
        pub domain_name: ::std::string::String,
        pub public_key_hash: [u8; 32],
        pub authorizer: ::ethers::core::types::Address,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `revokedDKIMPublicKeyHashes` function with signature `revokedDKIMPublicKeyHashes(bytes32,address)` and selector `0xf0bfb197`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "revokedDKIMPublicKeyHashes",
        abi = "revokedDKIMPublicKeyHashes(bytes32,address)"
    )]
    pub struct RevokedDKIMPublicKeyHashesCall(
        pub [u8; 32],
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `setDKIMPublicKeyHash` function with signature `setDKIMPublicKeyHash(string,bytes32,address,bytes)` and selector `0x614a4485`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "setDKIMPublicKeyHash",
        abi = "setDKIMPublicKeyHash(string,bytes32,address,bytes)"
    )]
    pub struct SetDKIMPublicKeyHashCall {
        pub domain_name: ::std::string::String,
        pub public_key_hash: [u8; 32],
        pub authorizer: ::ethers::core::types::Address,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setDKIMPublicKeyHashes` function with signature `setDKIMPublicKeyHashes(string[],bytes32[],address[],bytes[])` and selector `0x7ff103da`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "setDKIMPublicKeyHashes",
        abi = "setDKIMPublicKeyHashes(string[],bytes32[],address[],bytes[])"
    )]
    pub struct SetDKIMPublicKeyHashesCall {
        pub domain_names: ::std::vec::Vec<::std::string::String>,
        pub public_key_hashes: ::std::vec::Vec<[u8; 32]>,
        pub authorizers: ::std::vec::Vec<::ethers::core::types::Address>,
        pub signatures: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `setTimestampDelay` function with signature `setTimestampDelay()` and selector `0x812e12ce`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setTimestampDelay", abi = "setTimestampDelay()")]
    pub struct SetTimestampDelayCall;
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `upgradeToAndCall` function with signature `upgradeToAndCall(address,bytes)` and selector `0x4f1ef286`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "upgradeToAndCall", abi = "upgradeToAndCall(address,bytes)")]
    pub struct UpgradeToAndCallCall {
        pub new_implementation: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum UserOverridableDKIMRegistryCalls {
        ReactivatePrefix(ReactivatePrefixCall),
        RevokePrefix(RevokePrefixCall),
        SetPrefix(SetPrefixCall),
        UpgradeInterfaceVersion(UpgradeInterfaceVersionCall),
        ChangeMainAuthorizer(ChangeMainAuthorizerCall),
        ComputeSignedMsg(ComputeSignedMsgCall),
        DkimPublicKeyHashes(DkimPublicKeyHashesCall),
        EnabledTimeOfDKIMPublicKeyHash(EnabledTimeOfDKIMPublicKeyHashCall),
        Initialize(InitializeCall),
        IsDkimPublicKeyHashValidWithDomainNameAndPublicKeyHash(
            IsDkimPublicKeyHashValidWithDomainNameAndPublicKeyHashCall,
        ),
        IsDKIMPublicKeyHashValid(IsDKIMPublicKeyHashValidCall),
        MainAuthorizer(MainAuthorizerCall),
        Owner(OwnerCall),
        ProxiableUUID(ProxiableUUIDCall),
        ReactivateDKIMPublicKeyHash(ReactivateDKIMPublicKeyHashCall),
        ReactivatedDKIMPublicKeyHashes(ReactivatedDKIMPublicKeyHashesCall),
        RenounceOwnership(RenounceOwnershipCall),
        RevokeDKIMPublicKeyHash(RevokeDKIMPublicKeyHashCall),
        RevokedDKIMPublicKeyHashes(RevokedDKIMPublicKeyHashesCall),
        SetDKIMPublicKeyHash(SetDKIMPublicKeyHashCall),
        SetDKIMPublicKeyHashes(SetDKIMPublicKeyHashesCall),
        SetTimestampDelay(SetTimestampDelayCall),
        TransferOwnership(TransferOwnershipCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
    }
    impl ::ethers::core::abi::AbiDecode for UserOverridableDKIMRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ReactivatePrefixCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReactivatePrefix(decoded));
            }
            if let Ok(decoded) = <RevokePrefixCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokePrefix(decoded));
            }
            if let Ok(decoded) = <SetPrefixCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPrefix(decoded));
            }
            if let Ok(decoded) = <UpgradeInterfaceVersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpgradeInterfaceVersion(decoded));
            }
            if let Ok(decoded) = <ChangeMainAuthorizerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChangeMainAuthorizer(decoded));
            }
            if let Ok(decoded) = <ComputeSignedMsgCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeSignedMsg(decoded));
            }
            if let Ok(decoded) = <DkimPublicKeyHashesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DkimPublicKeyHashes(decoded));
            }
            if let Ok(decoded) = <EnabledTimeOfDKIMPublicKeyHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnabledTimeOfDKIMPublicKeyHash(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <IsDkimPublicKeyHashValidWithDomainNameAndPublicKeyHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::IsDkimPublicKeyHashValidWithDomainNameAndPublicKeyHash(decoded),
                );
            }
            if let Ok(decoded) = <IsDKIMPublicKeyHashValidCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsDKIMPublicKeyHashValid(decoded));
            }
            if let Ok(decoded) = <MainAuthorizerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MainAuthorizer(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <ProxiableUUIDCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProxiableUUID(decoded));
            }
            if let Ok(decoded) = <ReactivateDKIMPublicKeyHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReactivateDKIMPublicKeyHash(decoded));
            }
            if let Ok(decoded) = <ReactivatedDKIMPublicKeyHashesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReactivatedDKIMPublicKeyHashes(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <RevokeDKIMPublicKeyHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokeDKIMPublicKeyHash(decoded));
            }
            if let Ok(decoded) = <RevokedDKIMPublicKeyHashesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokedDKIMPublicKeyHashes(decoded));
            }
            if let Ok(decoded) = <SetDKIMPublicKeyHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetDKIMPublicKeyHash(decoded));
            }
            if let Ok(decoded) = <SetDKIMPublicKeyHashesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetDKIMPublicKeyHashes(decoded));
            }
            if let Ok(decoded) = <SetTimestampDelayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetTimestampDelay(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UpgradeToAndCallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpgradeToAndCall(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for UserOverridableDKIMRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ReactivatePrefix(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokePrefix(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPrefix(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeInterfaceVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeMainAuthorizer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeSignedMsg(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DkimPublicKeyHashes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnabledTimeOfDKIMPublicKeyHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsDkimPublicKeyHashValidWithDomainNameAndPublicKeyHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsDKIMPublicKeyHashValid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MainAuthorizer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProxiableUUID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReactivateDKIMPublicKeyHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReactivatedDKIMPublicKeyHashes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeDKIMPublicKeyHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokedDKIMPublicKeyHashes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetDKIMPublicKeyHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetDKIMPublicKeyHashes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetTimestampDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeToAndCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for UserOverridableDKIMRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ReactivatePrefix(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokePrefix(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPrefix(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeInterfaceVersion(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangeMainAuthorizer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ComputeSignedMsg(element) => ::core::fmt::Display::fmt(element, f),
                Self::DkimPublicKeyHashes(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnabledTimeOfDKIMPublicKeyHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDkimPublicKeyHashValidWithDomainNameAndPublicKeyHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsDKIMPublicKeyHashValid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MainAuthorizer(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxiableUUID(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReactivateDKIMPublicKeyHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReactivatedDKIMPublicKeyHashes(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeDKIMPublicKeyHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevokedDKIMPublicKeyHashes(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetDKIMPublicKeyHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetDKIMPublicKeyHashes(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetTimestampDelay(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeToAndCall(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ReactivatePrefixCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: ReactivatePrefixCall) -> Self {
            Self::ReactivatePrefix(value)
        }
    }
    impl ::core::convert::From<RevokePrefixCall> for UserOverridableDKIMRegistryCalls {
        fn from(value: RevokePrefixCall) -> Self {
            Self::RevokePrefix(value)
        }
    }
    impl ::core::convert::From<SetPrefixCall> for UserOverridableDKIMRegistryCalls {
        fn from(value: SetPrefixCall) -> Self {
            Self::SetPrefix(value)
        }
    }
    impl ::core::convert::From<UpgradeInterfaceVersionCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: UpgradeInterfaceVersionCall) -> Self {
            Self::UpgradeInterfaceVersion(value)
        }
    }
    impl ::core::convert::From<ChangeMainAuthorizerCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: ChangeMainAuthorizerCall) -> Self {
            Self::ChangeMainAuthorizer(value)
        }
    }
    impl ::core::convert::From<ComputeSignedMsgCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: ComputeSignedMsgCall) -> Self {
            Self::ComputeSignedMsg(value)
        }
    }
    impl ::core::convert::From<DkimPublicKeyHashesCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: DkimPublicKeyHashesCall) -> Self {
            Self::DkimPublicKeyHashes(value)
        }
    }
    impl ::core::convert::From<EnabledTimeOfDKIMPublicKeyHashCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: EnabledTimeOfDKIMPublicKeyHashCall) -> Self {
            Self::EnabledTimeOfDKIMPublicKeyHash(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for UserOverridableDKIMRegistryCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<
        IsDkimPublicKeyHashValidWithDomainNameAndPublicKeyHashCall,
    > for UserOverridableDKIMRegistryCalls {
        fn from(
            value: IsDkimPublicKeyHashValidWithDomainNameAndPublicKeyHashCall,
        ) -> Self {
            Self::IsDkimPublicKeyHashValidWithDomainNameAndPublicKeyHash(value)
        }
    }
    impl ::core::convert::From<IsDKIMPublicKeyHashValidCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: IsDKIMPublicKeyHashValidCall) -> Self {
            Self::IsDKIMPublicKeyHashValid(value)
        }
    }
    impl ::core::convert::From<MainAuthorizerCall> for UserOverridableDKIMRegistryCalls {
        fn from(value: MainAuthorizerCall) -> Self {
            Self::MainAuthorizer(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for UserOverridableDKIMRegistryCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<ProxiableUUIDCall> for UserOverridableDKIMRegistryCalls {
        fn from(value: ProxiableUUIDCall) -> Self {
            Self::ProxiableUUID(value)
        }
    }
    impl ::core::convert::From<ReactivateDKIMPublicKeyHashCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: ReactivateDKIMPublicKeyHashCall) -> Self {
            Self::ReactivateDKIMPublicKeyHash(value)
        }
    }
    impl ::core::convert::From<ReactivatedDKIMPublicKeyHashesCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: ReactivatedDKIMPublicKeyHashesCall) -> Self {
            Self::ReactivatedDKIMPublicKeyHashes(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<RevokeDKIMPublicKeyHashCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: RevokeDKIMPublicKeyHashCall) -> Self {
            Self::RevokeDKIMPublicKeyHash(value)
        }
    }
    impl ::core::convert::From<RevokedDKIMPublicKeyHashesCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: RevokedDKIMPublicKeyHashesCall) -> Self {
            Self::RevokedDKIMPublicKeyHashes(value)
        }
    }
    impl ::core::convert::From<SetDKIMPublicKeyHashCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: SetDKIMPublicKeyHashCall) -> Self {
            Self::SetDKIMPublicKeyHash(value)
        }
    }
    impl ::core::convert::From<SetDKIMPublicKeyHashesCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: SetDKIMPublicKeyHashesCall) -> Self {
            Self::SetDKIMPublicKeyHashes(value)
        }
    }
    impl ::core::convert::From<SetTimestampDelayCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: SetTimestampDelayCall) -> Self {
            Self::SetTimestampDelay(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpgradeToAndCallCall>
    for UserOverridableDKIMRegistryCalls {
        fn from(value: UpgradeToAndCallCall) -> Self {
            Self::UpgradeToAndCall(value)
        }
    }
    ///Container type for all return fields from the `REACTIVATE_PREFIX` function with signature `REACTIVATE_PREFIX()` and selector `0x225a08d4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ReactivatePrefixReturn(pub ::std::string::String);
    ///Container type for all return fields from the `REVOKE_PREFIX` function with signature `REVOKE_PREFIX()` and selector `0xd507c320`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RevokePrefixReturn(pub ::std::string::String);
    ///Container type for all return fields from the `SET_PREFIX` function with signature `SET_PREFIX()` and selector `0x07f1eaf5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SetPrefixReturn(pub ::std::string::String);
    ///Container type for all return fields from the `UPGRADE_INTERFACE_VERSION` function with signature `UPGRADE_INTERFACE_VERSION()` and selector `0xad3cb1cc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct UpgradeInterfaceVersionReturn(pub ::std::string::String);
    ///Container type for all return fields from the `computeSignedMsg` function with signature `computeSignedMsg(string,string,bytes32)` and selector `0x4bcbbe96`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ComputeSignedMsgReturn(pub ::std::string::String);
    ///Container type for all return fields from the `dkimPublicKeyHashes` function with signature `dkimPublicKeyHashes(string,bytes32,address)` and selector `0xe308de0c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DkimPublicKeyHashesReturn(pub bool);
    ///Container type for all return fields from the `enabledTimeOfDKIMPublicKeyHash` function with signature `enabledTimeOfDKIMPublicKeyHash(bytes32)` and selector `0x7f8e29ba`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnabledTimeOfDKIMPublicKeyHashReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `isDKIMPublicKeyHashValid` function with signature `isDKIMPublicKeyHashValid(string,bytes32,address)` and selector `0x0b55b37c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsDkimPublicKeyHashValidWithDomainNameAndPublicKeyHashReturn(pub bool);
    ///Container type for all return fields from the `isDKIMPublicKeyHashValid` function with signature `isDKIMPublicKeyHashValid(string,bytes32)` and selector `0xe7a7977a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsDKIMPublicKeyHashValidReturn(pub bool);
    ///Container type for all return fields from the `mainAuthorizer` function with signature `mainAuthorizer()` and selector `0x7d463648`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MainAuthorizerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `proxiableUUID` function with signature `proxiableUUID()` and selector `0x52d1902d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ProxiableUUIDReturn(pub [u8; 32]);
    ///Container type for all return fields from the `reactivatedDKIMPublicKeyHashes` function with signature `reactivatedDKIMPublicKeyHashes(bytes32,address)` and selector `0x574900dd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ReactivatedDKIMPublicKeyHashesReturn(pub bool);
    ///Container type for all return fields from the `revokedDKIMPublicKeyHashes` function with signature `revokedDKIMPublicKeyHashes(bytes32,address)` and selector `0xf0bfb197`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RevokedDKIMPublicKeyHashesReturn(pub bool);
    ///Container type for all return fields from the `setTimestampDelay` function with signature `setTimestampDelay()` and selector `0x812e12ce`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SetTimestampDelayReturn(pub ::ethers::core::types::U256);
}
