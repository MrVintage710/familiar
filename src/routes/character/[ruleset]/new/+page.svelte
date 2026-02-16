<script lang="ts">
  import { page } from "$app/state";
  import { getConstructorsForRuleset } from "$lib/rulebook";
  import ConstructorForm from "@components/constructor/ConstructorForm.svelte";
  import WaitDiv from "@components/load/WaitDiv.svelte";
  import { type Constructor } from "$lib/types";
  import { onMount } from "svelte";

  let { ruleset = "" } = page.params;
  
  let characterConstructor = $state<Constructor>();
  
  onMount(() => getConstructorsForRuleset(ruleset).then(data => characterConstructor = data.find(c => c.meta.name === "Character Creation")))
</script>


<WaitDiv value={characterConstructor} class="w-full h-full overflow-x-scroll">
  {#snippet done(characterConstructor)}
    <ConstructorForm  constructor={characterConstructor}/>
  {/snippet}
</WaitDiv>