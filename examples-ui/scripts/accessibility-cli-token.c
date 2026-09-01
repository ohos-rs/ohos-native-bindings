// QEMU-only wrapper for running the system accessibility CLI with the
// permission required to enable the built-in screen reader.
#include <dlfcn.h>
#include <errno.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

typedef struct {
  int32_t dcaps_num;
  int32_t perms_num;
  int32_t acls_num;
  const char **dcaps;
  const char **perms;
  const char **acls;
  const char *process_name;
  const char *apl;
  int32_t uid;
} NativeTokenInfoParams;

typedef uint64_t (*GetAccessTokenIdFn)(NativeTokenInfoParams *token_info);
typedef int (*SetSelfTokenIdFn)(uint64_t token_id);

static void *open_first(const char *const libraries[]) {
  for (size_t index = 0; libraries[index] != NULL; ++index) {
    void *handle = dlopen(libraries[index], RTLD_NOW | RTLD_LOCAL);
    if (handle != NULL) {
      return handle;
    }
  }
  return NULL;
}

static void *required_symbol(void *handle, const char *name) {
  dlerror();
  void *symbol = dlsym(handle, name);
  const char *error = dlerror();
  if (error != NULL) {
    fprintf(stderr, "accessibility-cli-token: dlsym(%s): %s\n", name, error);
    return NULL;
  }
  return symbol;
}

int main(int argc, char *argv[]) {
  static const char *const native_token_libraries[] = {
      "libnativetoken_shared.z.so",
      "libnativetoken_shared.so",
      "libnativetoken.z.so",
      "libnativetoken.so",
      NULL,
  };
  static const char *const setproc_libraries[] = {
      "libtokensetproc_shared.z.so",
      "libtokensetproc_shared.so",
      "libtoken_setproc.z.so",
      "libtoken_setproc.so",
      NULL,
  };
  static const char *permissions[] = {
      "ohos.permission.WRITE_ACCESSIBILITY_CONFIG",
  };

  if (argc < 2) {
    fprintf(stderr, "usage: accessibility-cli-token <program> [args...]\n");
    return 2;
  }

  void *native_token = open_first(native_token_libraries);
  void *setproc = open_first(setproc_libraries);
  if (native_token == NULL || setproc == NULL) {
    fprintf(stderr, "accessibility-cli-token: required access-token libraries are unavailable: %s\n",
            dlerror());
    return 1;
  }

  GetAccessTokenIdFn get_access_token_id = NULL;
  SetSelfTokenIdFn set_self_token_id = NULL;
  *(void **)(&get_access_token_id) =
      required_symbol(native_token, "GetAccessTokenId");
  *(void **)(&set_self_token_id) =
      required_symbol(setproc, "SetSelfTokenID");
  if (get_access_token_id == NULL || set_self_token_id == NULL) {
    return 1;
  }

  NativeTokenInfoParams token_info = {
      .dcaps_num = 0,
      .perms_num = 1,
      .acls_num = 0,
      .dcaps = NULL,
      .perms = permissions,
      .acls = NULL,
      .process_name = "accessibility_ci",
      .apl = "system_core",
      .uid = 0,
  };
  uint64_t token_id = get_access_token_id(&token_info);
  if (token_id == 0) {
    fprintf(stderr, "accessibility-cli-token: GetAccessTokenId failed\n");
    return 1;
  }
  int set_token_status = set_self_token_id(token_id);
  if (set_token_status != 0) {
    fprintf(stderr, "accessibility-cli-token: SetSelfTokenID failed: %d\n",
            set_token_status);
    return 1;
  }

  execv(argv[1], &argv[1]);
  fprintf(stderr, "accessibility-cli-token: execv(%s): %s\n", argv[1],
          strerror(errno));
  return 1;
}
