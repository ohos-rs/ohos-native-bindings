// QEMU-only helper for enabling an installed accessibility extension with the
// system permission required by AccessibilityConfig::EnableAbility.
#include <dlfcn.h>

#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <string>

struct NativeTokenInfoParams {
  int32_t dcaps_num;
  int32_t perms_num;
  int32_t acls_num;
  const char **dcaps;
  const char **perms;
  const char **acls;
  const char *process_name;
  const char *apl;
  int32_t uid;
};

using GetAccessTokenIdFn = uint64_t (*)(NativeTokenInfoParams *token_info);
using SetSelfTokenIdFn = int (*)(uint64_t token_id);
using GetAccessibilityConfigFn = void *(*)();
using EnableAbilityFn = int32_t (*)(void *config, const std::string &name,
                                    uint32_t capabilities,
                                    bool connect_callback);

static void *open_first(const char *const libraries[]) {
  for (size_t index = 0; libraries[index] != nullptr; ++index) {
    void *handle = dlopen(libraries[index], RTLD_NOW | RTLD_LOCAL);
    if (handle != nullptr) {
      return handle;
    }
  }
  return nullptr;
}

static void *required_symbol(void *handle, const char *name) {
  dlerror();
  void *symbol = dlsym(handle, name);
  const char *error = dlerror();
  if (error != nullptr) {
    std::fprintf(stderr, "accessibility-enable: dlsym(%s): %s\n", name,
                 error);
    return nullptr;
  }
  return symbol;
}

int main(int argc, char *argv[]) {
  static const char *const native_token_libraries[] = {
      "libnativetoken_shared.z.so", "libnativetoken_shared.so",
      "libnativetoken.z.so", "libnativetoken.so", nullptr};
  static const char *const setproc_libraries[] = {
      "libtokensetproc_shared.z.so", "libtokensetproc_shared.so",
      "libtoken_setproc.z.so", "libtoken_setproc.so", nullptr};
  static const char *const accessibility_config_libraries[] = {
      "libaccessibilityconfig.z.so", "libaccessibility_config.so", nullptr};
  static const char *permissions[] = {
      "ohos.permission.WRITE_ACCESSIBILITY_CONFIG"};

  if (argc != 2) {
    std::fprintf(stderr,
                 "usage: accessibility-enable <bundle-name/ability-name>\n");
    return 2;
  }

  void *native_token = open_first(native_token_libraries);
  void *setproc = open_first(setproc_libraries);
  void *accessibility_config = open_first(accessibility_config_libraries);
  if (native_token == nullptr || setproc == nullptr ||
      accessibility_config == nullptr) {
    std::fprintf(stderr,
                 "accessibility-enable: required platform libraries are "
                 "unavailable: %s\n",
                 dlerror());
    return 1;
  }

  auto get_access_token_id = reinterpret_cast<GetAccessTokenIdFn>(
      required_symbol(native_token, "GetAccessTokenId"));
  auto set_self_token_id = reinterpret_cast<SetSelfTokenIdFn>(
      required_symbol(setproc, "SetSelfTokenID"));
  auto get_accessibility_config = reinterpret_cast<GetAccessibilityConfigFn>(
      required_symbol(
          accessibility_config,
          "_ZN4OHOS19AccessibilityConfig19AccessibilityConfig11GetInstanceEv"));
  auto enable_ability = reinterpret_cast<EnableAbilityFn>(required_symbol(
      accessibility_config,
      "_ZN4OHOS19AccessibilityConfig19AccessibilityConfig13EnableAbilityERKNSt3__h12basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEjb"));
  if (get_access_token_id == nullptr || set_self_token_id == nullptr ||
      get_accessibility_config == nullptr || enable_ability == nullptr) {
    return 1;
  }

  NativeTokenInfoParams token_info{};
  token_info.perms_num = 1;
  token_info.perms = permissions;
  token_info.process_name = "accessibility_e2e";
  token_info.apl = "system_core";
  token_info.uid = 0;
  uint64_t token_id = get_access_token_id(&token_info);
  if (token_id == 0) {
    std::fprintf(stderr, "accessibility-enable: GetAccessTokenId failed\n");
    return 1;
  }
  int set_token_status = set_self_token_id(token_id);
  if (set_token_status != 0) {
    std::fprintf(stderr, "accessibility-enable: SetSelfTokenID failed: %d\n",
                 set_token_status);
    return 1;
  }

  constexpr uint32_t kAllCapabilities = 31;
  std::string ability_name(argv[1]);
  int32_t status = enable_ability(get_accessibility_config(), ability_name,
                                  kAllCapabilities, false);
  std::printf("EnableAbility(%s) returned %d\n", ability_name.c_str(), status);
  return status == 0 ? 0 : 1;
}
