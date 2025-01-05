interface PermissionMatrix {
  [key: string]: boolean;
}

const { $feedbackFusion } = useNuxtApp();

export const useAuthorizationStore = defineStore("authorization", () => {
  const permissions = ref({} as PermissionMatrix);

  async function fetch() {
    permissions.value = await $feedbackFusion
      .getUserInfo({})
      .then((value) => value.response.permissions);
  }

  async function canAccess(endpoint: string, action: string): Promise<boolean> {
    if (Object.keys(permissions.value).length === 0) {
      await fetch();
    }

    return permissions.value[`${endpoint}::${action}`] || false;
  }

  return {
    permissions,
    fetch,
    canAccess,
  };
});
