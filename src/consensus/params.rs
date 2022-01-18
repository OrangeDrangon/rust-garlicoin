// Rust Garlicoin Library
// Written in 2014 by
//   Andrew Poelstra <apoelstra@wpsoftware.net>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! Garlicoin consensus parameters.
//!
//! This module provides a predefined set of parameters for different Garlicoin
//! chains (such as mainnet, testnet).
//!

use network::constants::Network;
use util::uint::Uint256;

/// Lowest possible difficulty for Mainnet. See comment on Params::pow_limit for more info.
const MAX_BITS_BITCOIN: Uint256 = Uint256([
    0x00000fffffffffffu64,
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
]);
/// Lowest possible difficulty for Testnet. See comment on Params::pow_limit for more info.
const MAX_BITS_TESTNET: Uint256 = Uint256([
    0x00000fffffffffffu64,
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
]);
/// Lowest possible difficulty for Regtest. See comment on Params::pow_limit for more info.
const MAX_BITS_REGTEST: Uint256 = Uint256([
    0x7fffffffffffffffu64,
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
]);

/// Parameters that influence chain consensus.
#[derive(Debug, Clone)]
pub struct Params {
    /// Network for which parameters are valid.
    pub network: Network,
    /// Time when BIP16 becomes active.
    pub bip16_time: u32,
    /// Block height at which BIP34 becomes active.
    pub bip34_height: u32,
    /// Block height at which BIP65 becomes active.
    pub bip65_height: u32,
    /// Block height at which BIP66 becomes active.
    pub bip66_height: u32,
    /// Minimum blocks including miner confirmation of the total of 2016 blocks in a retargeting period,
    /// (nPowTargetTimespan / nPowTargetSpacing) which is also used for BIP9 deployments.
    /// Examples: 1916 for 95%, 1512 for testchains.
    pub rule_change_activation_threshold: u32,
    /// Number of blocks with the same set of rules.
    pub miner_confirmation_window: u32,
    /// Proof of work limit value. It contains the lowest possible difficulty.
    ///
    /// Note that this value differs from Garlicoin Core's powLimit field in that this value is
    /// attainable, but Garlicoin Core's is not. Specifically, because targets in Garlicoin are always
    /// rounded to the nearest float expressible in "compact form", not all targets are attainable.
    /// Still, this should not affect consensus as the only place where the non-compact form of
    /// this is used in Garlicoin Core's consensus algorithm is in comparison and there are no
    /// compact-expressible values between Garlicoin Core's and the limit expressed here.
    pub pow_limit: Uint256,
    /// Expected amount of time to mine one block.
    pub pow_target_spacing: u64,
    /// Difficulty recalculation interval.
    pub pow_target_timespan: u64,
    /// Determines whether minimal difficulty may be used for blocks or not.
    pub allow_min_difficulty_blocks: bool,
    /// Determines whether retargeting is disabled for this network or not.
    pub no_pow_retargeting: bool,
}

impl Params {
    /// Creates parameters set for the given network.
    pub fn new(network: Network) -> Self {
        match network {
            Network::Garlicoin => Params {
                network: Network::Garlicoin,
                bip16_time: 1333238400,                 // Apr 1 2012
                bip34_height: 0, // 2ada80bf415a89358d697569c96eb98cdbf4c3b8878ac5722c01284492e27228
                bip65_height: 0, // bab3041e8977e0dc3eeff63fe707b92bde1dd449d8efafb248c27c8264cc311a
                bip66_height: 0, // 7aceee012833fa8952f8835d8b1b3ae233cd6ab08fdb27a771d2bd7bdc491894
                rule_change_activation_threshold: 6048, // 75%
                miner_confirmation_window: 8064,
                pow_limit: MAX_BITS_BITCOIN,
                pow_target_spacing: 40,       // 40 seconds.
                pow_target_timespan: 60 * 60, // 1 hour.
                allow_min_difficulty_blocks: false,
                no_pow_retargeting: false,
            },
            Network::Testnet => Params {
                network: Network::Testnet,
                bip16_time: 1333238400,                 // Apr 1 2012
                bip34_height: 76, // 8075c771ed8b495ffd943980a95f702ab34fce3c8c54e379548bda33cc8c0573
                bip65_height: 76, // 8075c771ed8b495ffd943980a95f702ab34fce3c8c54e379548bda33cc8c0573
                bip66_height: 76, // 8075c771ed8b495ffd943980a95f702ab34fce3c8c54e379548bda33cc8c0573
                rule_change_activation_threshold: 1512, // 75%
                miner_confirmation_window: 2016,
                pow_limit: MAX_BITS_TESTNET,
                pow_target_spacing: 40,       // 40 seconds.
                pow_target_timespan: 60 * 60, // 1 hour.
                allow_min_difficulty_blocks: true,
                no_pow_retargeting: false,
            },
            Network::Regtest => Params {
                network: Network::Regtest,
                bip16_time: 1333238400,  // Apr 1 2012
                bip34_height: 100000000, // not activated on regtest
                bip65_height: 1351,
                bip66_height: 1251,                    // used only in rpc tests
                rule_change_activation_threshold: 108, // 75%
                miner_confirmation_window: 144,
                pow_limit: MAX_BITS_REGTEST,
                pow_target_spacing: 60,                // 60 seconds.
                pow_target_timespan: 14 * 24 * 6 * 60, // 1.4 days.
                allow_min_difficulty_blocks: true,
                no_pow_retargeting: true,
            },
        }
    }

    /// Calculates the number of blocks between difficulty adjustments.
    pub fn difficulty_adjustment_interval(&self) -> u64 {
        self.pow_target_timespan / self.pow_target_spacing
    }
}
