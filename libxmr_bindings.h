#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


const char *generate_address(const char *mnemonic,
                             uint8_t network,
                             uint32_t account,
                             uint32_t index);

const char *generate_seed(void);
