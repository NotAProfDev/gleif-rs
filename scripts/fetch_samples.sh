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
#   - isins/isins_<LEI>.json for ISIN endpoint data for each LEI
#   - lei_issuers/lei_issuer_<LEI>.json for LEI Issuer endpoint data for each LEI
#   - lei_issuers/lei_issuer_jurisdictions_<LEI>.json for LEI Issuer jurisdictions endpoint data for each LEI
#   - lei_issuers/lei_record_issuer_<LEI>.json for LEI Record Issuer endpoint data for each LEI
#   - lei_issuers/lei_issuers_all.json for all LEI Issuers
#   - vlei_issuers/vlei_issuer_<LEI>.json for vLEI Issuer endpoint data for each LEI
#   - vlei_issuers/vlei_issuers_all.json for all vLEI Issuers
#   - field_modifications/field_modifications_<LEI>.json for field modifications endpoint data for each LEI
#   - fields/fields_all.json for all fields
#   - fields/field_<ID>.json for specific field details by ID
#   - countries/countries_all.json for all countries
#   - countries/country_<ID>.json for specific country details by ID
#   - entity_legal_forms/entity_legal_forms_all.json for all entity legal forms
#   - entity_legal_forms/entity_legal_form_<ID>.json for specific entity legal form details by ID
#   - official_organizational_roles/official_organizational_roles_all.json for all official organizational roles
#   - official_organizational_roles/official_organizational_role_<ID>.json for specific official organizational role details by ID
#   - jurisdictions/jurisdictions_all.json for all jurisdictions
#   - jurisdictions/jurisdiction_<ID>.json for specific jurisdiction details by ID
#   - regions/regions_all.json for all regions
#   - regions/region_<ID>.json for specific region details by ID
#   - registration_authorities/registration_authorities_all.json for all registration authorities
#   - registration_authorities/registration_authority_<ID>.json for specific registration authority details by ID
#   - registration_agents/registration_agents_all.json for all registration agents
#   - registration_agents/registration_agent_<ID>.json for specific registration agent details by ID
#   - fuzzy_completions/fuzzycompletions_all.json for fuzzy completions endpoint data
#   - autocompletions/autocompletions_all.json for autocompletions endpoint data
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
ISINS_DIR="$DATA_DIR/isins"
LEI_ISSUERS_DIR="$DATA_DIR/lei_issuers"
VLEI_ISSUERS_DIR="$DATA_DIR/vlei_issuers"
FIELD_MODIFICATIONS_DIR="$DATA_DIR/field_modifications"
FIELD_DIR="$DATA_DIR/fields"
COUNTRY_DIR="$DATA_DIR/countries"
ENTITY_LEGAL_FORM_DIR="$DATA_DIR/entity_legal_forms"
OFFICIAL_ORG_ROLE_DIR="$DATA_DIR/official_organizational_roles"
JURISDICTION_DIR="$DATA_DIR/jurisdictions"
REGION_DIR="$DATA_DIR/regions"
REGISTRATION_AUTHORITY_DIR="$DATA_DIR/registration_authorities"
REGISTRATION_AGENT_DIR="$DATA_DIR/registration_agents"
FUZZY_COMPLETION_DIR="$DATA_DIR/fuzzy_completions"
AUTOCOMPLETION_DIR="$DATA_DIR/auto_completions"

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
  # Companies
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

  # Issuers
  "549300O897ZC5H7CY412" # Nordic Legal Entity Identifier AB (Nord vLEI)
  "984500983AD71E4FBC41" # Provenant Inc.
)

# Example IDs for metadata endpoints
FIELD_IDS=(LEIREC_LEGAL_NAME LEIREC_ENTITY_STATUS ISIN_MAPPING_CODE)
COUNTRY_CODES=(US DE GB)
ENTITY_LEGAL_FORM_CODES=(10UR 12N6 1VTA)
ORG_ROLE_CODES=(0CGNG5 1FWPRU 2YJ8BB)
JURISDICTION_CODES=(US AO-HUA AF-BGL)
REGION_CODES=(AD-03 AE-SH AO-LNO)
REGISTRATION_AUTHORITY_CODES=(RA000001 RA000044 RA000097)
REGISTRATION_AGENT_IDS=(5d10d4dc929ab6.72309473 5d10d4ddcb58f6.31794003 67bc6e3ad4bcd8.33945795)

