// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate chain configurations.

use grandpa_primitives::AuthorityId as GrandpaId;
use node_template_runtime::{
	MILLICENTS, CENTS, DOLLARS, deposit, wasm_binary_unwrap, AuthorityDiscoveryConfig, BabeConfig,
	BalancesConfig, Block, CouncilConfig, ElectionsConfig, GrandpaConfig,
	ImOnlineConfig, MaxNominations, NominationPoolsConfig, SessionConfig,
	opaque::SessionKeys, StakerStatus, StakingConfig, SudoConfig, SystemConfig,
	TechnicalCommitteeConfig,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::ChainSpecExtension;
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};

pub use node_template_runtime::GenesisConfig;
pub use crate::node_primitives::{AccountId, Balance, Signature};

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
	/// The light sync state extension used by the sync-state rpc.
	pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;
/// Flaming Fir testnet generator
// pub fn flaming_fir_config() -> Result<ChainSpec, String> {
// 	ChainSpec::from_json_bytes(&include_bytes!("../res/flaming-fir.json")[..])
// }

fn session_keys(
	grandpa: GrandpaId,
	babe: BabeId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { grandpa, babe, im_online, authority_discovery }
}

fn staging_testnet_config_genesis() -> GenesisConfig {
	#[rustfmt::skip]
	// stash, controller, session-key
	// generated with secret:
	// for i in 1 2 3 4 ; do for j in stash controller; do subkey inspect "$secret"/fir/$j/$i; done; done
	//
	// and
	//
	// for i in 1 2 3 4 ; do for j in session; do subkey --ed25519 inspect "$secret"//fir//$j//$i; done; done

	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)> = vec![
		(
			// 5DiGN3GxM9LxBXNGTBAchKXG9EEYXJ2jTKAS2cdVyg5VzD4b
			array_bytes::hex_n_into_unchecked("48e1bd5e0d5b206b6b5205a38e143a608f58c3adb2350d0a6d8f840dabf3d109"),
			// 5ED5tKkWQ3F6b6LUwhgEq21QF1hJng4sD5ruMQyJtJw5jSop
			array_bytes::hex_n_into_unchecked("5edc95b6a28b06c4e525e9df8cb8cd16f9901c72c320344af67911fbe4c79171"),
			// 5FSS8JYr3s5v8sAcXbngyuuPQ7MjmEDfrvyqwtd23E5tYm4Z
			array_bytes::hex2array_unchecked("954725588edbe2424653adc35c51e3196cedd3449cea9fd583aa5866a9056848")
				.unchecked_into(),

			// 5F7PVpzX2Pm6bLHBsmxqqpCpxYcybBoM4BZRrfUujZWtcges
			array_bytes::hex2array_unchecked("86c0a2335342fca26b195116e401dc8cdd4d0412651e00d216cf76fffe50c810")
				.unchecked_into(),

			// 5GeYjUpB17VQc1vx31xPdteB9rV5u2ByHos4i6NFmVm4PZmH
			array_bytes::hex2array_unchecked("cac0904b34d7ad1366f19b8accadbaba2a7c1ef461bd2a82c149511db9ec8040")
				.unchecked_into(),

			// 5FAJT5HUdSdvrABBBnBi3BTKHP4WFNqp4dvUspWoB2m9vmkY
			array_bytes::hex2array_unchecked("88f960e4d2ba1c165573230528388c6371871451e5d69b3aae191604c7284073")
				.unchecked_into(),
		),
		(
			// 5EATukbozXbq3viTK5vuLxri4yhRUy5mGV7evFjZcS8PGTzu
			array_bytes::hex_n_into_unchecked("5cdcfd7810e7d75358d9ab93141cc2ff96403fafa9eddbb300acf5ddda83253a"),
			// 5G9BudKJWiQHKSN2m2p4rJFWhMZES8gWxW75EkvPcmRoyzA8
			array_bytes::hex_n_into_unchecked("b45c57f132086c56df5333dcb84c33e99adefcc1d00924be5977f83b1fc9fc4c"),

			// 5Eb8Coyhpa9iCkGbYvHXuXhY9zPjdKfrYqPtY7n3mVT8HW2T
			array_bytes::hex2array_unchecked("6fabca0de9bd264db20116293bf623f47d43de010b3fd891fc615cd94f7799cb")
				.unchecked_into(),

			// 5GBRT4HR9guYic7PbnRZhsYV1XG9a7apDT3ioA8DXz2gwb1Q
			array_bytes::hex2array_unchecked("b6106ba2761a95e1d6e4e4ff1330c8b7cccf362c7551ef2c8c72aaf77014ec61")
				.unchecked_into(),

			// 5C7WADCp3zECYE6VwvDhuCrnbBSVaV7YmDeAXkLH7bv9mfqK
			array_bytes::hex2array_unchecked("02225bd86f0118acff64dadd802c8df5fd64201a9db252bcde8a80b7fcb8ec0b")
				.unchecked_into(),

			// 5DAPT1d2wmcRWEKftTXGhrtXYMzYYdBYUgojswfPvnHVuFMg
			array_bytes::hex2array_unchecked("3091bd4aac4950ccdae7e54ee70655529c463d000429edac3afb2384235ec72d")
				.unchecked_into(),
		),
		(
			// 5HanQqGaNc1tHgT9JT4VPWbZV5zdtyvWWWAKZ7zQBzJNac14
			array_bytes::hex_n_into_unchecked("f41ddc84b3db1bda94c73c7ef5ceb498beafbfb18b535fc9da9fd497d22d7f3a"),
			// 5GE3J1sQmaHfYiNbhrUfWx83Y65LyGiUhJtf7zKbfMP1rYt7
			array_bytes::hex_n_into_unchecked("b80f92c2ea7145cd4e37bc9ac7dd9684d966388beb695568760134b96c4b0731"),

			// 5H2UxoFFYtwXTYpEWc7Ww5P91dW2vkU9cPdZc9aSGGqYiR2J
			array_bytes::hex2array_unchecked("db7b4532a0378b1fe452bfa6c5a0eb45ad7255ce679bb84e60daa9faf48230c5")
				.unchecked_into(),

			// 5HN3W7ssoe47CmU89rtAqpKkTpoqFzaDT8PqrqJt86STTP3g
			array_bytes::hex2array_unchecked("ea6678f34ea9be651bf0e1ae637ec4574d7a26ee640c607cf15d0388b5641018")
				.unchecked_into(),

			// 5Fo21wJRZEELqyM2CQTsuivT9f74dRz223YzmXbHnpRncXFW
			array_bytes::hex2array_unchecked("a4fa237ff4b59966b6a27d43370f3b6581d89f86ec932e7449bbfdfff5cd4e61")
				.unchecked_into(),

			// 5Ck9NPuakr8CCuDfRSZSAAhsWBAecgKQe8NQLuhVqiYEPZPr
			array_bytes::hex2array_unchecked("1e14742041e508d7ec9801e61682f53930c4d80eee946f7a3347004c75bbad71")
				.unchecked_into(),
		),
		(
			// 5DAYhWj4z7qdWiiSEqdwYL6A9H8jE99KYy17XNwn9zb3BLzR
			array_bytes::hex_n_into_unchecked("30b0e0b788718ca1d4f55366548f6e84639a7772b704b6c913c1ce4f5b2e6014"),
			// 5DZc11UovP2CEBQyGcfGU1wJms14xb7WYspaNbty23sJWpy1
			array_bytes::hex_n_into_unchecked("4246a2ee5253622eba8044532d3861b927ccfb58db60e9755de1644974667047"),

			// 5EETX3onKHPat4ug6kbasmwZDJqhznBnR96W9tXuMSJncZTE
			array_bytes::hex2array_unchecked("5fe8a76ef50dba942242a29476bccb19f4c6bde8d6820331bbeebce2edc42833")
				.unchecked_into(),

			// 5Hq2kJ36Zd5yDcGpRyCqjAawBytrVzxTwckpiCEFGCsiWyf4
			array_bytes::hex2array_unchecked("fefb8e07765fa92bbe89b629529378f581a4d5efd5b41239028a57697fbdbd7b")
				.unchecked_into(),

			// 5CAoSpktMsqggkL1G4yo8BkbVg3XS4Ro9sg4QXWphfSkXrQP
			array_bytes::hex2array_unchecked("04a6494aca156c5aec62f6ac329b2a77623764f409f3a20ef3b9e07572264633")
				.unchecked_into(),

			// 5FWW7FdCSGNvcmxRhJNSFmaFKDHMATiqagcX2MKsub2wBopz
			array_bytes::hex2array_unchecked("9861881fb2644072055f61f65e307c3571d707499a63a9e3a7f321ed1759074c")
				.unchecked_into(),
		),
	];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = array_bytes::hex_n_into_unchecked(
		// 5D5T7mdTV5kCqSy8wift4VEy3eUmK8LUJGbr7x9n6ka8cDZB
		"2ccddc603bf0436482e1f1f30c2a3d2752cac053132e7969fa3a43caa075b26b",
	);

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

	testnet_genesis(initial_authorities, vec![], root_key, Some(endowed_accounts))
}

