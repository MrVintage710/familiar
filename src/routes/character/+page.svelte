<script lang="ts" module>
  import SearchBar from "@components/SearchBar.svelte";
  import RulesetCard from "@components/RulesetCard.svelte";
  import WaitDiv from "@components/load/WaitDiv.svelte";
  import CharacterSelectionPage, { CHARACTER_PAGE_UUID } from "$lib/page/CharacterSelectionPage.svelte";
  import { getPage } from "$lib/page/PageState.svelte";
</script>

<script lang="ts">
  let page : CharacterSelectionPage = getPage(CHARACTER_PAGE_UUID) as CharacterSelectionPage;
  console.log(page)
  
  let results = $state([]);
</script>

<div class="w-full flex flex-col ">
  <div class="flex gap-4 justify-center items-center p-4">
    <SearchBar items={[]} bind:results={results} class="w-full"/>
  </div>
  <!-- Character List -->
  <WaitDiv value={page.data.rulesets} class="w-full h-full px-4 pb-4 flex justify-center">
    {#snippet done(rulesets)}
      {#each rulesets as ruleset}
        <RulesetCard {ruleset}/>
      {/each}
    {/snippet}
  </WaitDiv>
</div>