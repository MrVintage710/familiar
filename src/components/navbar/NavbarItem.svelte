<script lang="ts" module>
  import { Portal, Tooltip, type TooltipRootProps } from "@skeletonlabs/skeleton-svelte";
  import { getCurrentPage, gotoPage, removePage, type Page } from "$lib/PageState.svelte";
  import PageLink from "./PageLink.svelte";

  export type Props = {
    selected? : boolean,
    tooltipOptions? : TooltipRootProps,
    expanded? : boolean
    linkedPage : Page
  }
</script>

<script lang="ts">
    import { X } from "@lucide/svelte";
    import { render } from "svelte/server";


  const {
    selected = false,
    expanded = true,
    tooltipOptions,
    linkedPage
  } : Props = $props()
  const height = () => expanded ? 12 : 32;
</script>


{#snippet interior()}
  <PageLink page={linkedPage}>
     <div class={["relative flex items-center gap-2 rounded-lg p-1 border-2", expanded ? "w-full" : "w-fit", (selected || getCurrentPage()?.id === linkedPage.id) ? "border-primary-500" : "border-transparent"]}>
       <linkedPage.icon size={height() + 12} class="stroke-primary-500 shrink-0 grow-0"/>
       {#if expanded}
         <div class="text-left truncate text-ellipsis w-full"> {linkedPage.title} </div>
         <button class="right-2 hidden-child h-full hover:bg-primary-contrast-500/75" onclick={() => removePage(linkedPage.id)}><X size={16} class="stroke-primary-500"/></button>
       {/if}
     </div>
 	</PageLink>
{/snippet}


{#if expanded}
	{@render interior()}
{:else}
  <Tooltip {...tooltipOptions}>
  	<Tooltip.Trigger>
   	{@render interior()}
  	</Tooltip.Trigger>
   	<Portal>
    		<Tooltip.Positioner>
          <Tooltip.Content class="card preset-filled-primary-900-100 p-2 shadow-xl">
   			  {linkedPage.title}
   			</Tooltip.Content>
    		</Tooltip.Positioner>
   	</Portal>
  </Tooltip>
{/if}


<style>
  *>.hidden-child {
    display: none;  
  }
  
  *:hover>.hidden-child {
    display: block;
  }
</style>