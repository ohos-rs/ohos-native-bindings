/** @brief Parallel event. @since 26.0.0 */
typedef struct ArkUI_ParallelGestureEvent ArkUI_ParallelGestureEvent;

/** @brief Metadata extension. @since 26.0.0 */
typedef struct OH_Camera_MetadataObjectExt OH_Camera_MetadataObjectExt;

/** @brief Native gesture API. @since 26.0.0 */
typedef struct {
    /** @brief Callback setter. @since 26.0.0 */
    int (*setGestureParallelTo)(void* node, void* userData,
        void* (*parallelGesture)(ArkUI_ParallelGestureEvent* event));
} ArkUI_NativeGestureAPI_3;

/** @brief Metadata access, deliberately excluded by the Camera allowlist. @since 26.0.0 */
int OH_MetadataObjectExt_GetType(OH_Camera_MetadataObjectExt* object);
