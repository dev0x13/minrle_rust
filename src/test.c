#include <string.h>
#include <stdio.h>
#include <stdint.h>

#define CHECK(cond) do { if (!(cond)) return -1; } while (0)

extern int rle_encode(uint8_t* src, size_t src_len, uint8_t* dst, size_t dst_len, size_t* bytes_written);
extern int rle_decode(uint8_t* src, size_t src_len, uint8_t* dst, size_t dst_len, size_t* bytes_written);

int test_hello() {
  uint8_t enc[20];
  uint8_t dec[20];
  size_t written;
  CHECK(rle_encode((uint8_t *)"hello", 5, enc, sizeof(enc), &written) == 0);
  CHECK(written == 8);
  uint8_t expected[8] = {1, 'h', 1, 'e', 2, 'l', 1, 'o' };
  CHECK(memcmp(enc, expected, 8) == 0);
  CHECK(rle_decode(enc, 8, dec, sizeof(dec), &written) == 0);
  CHECK(written == 5);
  CHECK(memcmp(dec, "hello", 5) == 0);
  return 0;
};

int test_empty_buffer() {
  uint8_t enc[20];
  size_t written;
  CHECK(rle_encode((uint8_t *)0, 0, enc, sizeof(enc), &written) == 0);
  CHECK(written == 0);
  return 0;
}

int test_short_buffer() {
  uint8_t enc[20];
  uint8_t dec[20];
  size_t written;
  CHECK(rle_encode((uint8_t *)"hello", 5, enc, 8, &written) == 0);
  CHECK(written == 8);
  CHECK(rle_encode((uint8_t *)"hello", 5, enc, 7, &written) == -1);

  CHECK(rle_decode(enc, 8, dec, 5, &written) == 0);
  CHECK(written == 5);
  CHECK(rle_decode(enc, 8, dec, 4, &written) == -1);
  return 0;
}

#define TEST(test) do { if (test() != 0) { exit_code = -1; printf(#test " FAIL\n"); } else { printf(#test " OK\n"); } } while(0)

int main() {
  int exit_code = 0;
  TEST(test_hello);
  TEST(test_empty_buffer);
  TEST(test_short_buffer);

  return exit_code;
}
