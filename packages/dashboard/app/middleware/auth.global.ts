type AuthenticationMiddlewareOptions =
  | false
  | {
      only?: "guest" | "user";
    };

declare module "#app" {
  interface PageMeta {
    auth?: AuthenticationMiddlewareOptions;
  }
}

declare module "vue-router" {
  interface RouteMeta {
    auth?: AuthenticationMiddlewareOptions;
  }
}

export default defineNuxtRouteMiddleware(async (to) => {
  const options = to.meta?.auth;
  if (!options) {
    return;
  }

  const session = import.meta.client
    ? await getAuthSession().then(({ session }) => ({ value: session }))
    : await useAuthSession().then(({ session }) => session);
  const loggedIn = !!session.value;

  if (options.only === "guest" && loggedIn && to.path !== "/") {
    return navigateTo("/");
  }

  if (!loggedIn && options.only !== "guest") {
    return navigateTo("/auth/login");
  }
});
