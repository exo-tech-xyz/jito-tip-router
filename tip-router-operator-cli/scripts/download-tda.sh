# Directory to save the account data
output_dir="tests/fixtures/accounts"
mkdir -p "$output_dir"

stake_meta_json='{
  "stake_metas": [
    {
      "validator_vote_account": "2rnDMvRuvdq9QTViEziTPmSof7JqsdFjajfGUgcqX2pN",
      "validator_node_pubkey": "EcYQ6p8BfHu5u1oH6KXW7Fdgf719dpj9H7pxSaYVgyhT",
      "maybe_tip_distribution_meta": null,
      "delegations": [
        {
          "stake_account_pubkey": "GdeAw7P5j27Zepm2WX2XWHno8nL7eJHF7PGkrFoHni5d",
          "staker_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "withdrawer_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "lamports_delegated": 3000000000
        },
        {
          "stake_account_pubkey": "HAWTV1apRR774jRJUJ69zRBZArk66AGekAWK1tV4eWbq",
          "staker_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "withdrawer_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "lamports_delegated": 1000000000
        }
      ],
      "total_delegated": 4000000000,
      "commission": 1
    },
    {
      "validator_vote_account": "2z4wzY1KgeqEuViJowroygn9bvqnwRNtZZrJrfmEUgw1",
      "validator_node_pubkey": "DEdjbQJWejSpE8hZ6MVWp56u5pFtZUhV2pteBLvDcaa7",
      "maybe_tip_distribution_meta": null,
      "delegations": [
        {
          "stake_account_pubkey": "3BhnvXetarYptmmzAdiLdbWaPzK4yMc3ftaGU8RVWcGU",
          "staker_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "withdrawer_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "lamports_delegated": 1000000000
        },
        {
          "stake_account_pubkey": "9czjNT9VZMLYo1VeMazuFz4naiVBiDXJX7o2uFRx4TuR",
          "staker_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "withdrawer_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "lamports_delegated": 3000000000
        }
      ],
      "total_delegated": 4000000000,
      "commission": 1
    },
    {
      "validator_vote_account": "76hRaCTRMFPMjnfW8LxuppWsXP7CytJeKc4FHmpvyeX2",
      "validator_node_pubkey": "4bxmYxF5TXozJXA2T7J49K7oPRDBoYkCmZ6Z6T6yF2ex",
      "maybe_tip_distribution_meta": null,
      "delegations": [
        {
          "stake_account_pubkey": "C6PnxPAn35UigiDc2KJds4nHem9r6hRRxkkdHzkR8tqK",
          "staker_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "withdrawer_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "lamports_delegated": 4000000000
        }
      ],
      "total_delegated": 4000000000,
      "commission": 1
    },
    {
      "validator_vote_account": "9pGeMuYMMEKJC5j7UsPziKE6w4czn15Wq46pzC3urswh",
      "validator_node_pubkey": "BV5orTqfY7uNnyAxY5NsAB8EyJxZ5Wz2jhabGzZX9upN",
      "maybe_tip_distribution_meta": null,
      "delegations": [
        {
          "stake_account_pubkey": "sntnKuhyRVSRB54U9DtvsNQHxNXBYDvExDx97Xqv4yE",
          "staker_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "withdrawer_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "lamports_delegated": 1000000000
        },
        {
          "stake_account_pubkey": "HgxyS26b2A656BcR56V4dALHQAUJiKBYmQ56vedFqPU4",
          "staker_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "withdrawer_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "lamports_delegated": 3000000000
        }
      ],
      "total_delegated": 4000000000,
      "commission": 1
    },
    {
      "validator_vote_account": "A6sgpbXNSjWVMhik7gGPoNsmrR7Wbufv5qta8yt5tnvj",
      "validator_node_pubkey": "6JXNuecCvwRRhsA76T7uQAWxCZCP35q8gkSkz7w7k6ev",
      "maybe_tip_distribution_meta": null,
      "delegations": [
        {
          "stake_account_pubkey": "cfKAD2PLwukT1yFVvyLX9HZKkd4kPTNCkdzUGUEJLaU",
          "staker_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "withdrawer_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "lamports_delegated": 3000000000
        },
        {
          "stake_account_pubkey": "ovwH9EM1yXtGhsmFhGZvBJvSSBR73igwJ7aDEUYHFKv",
          "staker_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "withdrawer_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "lamports_delegated": 1000000000
        }
      ],
      "total_delegated": 4000000000,
      "commission": 1
    },
    {
      "validator_vote_account": "A9DPUVtizTfQ1iqemNVoMWtvzwp1axX5Wyijhp4e9SmQ",
      "validator_node_pubkey": "T4KRVmxidyCwSatQPd1NZJBdFfUZg652YyknbWShDCu",
      "maybe_tip_distribution_meta": null,
      "delegations": [
        {
          "stake_account_pubkey": "95cjqc8huFhDMe8v4NaGeSQmNhXvoZoeNZEwJhyyevWa",
          "staker_pubkey": "95cjqc8huFhDMe8v4NaGeSQmNhXvoZoeNZEwJhyyevWa",
          "withdrawer_pubkey": "95cjqc8huFhDMe8v4NaGeSQmNhXvoZoeNZEwJhyyevWa",
          "lamports_delegated": 1000046515444390
        }
      ],
      "total_delegated": 1000046515444390,
      "commission": 0
    },
    {
      "validator_vote_account": "AxtWXZfy4yv6UsVBnMefu41ggiS5NdV8VMHEMLqrFJC8",
      "validator_node_pubkey": "5M9gSjmpYhhusX5NhTEQwNViX4d4FhfmAZLyqTzs7xRM",
      "maybe_tip_distribution_meta": null,
      "delegations": [
        {
          "stake_account_pubkey": "Exn2zjsbVz4qLn6JvqSjGvPqAJPsLcdtYvup5b5oGMpa",
          "staker_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "withdrawer_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "lamports_delegated": 4000000000
        }
      ],
      "total_delegated": 4000000000,
      "commission": 1
    },
    {
      "validator_vote_account": "BuA3rpzJ54EU51gpm6oKYmyK5pvxArCu1q9nqqSngN9Q",
      "validator_node_pubkey": "DBJxNPwVkfeufeeeZunTspsCj4oa1yTDfAJNBfdNy7jj",
      "maybe_tip_distribution_meta": null,
      "delegations": [
        {
          "stake_account_pubkey": "FP8qZPJkd27PCSZnkqn8Qt2b7pircdq9vorFp2HyLFz",
          "staker_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "withdrawer_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "lamports_delegated": 4000000000
        }
      ],
      "total_delegated": 4000000000,
      "commission": 1
    },
    {
      "validator_vote_account": "ENprLrxZuy9VEccMEBnKkE4tQkot58FYhaQG1zkQoMyL",
      "validator_node_pubkey": "3eWuD35KAbo7LUjyGyQuCYEmzvFQhvZdRiVpco7K3TCb",
      "maybe_tip_distribution_meta": null,
      "delegations": [
        {
          "stake_account_pubkey": "7Eb8LFTqwRfurqwJVna7CzDriCaWZX65awXgwYKWAA65",
          "staker_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "withdrawer_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "lamports_delegated": 4000000000
        }
      ],
      "total_delegated": 4000000000,
      "commission": 1
    },
    {
      "validator_vote_account": "EqnPfbyXGFWwRBn3E6UUvK1HrLN2iwqkh9YP12z8bDdy",
      "validator_node_pubkey": "GuYSZhf9y9ysR3P89CD4SFpQGi6JXd3akPTiVvuPJroA",
      "maybe_tip_distribution_meta": null,
      "delegations": [
        {
          "stake_account_pubkey": "5K4U6zEqE6FLepvJcvqSv4JjZDsdMPHHwZppEYBttDgp",
          "staker_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "withdrawer_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "lamports_delegated": 3000000000
        },
        {
          "stake_account_pubkey": "Eh7PYSBZhso2PtkSv3PpE5QdMvubnaevnM1SnKF8PXtv",
          "staker_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "withdrawer_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "lamports_delegated": 1000000000
        }
      ],
      "total_delegated": 4000000000,
      "commission": 1
    },
    {
      "validator_vote_account": "H28EbU8vF8NzpYn6R6ZYSMv7CUMTM47j5FUVbaDypmGD",
      "validator_node_pubkey": "HLtptJScyxEf1aLxYUkpqnyaGkRxbxqQyasZQ7b2MA18",
      "maybe_tip_distribution_meta": null,
      "delegations": [
        {
          "stake_account_pubkey": "o6XtvTQoNsRNxXBVjjPZQhbHxzdef4h3TsHUz1gtoqa",
          "staker_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "withdrawer_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "lamports_delegated": 1000000000
        },
        {
          "stake_account_pubkey": "69PJNN3L5NptwF4SEf3hEG4nBj3Xb8DHS9ExKW85HnYM",
          "staker_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "withdrawer_pubkey": "DuRNtEnbqbe9jEjb1d8WzEdVs1exErFh11uBtpbwm8ed",
          "lamports_delegated": 3000000000
        }
      ],
      "total_delegated": 4000000000,
      "commission": 1
    }
  ],
  "tip_distribution_program_id": "4R3gSG8BpU4t19KYj8CfnbtRpnT8gtk4dvTHxVRwc2r7",
  "bank_hash": "5RoMJmhu6uVnUvop8TnRbdaHCQNxVAhEyXQACdnAcym6",
  "epoch": 4,
  "slot": 144
}'

# Parse the JSON and iterate over each 
echo "$stake_meta_json" | jq -c '.stake_metas[]' | while read -r stake_meta; do
  validator_vote_account=$(echo "$stake_meta" | jq -r '.validator_vote_account')
  validator_node_pubkey=$(echo "$stake_meta" | jq -r '.validator_node_pubkey')
  total_delegated=$(echo "$stake_meta" | jq -r '.total_delegated')
  commission=$(echo "$stake_meta" | jq -r '.commission')

  # Call the Rust script with the extracted data
    cargo run --bin serialize-accounts -- \
    --validator-vote-account "$validator_vote_account" \
    --merkle-root-upload-authority "$validator_node_pubkey" \
    --epoch-created-at 0 \
    --validator-commission-bps "$commission" \
    --expires-at 1000 \
    --bump 1
done