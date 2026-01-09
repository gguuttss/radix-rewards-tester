send Radix Reward NFTs, package is live on Stokenet, at: `package_tdx_2_1phcdk4r98k49wus9jfz4n2dhutagmdwr0nsn4l6y796y4syndpk98l`

## Step 1: Instantiate an Account Locker
The mint and send component needs to use an AccountLocker, you can use an existing one, but if you don't have one you want to use, you can use the AccountLockerWrapper package (example deployed at `package_tdx_2_1pk8hnmyp36uduxk86cjt0w28w76kxdyuz76th89qr6peyjv2yednwr`) to instantiate one.

To instantiate one, you need a resource address of a badge you want to assign as the owner of the locker you're instantiated. You will need this badge to interact with any of the admin methods on the locker (airdrop, recover tokens, etc.):

```
CALL_FUNCTION
  Address("package_tdx_2_1pk8hnmyp36uduxk86cjt0w28w76kxdyuz76th89qr6peyjv2yednwr")
  "AccountLockerWrapper"
  "instantiate"
  Enum<2u8>(
    Enum<0u8>(
      Enum<0u8>(
        Enum<1u8>(
          Address("resource_tdx_2_1t4mpfndnsszcfjw0c6kmdzxwzrnr64kcptmvgcx6l3ad6ul5s7c8pf") # owner badge address
        )
      )
    )
  )
;
```

## Step 2: Instantiate component
To instantiate the mint and send component, you need: 1. an icon_url for the NFT collection, 2. an image_url for the individual NFTs, 3. a dApp definition for the NFTs, 4. an AccountLocker you want to use (instantiated in previous step), 5. the address of a badge you want to use as owner role for this component. For testing purposes you can obviously use placeholders for the first 3.

Manifest:
```
CALL_FUNCTION
  Address("{package_address}")
  "RadixRewardsNft"
  "instantiate"
  "https://example.com" #icon_url
  "https://example.com" #key_image_url
  Address("{dapp_def_for_nft}")
  Address("{account_locker}")
  Address("{owner_role_badge_address}"}
;

CALL_METHOD
  Address("account_tdx_2_12yjctk8r4csusav9c7z9a7j9vahmnnhnht5ym2ffngh9rqyajsgsdd")
  "deposit_batch"
  Expression("ENTIRE_WORKTOP")
;
```

## Step 3: Send NFTs
To send NFTs, you need to call the `mint_and_send_nfts` method. You need to pass all addresses you want to send an NFT to, and the badge that is authorized to airdrop using your Account Locker (with the resource address from step 1).

Manifest:

```
CALL_METHOD
  Address("{dev_account_address}")
  "withdraw"
  Address("{badge_with_airdrop_permission}")
  Decimal("1")
;

TAKE_ALL_FROM_WORKTOP
  Address("{badge_with_airdrop_permission}")
  Bucket("badge")
;

CALL_METHOD
  Address("{dev_account_address}")
  "create_proof_of_amount"
  Address("resource_tdx_2_1t4mpfndnsszcfjw0c6kmdzxwzrnr64kcptmvgcx6l3ad6ul5s7c8pf")
  Decimal("1")
;

CALL_METHOD
  Address("{instantiated_nft_mint_component}")
  "mint_and_send_nfts"
  Array<Address>(
    Address("{account_to_airdrop_to_1}"),
    Address("{account_to_airdrop_to_2}")
  )
  Bucket("badge")
;

CALL_METHOD
  Address("{dev_account_address}")
  "deposit_batch"
  Expression("ENTIRE_WORKTOP")
;

```
