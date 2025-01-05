<script setup lang="ts">
import { useOidcAuth } from "#imports";

const { logout } = useOidcAuth();

logout();
</script>
