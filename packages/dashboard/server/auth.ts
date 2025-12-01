import { betterAuth } from "better-auth";
import { genericOAuth, username } from "better-auth/plugins";

export const auth = betterAuth({
  session: {
    expiresIn: 60 * 60 * 24 * 7,
    cookieCache: {
      enabled: true,
      maxAge: 30 * 24 * 60 * 60,
      strategy: "jwt",
      refreshCache: true,
    },
  },
  plugins: [
    username(),
    genericOAuth({
      config: [
        {
          providerId: "oidc",
          clientId: useRuntimeConfig().clientId,
          clientSecret: useRuntimeConfig().clientSecret,
          discoveryUrl: useRuntimeConfig().oidcDiscovery,
          scopes: useRuntimeConfig().scope.split(" "),
        },
      ],
    }),
  ],
});
