#ifndef POLYJUICE_GLOBALS_H
#define POLYJUICE_GLOBALS_H

static uint8_t g_rollup_script_hash[32] = {0};
static uint32_t g_sudt_id = UINT32_MAX;
static uint32_t g_tx_origin_id = UINT32_MAX;
/* Receipt.contractAddress - The contract address created, if the transaction was a contract creation, otherwise null */
static uint32_t g_created_id = UINT32_MAX;
static uint8_t g_created_address[20] = {0};
static uint32_t g_creator_account_id = UINT32_MAX;
static evmc_address g_tx_origin = {0};
static uint8_t g_script_code_hash[32] = {0};
static uint8_t g_script_hash_type = 0xff;

#endif // POLYJUICE_GLOBALS_H
