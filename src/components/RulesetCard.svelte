<script lang="ts" module>
  import type { RulesetInfoWithCovers } from "$lib/types";
    import { Plus } from "@lucide/svelte";
  import Button from "./Button.svelte";

  export type Props = {
    ruleset : RulesetInfoWithCovers
  }
</script>

<script lang="ts">
  const { ruleset } : Props = $props()
  
  const imageUri = () => 'data:image/gif;base64,' + ruleset.covers[0].data
</script>

<div style="--image-data : url({imageUri()})" class="container relative w-full card preset-filled-surface-100-900 flex justify-center p-2 darken">
  <Button class="absolute right-2 drop-shadow-[0_2.2px_2.2px_rgba(0,0,0,0.8)] m-1">
    <Plus class="stroke-primary-500"/>
  </Button>
  <div class="font-taroca text-3xl drop-shadow-[0_2.2px_2.2px_rgba(0,0,0,0.8)] bg-primary-900 p-2 h-fit">
    {ruleset.title}
  </div>
</div>

<style>
  .container {
    background-image: var(--image-data);
    background-size: cover; /* Corresponds to Figma's "Fill" mode */
    background-repeat: no-repeat;
    background-position: center;
    /* Add other properties like width, height, etc. */
  }
</style>