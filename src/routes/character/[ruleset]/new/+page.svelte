<script lang="ts">
  import { page } from "$app/state";
    import { getConstructorsForRuleset } from "$lib/rulebook";
    import WaitDiv from "@components/load/WaitDiv.svelte";
    import { onMount } from "svelte";

  let { ruleset = "" } = page.params;
  
  let characterConstructor = $state();
  
  onMount(() => getConstructorsForRuleset(ruleset).then(data => characterConstructor = data))
</script>

<WaitDiv value={ruleset} class="w-full h-full">
  {#snippet done(ruleset)}
    {JSON.stringify(ruleset)}
  {/snippet}
</WaitDiv>