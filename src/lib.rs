use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData, Clone, Debug)]
pub struct RadixRewardsNftData {
    #[mutable]
    pub key_image_url: Url,
}

#[blueprint]
#[types(RadixRewardsNftData)]
mod radix_rewards_nft_component {
    enable_method_auth! {
        methods {
            mint_and_send_nfts => restrict_to: [OWNER];
        }
    }
    struct RadixRewardsNft {
        nft_manager: NonFungibleResourceManager,
        nft_count: u64,
        nft_key_image_url: Url,
        locker: Global<AccountLocker>,
    }

    impl RadixRewardsNft {
        pub fn instantiate(nft_icon_url: Url, nft_key_image_url: Url, nft_dapp_definition: GlobalAddress, locker: Global<AccountLocker>) -> (Global<RadixRewardsNft>, FungibleBucket) {
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(RadixRewardsNft::blueprint_id());

            let controller_badges: FungibleBucket = ResourceBuilder::new_fungible(OwnerRole::None)
            .divisibility(DIVISIBILITY_NONE)
            .metadata(metadata! (
                init {
                    "name" => "Radix Rewards NFT Owner Badge", locked;
                    "symbol" => "ownRRNFT", locked;
                }
            ))
            .mint_initial_supply(5);

            let controller_badge_address = controller_badges.resource_address();

            let controller_badge_access_rule = rule!(
                require(controller_badge_address) || require(global_caller(component_address))
            );
            let controller_badge_owner_role = OwnerRole::Fixed(controller_badge_access_rule.clone());

            let nft_manager: NonFungibleResourceManager =
                ResourceBuilder::new_integer_non_fungible_with_registered_type::<RadixRewardsNftData>(controller_badge_owner_role.clone())
                .metadata(metadata!(
                    init {
                        "name" => "Radix Rewards Tester", locked;
                        "symbol" => "RRTESTER", locked;
                        "description" => "Awarded for testing Radix Rewards.", locked;
                        "icon_url" => nft_icon_url, updatable;
                        "dapp_definitions" => vec![nft_dapp_definition], updatable;
                    }
                ))
                .mint_roles(mint_roles!(
                    minter => controller_badge_access_rule.clone();
                    minter_updater => controller_badge_access_rule.clone();
                ))
                .create_with_no_initial_supply();

            let radix_rewards_nft = Self {
                nft_manager,
                nft_count: 0,
                nft_key_image_url,
                locker,
            }
            .instantiate()
            .prepare_to_globalize(controller_badge_owner_role)
            .with_address(address_reservation)
            .globalize();

            (radix_rewards_nft, controller_badges)
        }

        pub fn mint_and_send_nfts(
            &mut self,
            recipients: IndexSet<Global<Account>>,
            badge: FungibleBucket,
        ) -> FungibleBucket {
            assert!(!recipients.is_empty(), "No recipients provided");

            let mut nft_bucket: NonFungibleBucket = NonFungibleBucket::new(
                self.nft_manager.address()
            );

            let mut claimants: IndexMap<Global<Account>, ResourceSpecifier> = IndexMap::new();

            for account in recipients {
                let nft_id = NonFungibleLocalId::integer(self.nft_count);
                self.nft_count += 1;

                let nft = self.nft_manager.mint_non_fungible(
                    &nft_id,
                    RadixRewardsNftData {
                        key_image_url: self.nft_key_image_url.clone(),
                    },
                );

                nft_bucket.put(nft);

                claimants.insert(
                    account,
                    ResourceSpecifier::NonFungible(IndexSet::from([nft_id])),
                );
            }

            badge.authorize_with_all(|| {
                self.locker.airdrop(
                    claimants,
                    nft_bucket.into(),
                    true, // try_direct_send
                );
            });

            badge
        }
    }
}