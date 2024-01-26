#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct BoundingBox {
  float xc;
  float yc;
  float width;
  float height;
  float angle;
  bool oriented;
} BoundingBox;

/**
 * # Safety
 *
 * The function is intended for invocation from C/C++, so it is unsafe by design.
 */
bool check_version(const char *external_version);

int64_t object_get_id(uintptr_t handle);

bool object_get_confidence(uintptr_t handle, float *conf);

void object_set_confidence(uintptr_t handle, float conf);

void object_clear_confidence(uintptr_t handle);

uintptr_t object_get_namespace(uintptr_t handle, char *caller_allocated_buf, uintptr_t len);

uintptr_t object_get_label(uintptr_t handle, char *caller_allocated_buf, uintptr_t len);

uintptr_t object_get_draw_label(uintptr_t handle, char *caller_allocated_buf, uintptr_t len);

void object_get_detection_box(uintptr_t handle, struct BoundingBox *caller_allocated_bb);

void object_set_detection_box(uintptr_t handle, const struct BoundingBox *bb);

bool object_get_tracking_info(uintptr_t handle,
                              struct BoundingBox *caller_allocated_bb,
                              int64_t *caller_allocated_tracking_id);

void object_set_tracking_info(uintptr_t handle, const struct BoundingBox *bb, int64_t tracking_id);

void object_clear_tracking_info(uintptr_t handle);

bool object_get_float_vec_attribute_value(uintptr_t handle,
                                          const char *namespace_,
                                          const char *name,
                                          uintptr_t index,
                                          double *caller_allocated_result,
                                          uintptr_t *caller_allocated_result_len,
                                          float *caller_allocated_confidence,
                                          bool *caller_allocated_confidence_set);

/**
 * # Safety
 *
 * The function is intended for invocation from C/C++, so it is unsafe by design.
 */
void pipeline2_move_as_is(uintptr_t handle,
                          const char *dest_stage,
                          const int64_t *ids,
                          uintptr_t len);

/**
 * # Safety
 *
 * The function is intended for invocation from C/C++, so it is unsafe by design.
 */
int64_t pipeline2_move_and_pack_frames(uintptr_t handle,
                                       const char *dest_stage,
                                       const int64_t *frame_ids,
                                       uintptr_t len);

/**
 * # Safety
 *
 * The function is intended for invocation from C/C++, so it is unsafe by design.
 */
uintptr_t pipeline2_move_and_unpack_batch(uintptr_t handle,
                                          const char *dest_stage,
                                          int64_t batch_id,
                                          int64_t *resulting_ids,
                                          uintptr_t resulting_ids_len);

/**
 * # Safety
 *
 * The function is intended for invocation from C/C++, so it is unsafe by design.
 *
 * Arguments
 * ---------
 * handle: usize
 *   The pipeline handle
 * id: i64
 *   The batch or frame id to apply updates to
 *
 * Returns
 * -------
 * bool
 *   True if the updates were applied, false otherwise
 *
 */
bool pipeline2_apply_updates(uintptr_t handle, int64_t id);

/**
 * # Safety
 *
 * The function is intended for invocation from C/C++, so it is unsafe by design.
 *
 * Arguments
 * ---------
 * handle: usize
 *   The pipeline handle
 * id: i64
 *   The batch or frame id to clear updates from
 *
 * Returns
 * -------
 * bool
 *   True if the updates were cleared, false otherwise
 *
 */
bool pipeline2_clear_updates(uintptr_t handle, int64_t id);
