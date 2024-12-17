# Directory to save the account data
output_dir="tests/fixtures/accounts"
mkdir -p "$output_dir"

initialize_tda_accounts () {
  for vote_pubkey in $(cat "$validator_file"); do
    # Derive the TDA address
    tda_address=$(solana-keygen pubkey "$keys_dir/tda_$vote_pubkey.json")

    # Create and fund the TDA account
    solana transfer "$tda_address" 1 --allow-unfunded-recipient --url http://127.0.0.1:8899

    echo "Initialized TDA account: $tda_address"
  done
}

# Function to download account data
download_account_data() {
  local address=$1
  local output_file="$output_dir/$address.json"

  echo "Downloading account data for $address"
  solana account "$address" --output-file "$output_file" --output json
}

# List of derived TDA addresses
tda_addresses=(
  "GRnmQpDc85t9iYrQ1dAHrkFCfBpxmcfZ8bivYixdyfyw"
  "HFwTj4byZaYquUw4UWCjELJ2k5c2HEUYLMFgGGVBLdmm"
  "5ueZLhSYTgXaun4urSW7iQnwtv9wSy5sMNByG22Gjwxt"
  "wFATSPzb2bs8VUDs3u9ZfichzTf9ibhiZJVN1cKo2GM"
  "EHVQV7RA5rvqgS7TZGf4KQb53xa5nqp7yhzUvxFhP8wT"
  "2NPu98QC68Q9fcZg6mgzgaicfHMD2SrwSUsW3sbchwpK"
  "BWE9kYcNYzto5yqz2ADxRQCujUqiTjHRvGNMJH2ehwYj"
  "B96BynBZi7rRbQp847x6KNqs9oxWUKyCgtScq16zeyRz"
  "6A1P7jz3ioonXYCbw2hHh1hc4zAtzp6eBYFtqXXsK9wu"
  "4YvCj2DikCyFj6UKpRQUpiAPQbegucJeiYKMjEkGtjEN"
  "2L8s83JFggPV4JBENTTnVvByP1NpSamHqzKXVGnmucde"
)

initialize_tda_accounts
# Iterate over each TDA address and download the account data
# for address in "${tda_addresses[@]}"; do
#   download_account_data "$address"
# done