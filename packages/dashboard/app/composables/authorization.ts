import { createAuthClient } from "better-auth/vue";
import { genericOAuthClient } from "better-auth/client/plugins";

interface PermissionMatrix {
  [key: string]: boolean;
}

export const useAuthorizationStore = defineStore("authorization", () => {
  const permissions = ref({} as PermissionMatrix);
  const session = ref<
    Awaited<ReturnType<typeof oidcClient.useSession>>["data"] | undefined
  >();
  const accessToken = ref<string | undefined>();
  const loggedIn = computed(() => !!session.value);

  async function fetchPermissions() {
    const { $feedbackFusion } = useNuxtApp();

    await $feedbackFusion
      .getUserInfo({}, await useRpcOptions())
      .then((value: any) => {
        permissions.value = value.response.permissions;
      });
  }

  async function fetchSession() {
    await oidcClient.useSession(useFetch).then(({ data }) => {
      session.value = data.value;
    });
  }

  async function fetchAccessToken() {
    await oidcClient.getAccessToken({ providerId: "oidc" }).then(({ data }) => {
      accessToken.value = data?.accessToken;
    });
  }

  async function hasPermission(
    endpoint: string,
    action: string,
  ): Promise<boolean> {
    if (Object.keys(permissions.value).length === 0) {
      await fetchPermissions();
    }
    return permissions.value[`${endpoint}::${action}`] ?? false;
  }

  function logout() {
    permissions.value = {};
    session.value = undefined;
    accessToken.value = undefined;
  }

  async function getAccessToken() {
    if (!accessToken.value) await fetchAccessToken();
    return accessToken.value;
  }

  async function getSession() {
    if (!session.value) await fetchSession();
    return session.value;
  }

  return {
    permissions: readonly(permissions),
    loggedIn,
    accessToken: readonly(accessToken),
    fetchPermissions,
    fetchSession,
    fetchAccessToken,
    hasPermission,
    logout,
    getAccessToken,
    getSession,
  };
});

export const oidcClient = createAuthClient({
  plugins: [genericOAuthClient()],
});
