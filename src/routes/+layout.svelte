<script lang="ts">
  import AppShell from '@components/AppShell.svelte';

  let { children } = $props();
  import("../style.css")
</script>


<AppShell>
  {@render children()}
</AppShell>