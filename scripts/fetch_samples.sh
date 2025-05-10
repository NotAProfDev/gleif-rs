#!/bin/bash
# fetch_samples.sh
# ---------------------------------------------
# Fetch sample LEI record data from the GLEIF API for a set of LEIs.
#
# Usage:
#   ./fetch_samples.sh
#
# This script downloads sample LEI record JSON responses from the GLEIF API
# for a predefined set of LEIs, saving each response to a separate file in tests/data/.
#
# Requirements:
#   - curl
#   - Internet connection
#
# Output:
#   - lei_records/single_lei_record_<LEI>.json for each LEI in the list
#   - lei_records/<additional_type>_<LEI>.json for each additional LEI record type and LEI
#   - lei_records/multi_lei_records.json containing all LEI records in a single file
#   - relationships/<relationship_endpoint>_<LEI>.json for each relationship endpoint and LEI
#   - reporting_exceptions/<reporting_exception>_<LEI>.json for each reporting exception endpoint and LEI
# ---------------------------------------------

set -euo pipefail

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# Output directory for all sample data
DATA_DIR="$(cd "$SCRIPT_DIR/../tests/data" && pwd)"

# Output subdirectories for each group
LEI_RECORDS_DIR="$DATA_DIR/lei_records"
RELATIONSHIPS_DIR="$DATA_DIR/relationships"
REPORTING_EXCEPTIONS_DIR="$DATA_DIR/reporting_exceptions"

# Extra endpoints
BASE_URL="https://api.gleif.org/api/v1/"
LEI_RECORDS_ENDPOINT="${BASE_URL}lei-records"

# Additional Response Types
ADDITIONAL_LEI_RECORD_TYPES=(
  "direct-children"
  "direct-parent"
  "ultimate-children"
  "ultimate-parent"
  "managing-lou"
)
RELATIONSHIP_TYPES=(
  "direct-child-relationships"
  "direct-parent-relationship"
  "ultimate-child-relationships"
  "ultimate-parent-relationship"
)
REPORTING_EXCEPTIONS=(
  "direct-parent-reporting-exception"
  "ultimate-parent-reporting-exception"
)

# List of LEIs to fetch (with comments for reference)
LEIS=(
  "21380068P1DRHMJ8KU70" # SHELL PLC
  "254900LNRYNOQ9YPU758" # Amazon Digital UK Limited
  "254900MSZM6DMP853B11" # ROCA NETWORKS INC.
  "335800CCCPHKUOXPC332" # NFA INTERNATIONAL
  "5299001ERX0K10IZUL40" # Fraport AG Frankfurt Airport Services Worldwide
  "529900GRZ2BQY5ZM9N49" # PUMA SE
  "5493001KJTIIGC8Y1R12" # Bloomberg Finance L.P.
  "549300YX4S1LLSMK2627" # APPLE ENERGY LLC
  "851WYGNLUQLFZBSYGB56" # COMMERZBANK Aktiengesellschaft
  "INR2EJN1ERAN0W5ZP974" # MICROSOFT CORPORATION
)

# Global progress counter
PROGRESS_CURRENT=0
PROGRESS_TOTAL=0

# Calculate total number of fetches
calculate_total_fetches() {
  local n_leis=${#LEIS[@]}
  local n_relationships=${#RELATIONSHIP_TYPES[@]}
  local n_extras=${#ADDITIONAL_LEI_RECORD_TYPES[@]}
  local n_reporting_exceptions=${#REPORTING_EXCEPTIONS[@]}
  PROGRESS_TOTAL=$((\
    n_leis + \
    n_leis * n_relationships + \
    n_leis * n_extras + \
    n_leis * n_reporting_exceptions + \
    2))
}

# Print usage information
usage() {
  echo "Usage: $0"
  echo "Fetch sample LEI record data from the GLEIF API for a set of LEIs."
  exit 1
}

# Fetch a URL and write to a file, with error handling and progress
fetch_json() {
  local url="$1"
  local out_file="$2"
  PROGRESS_CURRENT=$((PROGRESS_CURRENT + 1))
  echo "[$PROGRESS_CURRENT/$PROGRESS_TOTAL] Fetching $out_file"
  if curl -sSf -H "Accept: application/vnd.api+json" "$url" -o "$out_file"; then
    echo "Fetched: $out_file"
  else
    echo "Warning: Failed to fetch $url" >&2
  fi
  sleep 2 # Pause to avoid overwhelming the API; adjust as needed
}

# Fetch LEI records
fetch_lei_records() {
  for lei in "${LEIS[@]}"; do
    # Fetch single LEI record
    fetch_json "$LEI_RECORDS_ENDPOINT/$lei" "$LEI_RECORDS_DIR/single_lei_record_$lei.json"
    for endpoint in "${ADDITIONAL_LEI_RECORD_TYPES[@]}"; do
      # Fetch additional LEI record types
      fetch_json "$LEI_RECORDS_ENDPOINT/$lei/$endpoint" "$LEI_RECORDS_DIR/${endpoint}_$lei.json"
    done
  done
  local joined_leis
  joined_leis=$(
    IFS=,
    echo "${LEIS[*]}"
  )
  # Fetch all LEI records at once
  fetch_json "$LEI_RECORDS_ENDPOINT?filter%5Blei%5D=$joined_leis" "$LEI_RECORDS_DIR/multi_lei_records.json"
}

# Fetch relationship records data for each LEI
fetch_relationships() {
  for lei in "${LEIS[@]}"; do
    for endpoint in "${RELATIONSHIP_TYPES[@]}"; do
      fetch_json "$LEI_RECORDS_ENDPOINT/$lei/$endpoint" "$RELATIONSHIPS_DIR/${endpoint}_$lei.json"
    done
  done
}

# Fetch reporting exceptions for each LEI
fetch_reporting_exceptions() {
  for lei in "${LEIS[@]}"; do
    for endpoint in "${REPORTING_EXCEPTIONS[@]}"; do
      fetch_json "$LEI_RECORDS_ENDPOINT/$lei/$endpoint" "$REPORTING_EXCEPTIONS_DIR/${endpoint}_$lei.json"
    done
  done
}

# Main entry point
main() {
  # Ensure output directories exist
  mkdir -p "$LEI_RECORDS_DIR" "$RELATIONSHIPS_DIR" "$REPORTING_EXCEPTIONS_DIR"

  calculate_total_fetches

  echo "Fetching LEI records..."
  fetch_lei_records

  echo "Fetching relationship endpoint data..."
  fetch_relationships

  echo "Fetching reporting exceptions..."
  fetch_reporting_exceptions

  echo "Script completed successfully."
}

main "$@"
