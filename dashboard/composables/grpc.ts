import type { RpcOptions } from "@protobuf-ts/runtime-rpc";

export const useRpcOptions = (): RpcOptions => {
  const { user } = useOidcAuth();

  return {
    // https://github.com/timostamm/protobuf-ts/blob/main/MANUAL.md#rpc-options
    interceptors: [
      {
        interceptUnary(next, method, input, options) {
          if (!options.meta) {
            options.meta = {};
          }

          options.meta["Authorization"] = `Bearer ${user.value!.accessToken}`;
          return next(method, input, options);
        },
      },
    ],
  };
};
