#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_Tag {
  struct wire_uint_8_list *title;
  struct wire_uint_8_list *artist;
  struct wire_uint_8_list *album;
  int32_t *year;
  struct wire_uint_8_list *genre;
  double *duration;
  struct wire_uint_8_list *picture;
} wire_Tag;

typedef struct WireSyncReturnStruct {
  uint8_t *ptr;
  int32_t len;
  bool success;
} WireSyncReturnStruct;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

void wire_read(int64_t port_, struct wire_uint_8_list *path);

void wire_write(int64_t port_, struct wire_uint_8_list *path, struct wire_Tag *data);

double *new_box_autoadd_f64_0(double value);

int32_t *new_box_autoadd_i32_0(int32_t value);

struct wire_Tag *new_box_autoadd_tag_0(void);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

void free_WireSyncReturnStruct(struct WireSyncReturnStruct val);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_read);
    dummy_var ^= ((int64_t) (void*) wire_write);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_f64_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i32_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_tag_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturnStruct);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}