<script lang="ts">
  import { page } from "$app/state";
  import type CreateCharacterPage from "$lib/page/CreateCharacterPage.svelte";
  import PageState from "$lib/page/PageState.svelte";
  import ConstructorForm from "@components/constructor/ConstructorForm.svelte";
  import WaitDiv from "@components/load/WaitDiv.svelte";
  
  let tab : CreateCharacterPage = PageState.currentPage as CreateCharacterPage;
</script>

<!-- <div class="absolute top-2 right-2 bg-surface-900 z-50 p-2 max-w-3/4">{JSON.stringify($state.snapshot(tab?.state), null, 4)}</div> -->
<WaitDiv value={tab.state.constructorId} class="w-full h-full">
  {#snippet done(constructorUuid)}
    <ConstructorForm constructor={constructorUuid}/>
  {/snippet}
</WaitDiv>