/// Staging testnet config.
pub fn staging_testnet_config() -> ChainSpec {
	let boot_nodes = vec![];
	ChainSpec::from_genesis(
		"Staging Testnet",
		"staging_testnet",
		ChainType::Live,
		staging_testnet_config_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		None,
		None,
		None,
		Default::default(),
	)
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(
	seed: &str,
) -> (AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	initial_nominators: Vec<AccountId>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> GenesisConfig {
	let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
			get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
			get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
		]
	});
	// endow all authorities and nominators.
	initial_authorities
		.iter()
		.map(|x| &x.0)
		.chain(initial_nominators.iter())
		.for_each(|x| {
			if !endowed_accounts.contains(x) {
				endowed_accounts.push(x.clone())
			}
		});

	// stakers: all validators and nominators.
	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
		.chain(initial_nominators.iter().map(|x| {
			use rand::{seq::SliceRandom, Rng};
			let limit = (MaxNominations::get() as usize).min(initial_authorities.len());
			let count = rng.gen::<usize>() % limit;
			let nominations = initial_authorities
				.as_slice()
				.choose_multiple(&mut rng, count)
				.into_iter()
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), STASH, StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();

	let num_endowed_accounts = endowed_accounts.len();

	const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
	const STASH: Balance = ENDOWMENT / 1000;

	GenesisConfig {
		system: SystemConfig { code: wasm_binary_unwrap().to_vec() },
		balances: BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|x| (x, ENDOWMENT)).collect(),
		},
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			..Default::default()
		},
		elections: ElectionsConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.map(|member| (member, STASH))
				.collect(),
		},
		council: CouncilConfig::default(),
		technical_committee: TechnicalCommitteeConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default(),
		},
		sudo: SudoConfig { key: Some(root_key) },
		babe: BabeConfig {
			authorities: vec![],
			epoch_config: Some(node_template_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		im_online: ImOnlineConfig { keys: vec![] },
		authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },
		grandpa: GrandpaConfig { authorities: vec![] },
		technical_membership: Default::default(),
		treasury: Default::default(),
		assets: pallet_assets::GenesisConfig {
			// This asset is used by the NIS pallet as counterpart currency.
			assets: vec![(9, get_account_id_from_seed::<sr25519::Public>("Alice"), true, 1)],
			..Default::default()
		},
		transaction_payment: Default::default(),
		alliance: Default::default(),
		alliance_motion: Default::default(),
		nomination_pools: NominationPoolsConfig {
			min_create_bond: 10 * DOLLARS,
			min_join_bond: 1 * DOLLARS,
			..Default::default()
		},
	}
}

