<template>
  <div>
    <v-breadcrumbs :items="breadcrumbs" />

    <v-card :loading="versions === undefined">
      <v-card-title>
        <slot name="title" :id="id">
          {{ $t("audit.title") }}
        </slot>
      </v-card-title>

      <v-card-subtitle>
        <slot name="subtitle" :id="id" />
      </v-card-subtitle>

      <v-card-text class="mt-4" v-if="versions">
        <slot :versions="versions.auditVersions">
          <v-expansion-panels>
            <v-expansion-panel
              color="primary"
              v-for="version in versions.auditVersions"
              :key="version.id"
            >
              <template #title>
                <v-row>
                  <v-col cols="6">
                    <span class="mr-2">
                      {{
                        new Date(
                          version.createdAt.seconds * 1000,
                        ).toLocaleString()
                      }}
                    </span>
                    -
                    <span :class="`${actionColor(version.action)} ml-2`">
                      {{ $t(`audit.action.${version.action}`) }}
                    </span>
                  </v-col>

                  <v-col cols="6">
                    <UserInlined :username="version.madeBy.username" />
                  </v-col>
                </v-row>
              </template>

              <v-expansion-panel-text>
                <AuditEntry
                  @rollback="onRollback"
                  :entry="version"
                  :endpoint="endpoint"
                />
              </v-expansion-panel-text>
            </v-expansion-panel>
          </v-expansion-panels>
        </slot>

        <v-pagination
          class="mt-4"
          v-if="versions && versions.total > versions.pageSize"
          v-model="pageToken"
          :length="Math.ceil(versions.total / versions.pageSize)"
        />
      </v-card-text>
    </v-card>
  </div>
</template>

<script setup lang="ts">
import {
  onMounted,
  useRouter,
  ref,
  watch,
  useNuxtApp,
  useRoute,
} from "#imports";
import { useAuthorizationStore } from "~/composables/authorization";
import { useRpcOptions } from "~/composables/grpc";
import { ProtoAuditAction } from "~/composables/feedback-fusion-v1/audit";

const props = defineProps({
  kind: Number,
  endpoint: String,
  id: String,
  breadcrumbs: Array,
});

const { $feedbackFusion } = useNuxtApp();
const authorization = useAuthorizationStore();
const router = useRouter();
const route = useRoute();
const versions = ref(undefined);
const pageToken = ref(1);

const fetchPage = async (pageToken: number) => {
  versions.value = await $feedbackFusion
    .getAuditVersions(
      {
        pageToken,
        pageSize: 10,
        resourceId: props.id,
        resourceType: props.kind,
      },
      await useRpcOptions(),
    )
    .then((value) => value.response);
};

watch(
  () => pageToken.value,
  async (pageToken: number) => {
    await fetchPage(pageToken);
  },
);

onMounted(async () => {
  if (!authorization.hasPermission(props.endpoint, "Read")) {
    return router.push("/");
  }

  await fetchPage(pageToken.value);
});

const actionColor = (action: number) => {
  switch (action) {
    case ProtoAuditAction.CREATE:
      return "text-success";
    case ProtoAuditAction.UPDATE:
      return "text-warning";
    case ProtoAuditAction.DELETE:
      return "text-danger";
    case ProtoAuditAction.ROLLBACK:
      return "text-warning";
    default:
      return "text-danger";
  }
};

// on rollback we want to get back to the modified resource and therefore have to leave the audit log
const onRollback = () => {
  const segments = route.path.split("/");
  // remove the last segment
  segments.pop();

  // for fields we have to pop another 2
  if (props.endpoint == "Field") {
    segments.pop();
    segments.pop();
  }

  router.push(segments.join("/"));
};
</script>
