<script setup lang="ts">
import { oidcClient } from "~/composables/authorization";

oidcClient.signOut();
</script>
