import type { RpcOptions } from "@protobuf-ts/runtime-rpc";

export const useRpcOptions = async (): Promise<RpcOptions> => {
  const store = useAuthorizationStore();
  const token = await store.getAccessToken();

  return {
    // https://github.com/timostamm/protobuf-ts/blob/main/MANUAL.md#rpc-options
    interceptors: [
      {
        interceptUnary(next, method, input, options) {
          if (!options.meta) {
            options.meta = {};
          }

          options.meta["Authorization"] = `Bearer ${token}`;
          return next(method, input, options);
        },
      },
    ],
  };
};