fn development_config_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![authority_keys_from_seed("Alice")],
		vec![],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		development_config_genesis,
		vec![],
		None,
		None,
		None,
		None,
		Default::default(),
	)
}

fn local_testnet_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
		vec![],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		local_testnet_genesis,
		vec![],
		None,
		None,
		None,
		None,
		Default::default(),
	)
}

#[cfg(test)]
pub(crate) mod tests {
	use super::*;
	use crate::service::{new_full_base, NewFullBase};
	use sc_service_test;
	use sp_runtime::BuildStorage;

	fn local_testnet_genesis_instant_single() -> GenesisConfig {
		testnet_genesis(
			vec![authority_keys_from_seed("Alice")],
			vec![],
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			None,
		)
	}

	/// Local testnet config (single validator - Alice)
	pub fn integration_test_config_with_single_authority() -> ChainSpec {
		ChainSpec::from_genesis(
			"Integration Test",
			"test",
			ChainType::Development,
			local_testnet_genesis_instant_single,
			vec![],
			None,
			None,
			None,
			None,
			Default::default(),
		)
	}

	/// Local testnet config (multivalidator Alice + Bob)
	pub fn integration_test_config_with_two_authorities() -> ChainSpec {
		ChainSpec::from_genesis(
			"Integration Test",
			"test",
			ChainType::Development,
			local_testnet_genesis,
			vec![],
			None,
			None,
			None,
			None,
			Default::default(),
		)
	}

	#[test]
	#[ignore]
	fn test_connectivity() {
		sp_tracing::try_init_simple();

		sc_service_test::connectivity(integration_test_config_with_two_authorities(), |config| {
			let NewFullBase { task_manager, client, network, sync, transaction_pool, .. } =
				new_full_base(config, false, |_, _| ())?;
			Ok(sc_service_test::TestNetComponents::new(
				task_manager,
				client,
				network,
				sync,
				transaction_pool,
			))
		});
	}

	#[test]
	fn test_create_development_chain_spec() {
		development_config().build_storage().unwrap();
	}

	#[test]
	fn test_create_local_testnet_chain_spec() {
		local_testnet_config().build_storage().unwrap();
	}

	#[test]
	fn test_staging_test_net_chain_spec() {
		staging_testnet_config().build_storage().unwrap();
	}
}
