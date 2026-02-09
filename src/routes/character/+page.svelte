<script lang="ts" module>
  
</script>

<script lang="ts">
  import SearchBar from "@components/SearchBar.svelte";
  import { Plus } from "@lucide/svelte";
  import { getRulesetList } from "$lib/rulebook";
  import FetchArea from "@components/FetchArea.svelte";
  import RulesetCard from "@components/RulesetCard.svelte";
    import type { Uuid } from "$lib/types";
  
  let results = $state([]);
</script>

<div class="w-full flex flex-col ">
  <div class="flex gap-4 justify-center items-center p-4">
    <SearchBar items={[]} bind:results={results} class="w-full"/>
  </div>
  <!-- Character List -->
  <FetchArea callback={getRulesetList} class="w-full h-full px-4 pb-4 flex justify-center">
    {#snippet done(rulesets)}
      {#each rulesets as ruleset}
        <RulesetCard {ruleset}/>
      {/each}
    {/snippet}
  </FetchArea>
</div>