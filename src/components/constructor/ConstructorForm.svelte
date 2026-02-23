<script lang="ts" module>
  import type { Constructor, Uuid } from "$lib/types";
    import TitleSection from "@components/TitleSection.svelte";

  export type ConstructorProps = {
    constructor : Uuid
  }
</script>

<script lang="ts">
  const {constructor} : ConstructorProps = $props();
  
  let currentStep = $state(0);
</script>

<div class="w-full flex flex-col items-center p-4 gap-4">
    <span class="text-4xl font-taroca">Create Character</span>
    <!-- {#each Object.entries(constructor.steps) as [stepTitle, step]}
        <TitleSection title={stepTitle}>
          <form action="">
            {#each Object.entries(step) as [fieldName, field]}
              {fieldName}
            {/each}
          </form>
        </TitleSection>
    {/each} -->
</div>