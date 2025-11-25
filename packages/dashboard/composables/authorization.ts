interface PermissionMatrix {
  [key: string]: boolean;
}

export const useAuthorizationStore = defineStore("authorization", () => {
  const permissions = ref({} as PermissionMatrix);

  async function fetch() {
    const { $feedbackFusion } = useNuxtApp();

    permissions.value = await $feedbackFusion
      .getUserInfo({}, useRpcOptions())
      .then((value) => value.response.permissions);
  }

  function hasPermission(endpoint: string, action: string): boolean {
    if (Object.keys(permissions.value).length === 0) {
      fetch();
    }

    return permissions.value[`${endpoint}::${action}`] || false;
  }

  return {
    permissions,
    fetch,
    hasPermission,
  };
});
