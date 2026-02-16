<script lang="ts" module>
  import type { Constructor } from "$lib/types";
    import TitleSection from "@components/TitleSection.svelte";

  export type ConstructorProps = {
    constructor : Constructor
  }
</script>

<script lang="ts">
  const {constructor} : ConstructorProps = $props();
  
  let currentStep = $state(0);
  
  console.log("Form", constructor)
</script>

<div class="w-full flex flex-col items-center p-4 gap-4">
    <span class="text-4xl font-taroca">Create Character</span>
    {#each Object.entries(constructor.steps) as [stepTitle, step]}
        <TitleSection title={stepTitle}></TitleSection>
    {/each}
</div>