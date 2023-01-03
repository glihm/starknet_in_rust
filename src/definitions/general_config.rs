use num_bigint::BigInt;
use num_traits::Zero;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub(crate) enum StarknetChainId {
    MainNet,
    TestNet,
    TestNet2,
}

impl ToString for StarknetChainId {
    fn to_string(&self) -> String {
        match self {
            StarknetChainId::MainNet => "SN_MAIN",
            StarknetChainId::TestNet => "SN_GOERLI",
            StarknetChainId::TestNet2 => "SN_GOERLI2",
        }
        .to_string()
    }
}

impl StarknetChainId {
    pub(crate) fn to_bigint(self) -> BigInt {
        BigInt::from_bytes_be(num_bigint::Sign::Plus, self.to_string().as_bytes())
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub(crate) struct StarknetOsConfig {
    pub(crate) chain_id: StarknetChainId,
    pub(crate) fee_token_address: BigInt,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub(crate) struct StarknetGeneralConfig {
    pub(crate) starknet_os_config: StarknetOsConfig,
}

impl StarknetGeneralConfig {
    pub(crate) fn new() -> Self {
        Self {
            starknet_os_config: StarknetOsConfig {
                chain_id: StarknetChainId::TestNet,
                fee_token_address: BigInt::zero(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bigint_str;

    #[test]
    fn starknet_chain_to_bigint() {
        assert_eq!(
            bigint_str!(b"23448594291968334"),
            StarknetChainId::MainNet.to_bigint()
        );
        assert_eq!(
            bigint_str!(b"1536727068981429685321"),
            StarknetChainId::TestNet.to_bigint()
        );
        assert_eq!(
            bigint_str!(b"393402129659245999442226"),
            StarknetChainId::TestNet2.to_bigint()
        );
    }
}