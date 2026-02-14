<script lang="ts">
  import { page } from "$app/state";
  import { getConstructorsForRuleset } from "$lib/rulebook";
  import Constructor from "@components/constructor/Constructor.svelte";
  import WaitDiv from "@components/load/WaitDiv.svelte";
  import { onMount } from "svelte";

  let { ruleset = "" } = page.params;
  
  let characterConstructor = $state();
  
  onMount(() => getConstructorsForRuleset(ruleset).then(data => characterConstructor = data))
</script>


<WaitDiv value={characterConstructor} class="w-full h-full overflow-x-scroll">
  {#snippet done(characterConstructor)}
    <Constructor  constructor={characterConstructor}/>
  {/snippet}
</WaitDiv>