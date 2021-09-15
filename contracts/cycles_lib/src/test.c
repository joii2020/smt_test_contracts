
#include <stddef.h>
#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include "blake2b_decl_only.h"
#include "ckb_smt.h"

//#define NO_SMT
//#define NO_VERIFY

int getbin(int x) {
  if (x >= '0' && x <= '9') return x - '0';
  if (x >= 'A' && x <= 'F') return x - 'A' + 10;
  return x - 'a' + 10;
}

int hex2bin(uint8_t* buf, const char* src) {
  size_t length = strlen(src) / 2;
  if (src[0] == '0' && (src[1] == 'x' || src[1] == 'X')) {
    src += 2;
    length--;
  }
  for (size_t i = 0; i < length; i++) {
    buf[i] = (getbin(src[i * 2]) << 4) | getbin(src[i * 2 + 1]);
  }
  return length;
}

int test_verify() {
    uint8_t key[32];
  uint8_t value[32];
  uint8_t root_hash[32];
  uint8_t proof[96];

  hex2bin(key,
          "0x381dc5391dab099da5e28acd1ad859a051cf18ace804d037f12819c6fbc0e18b");
  hex2bin(value,
          "0x9158ce9b0e11dd150ba2ae5d55c1db04b1c5986ec626f2e38a93fe8ad0b2923b");
  hex2bin(root_hash,
          "0xebe0fab376cd802d364eeb44af20c67a74d6183a33928fead163120ef12e6e06");
#if !defined(NO_SMT) && !defined(NO_VERIFY)
  int proof_length = 
#endif
  hex2bin(
      proof,
      "0x4c4fff51ff322de8a89fe589987f97220cfcb6820bd798b31a0b56ffea221093d35f90"
      "9e580b00000000000000000000000000000000000000000000000000000000000000");

#ifdef NO_SMT
    return 0;
#else // NO_SMT
    smt_pair_t entries[8];
    smt_state_t changes;
    smt_state_init(&changes, entries, 32);
    smt_state_insert(&changes, key, value);
    smt_state_normalize(&changes);

#ifdef NO_VERIFY
    return 0;
#else // NO_VERIFY
    return smt_verify(root_hash, &changes, proof, proof_length);
#endif // NO_VERIFY
#endif // NO_SMT
}
