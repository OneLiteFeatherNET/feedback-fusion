import { oidcClient } from "~/composables/authorization";

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

  const store = useAuthorizationStore();
  const session = await store.getSession();
  const loggedIn = !!session;

  if (options.only === "guest" && loggedIn && to.path !== "/") {
    return navigateTo("/");
  }

  if (!loggedIn && options.only !== "guest") {
    return navigateTo("/auth/login");
  }
});
