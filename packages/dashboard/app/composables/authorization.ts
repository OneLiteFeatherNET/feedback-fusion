import { createAuthClient } from "better-auth/vue";
import { genericOAuthClient } from "better-auth/client/plugins";

interface PermissionMatrix {
  [key: string]: boolean;
}

export const useAuthorizationStore = defineStore("authorization", () => {
  const fetchedPermissions = ref(false);
  const permissions = ref({} as PermissionMatrix);
  const session = ref(undefined);
  const accessToken = ref<string | undefined>();

  async function fetchPermissions() {
    const { $feedbackFusion } = useNuxtApp();

    await $feedbackFusion
      .getUserInfo({}, await useRpcOptions())
      .then((value: any) => {
        permissions.value = value.response.permissions;
        fetchedPermissions.value = true;
      });
  }

  async function fetchSession() {
    await useAuthSession().then((value) => {
      session.value = value.session.value;
    });
  }

  async function fetchAccessToken() {
    await useOIDCClient()
      .getAccessToken({ providerId: "oidc" })
      .then(({ data }) => {
        accessToken.value = data?.accessToken;
      });
  }

  async function hasPermission(
    endpoint: string,
    action: string,
  ): Promise<boolean> {
    if (!fetchedPermissions) {
      await fetchPermissions();
    }
    return permissions.value[`${endpoint}::${action}`] ?? false;
  }

  function logout() {
    permissions.value = {};
    session.value = undefined;
    accessToken.value = undefined;
    fetchedPermissions.value = false;
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

export const useOIDCClient = () =>
  createAuthClient({
    plugins: [genericOAuthClient()],
    baseURL: useRequestURL().origin,
  });

// A workaround for https://github.com/better-auth/better-auth/issues/5358
// Create a wrapper for useFetch that strips the origin from URLs
// This fixes the SSR hydration issue with BetterAuth and Nuxt
const relativeFetch = ((url: string, opts?: any) => {
  try {
    if (url.startsWith("http")) {
      url = new URL(url).pathname;
    }
  } catch {}
  return useFetch(url, opts);
}) as any;

export async function useAuthSession() {
  const { data, isPending, error } =
    await useOIDCClient().useSession(relativeFetch);

  return {
    session: data, // rename data to session, should no long be undefined
    isPending,
    error,
  };
}

export async function getAuthSession() {
  const { data, isPending, error } =
    await useOIDCClient().getSession(relativeFetch);

  return {
    session: data, // rename data to session, should no long be undefined
    isPending,
    error,
  };
}