# Collect all output directories in an array
OUTPUT_DIRS=(
  "$LEI_RECORDS_DIR" "$RELATIONSHIPS_DIR" "$REPORTING_EXCEPTIONS_DIR" "$ISINS_DIR"
  "$LEI_ISSUERS_DIR" "$VLEI_ISSUERS_DIR" "$FIELD_MODIFICATIONS_DIR" "$FIELD_DIR"
  "$COUNTRY_DIR" "$ENTITY_LEGAL_FORM_DIR" "$OFFICIAL_ORG_ROLE_DIR" "$JURISDICTION_DIR"
  "$REGION_DIR" "$REGISTRATION_AUTHORITY_DIR" "$REGISTRATION_AGENT_DIR" "$FUZZY_COMPLETION_DIR" "$AUTOCOMPLETION_DIR"
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
  local n_field_ids=${#FIELD_IDS[@]}
  local n_country_ids=${#COUNTRY_CODES[@]}
  local n_elf_ids=${#ENTITY_LEGAL_FORM_CODES[@]}
  local n_org_role_ids=${#ORG_ROLE_CODES[@]}
  local n_jurisdiction_ids=${#JURISDICTION_CODES[@]}
  local n_region_ids=${#REGION_CODES[@]}
  local n_ra_ids=${#REGISTRATION_AUTHORITY_CODES[@]}
  local n_agent_ids=${#REGISTRATION_AGENT_IDS[@]}
  PROGRESS_TOTAL=$((\
    n_leis + \
    n_leis * n_relationships + \
    n_leis * n_extras + \
    n_leis * n_reporting_exceptions + \
    n_leis + \
    n_leis * 3 + \
    3 + \
    n_leis + 1 + \
    n_leis + \
    1 + n_field_ids + \
    1 + n_country_ids + \
    1 + n_elf_ids + \
    1 + n_org_role_ids + \
    1 + n_jurisdiction_ids + \
    1 + n_region_ids + \
    1 + n_ra_ids + \
    1 + n_agent_ids + \
    1)) # field_modifications, fields, countries, entity legal forms, org roles, jurisdictions, regions endpoints, registration authorities, registration agents, fuzzy completions, autocompletions
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

# Fetch ISINs for each LEI
fetch_isins() {
  for lei in "${LEIS[@]}"; do
    fetch_json "${LEI_RECORDS_ENDPOINT}/$lei/isins" "$ISINS_DIR/isins_$lei.json"
  done
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

# Fetch LEI Issuer endpoints for each LEI
fetch_lei_issuers_endpoints() {
  for lei in "${LEIS[@]}"; do
    # /lei-issuers/{lei}
    fetch_json "${BASE_URL}lei-issuers/$lei" "$LEI_ISSUERS_DIR/lei_issuer_$lei.json"
    # /lei-issuers/{lei}/jurisdictions
    fetch_json "${BASE_URL}lei-issuers/$lei/jurisdictions" "$LEI_ISSUERS_DIR/lei_issuer_jurisdictions_$lei.json"
    # /lei-records/{lei}/lei-issuer
    fetch_json "${LEI_RECORDS_ENDPOINT}/$lei/lei-issuer" "$LEI_ISSUERS_DIR/lei_record_issuer_$lei.json"
  done
  # /lei-issuers (all issuers)
  fetch_json "${BASE_URL}lei-issuers" "$LEI_ISSUERS_DIR/lei_issuers_all.json"
}

# Fetch vLEI Issuer endpoints for each LEI
fetch_vlei_issuers_endpoints() {
  for lei in "${LEIS[@]}"; do
    fetch_json "${BASE_URL}vlei-issuers/$lei" "$VLEI_ISSUERS_DIR/vlei_issuer_$lei.json"
  done
  fetch_json "${BASE_URL}vlei-issuers" "$VLEI_ISSUERS_DIR/vlei_issuers_all.json"
}

# Fetch field modifications for each LEI
fetch_field_modifications() {
  for lei in "${LEIS[@]}"; do
    fetch_json "${LEI_RECORDS_ENDPOINT}/$lei/field-modifications" "$FIELD_MODIFICATIONS_DIR/field_modifications_$lei.json"
  done
}

# Refactored fetch functions for metadata endpoints
fetch_fields() {
  fetch_json "${BASE_URL}fields" "$FIELD_DIR/fields_all.json"
  for id in "${FIELD_IDS[@]}"; do
    fetch_json "${BASE_URL}fields/$id" "$FIELD_DIR/field_$id.json"
  done
}

fetch_countries() {
  fetch_json "${BASE_URL}countries" "$COUNTRY_DIR/countries_all.json"
  for code in "${COUNTRY_CODES[@]}"; do
    fetch_json "${BASE_URL}countries/$code" "$COUNTRY_DIR/country_$code.json"
  done
}

fetch_entity_legal_forms() {
  fetch_json "${BASE_URL}entity-legal-forms" "$ENTITY_LEGAL_FORM_DIR/entity_legal_forms_all.json"
  for code in "${ENTITY_LEGAL_FORM_CODES[@]}"; do
    fetch_json "${BASE_URL}entity-legal-forms/$code" "$ENTITY_LEGAL_FORM_DIR/entity_legal_form_$code.json"
  done
}

fetch_official_organizational_roles() {
  fetch_json "${BASE_URL}official-organizational-roles" "$OFFICIAL_ORG_ROLE_DIR/official_organizational_roles_all.json"
  for code in "${ORG_ROLE_CODES[@]}"; do
    fetch_json "${BASE_URL}official-organizational-roles/$code" "$OFFICIAL_ORG_ROLE_DIR/official_organizational_role_$code.json"
  done
}

fetch_jurisdictions() {
  fetch_json "${BASE_URL}jurisdictions" "$JURISDICTION_DIR/jurisdictions_all.json"
  for code in "${JURISDICTION_CODES[@]}"; do
    fetch_json "${BASE_URL}jurisdictions/$code" "$JURISDICTION_DIR/jurisdiction_$code.json"
  done
}

fetch_regions() {
  fetch_json "${BASE_URL}regions" "$REGION_DIR/regions_all.json"
  for code in "${REGION_CODES[@]}"; do
    fetch_json "${BASE_URL}regions/$code" "$REGION_DIR/region_$code.json"
  done
}

fetch_registration_authorities() {
  fetch_json "${BASE_URL}registration-authorities" "$REGISTRATION_AUTHORITY_DIR/registration_authorities_all.json"
  for code in "${REGISTRATION_AUTHORITY_CODES[@]}"; do
    fetch_json "${BASE_URL}registration-authorities/$code" "$REGISTRATION_AUTHORITY_DIR/registration_authority_$code.json"
  done
}

fetch_registration_agents() {
  fetch_json "${BASE_URL}registration-agents" "$REGISTRATION_AGENT_DIR/registration_agents_all.json"
  for id in "${REGISTRATION_AGENT_IDS[@]}"; do
    fetch_json "${BASE_URL}registration-agents/$id" "$REGISTRATION_AGENT_DIR/registration_agent_$id.json"
  done
}

fetch_fuzzy_completions() {
  fetch_json "${BASE_URL}fuzzycompletions?field=fulltext&q=factbook" "$FUZZY_COMPLETION_DIR/fuzzycompletions_all.json"
}

fetch_autocompletions() {
  fetch_json "${BASE_URL}autocompletions?field=fulltext&q=Global" "$AUTOCOMPLETION_DIR/autocompletions_all.json"
}

# Main entry point
main() {
  # Ensure output directories exist
  for dir in "${OUTPUT_DIRS[@]}"; do
    mkdir -p "$dir"
  done

  calculate_total_fetches

  echo "Fetching LEI records..."
  fetch_lei_records

  echo "Fetching ISINs..."
  fetch_isins

  echo "Fetching relationship endpoint data..."
  fetch_relationships

  echo "Fetching reporting exceptions..."
  fetch_reporting_exceptions

  echo "Fetching LEI Issuer endpoints..."
  fetch_lei_issuers_endpoints

  echo "Fetching vLEI Issuer endpoints..."
  fetch_vlei_issuers_endpoints

  echo "Fetching field modifications..."
  fetch_field_modifications

  echo "Fetching fields..."
  fetch_fields

  echo "Fetching countries..."
  fetch_countries

  echo "Fetching entity legal forms..."
  fetch_entity_legal_forms

  echo "Fetching official organizational roles..."
  fetch_official_organizational_roles

  echo "Fetching jurisdictions..."
  fetch_jurisdictions

  echo "Fetching regions..."
  fetch_regions

  echo "Fetching registration authorities..."
  fetch_registration_authorities

  echo "Fetching registration agents..."
  fetch_registration_agents

  echo "Fetching fuzzy completions..."
  fetch_fuzzy_completions

  echo "Fetching autocompletions..."
  fetch_autocompletions

  echo "Script completed successfully."
}

main "$@"
