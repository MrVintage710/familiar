<script lang="ts" module>
  import NavbarItem from "./NavbarItem.svelte";
  import PageState from "$lib/page/PageState.svelte";
  import { onMount } from "svelte";
  
  export type Props = {
    width? : number,
    minWidth? : number,
    maxWidth? : number,
  }
</script>

<script lang="ts">

  let {
    width = $bindable(240),
    minWidth = 130,
    maxWidth = 310,
  } : Props = $props()
  
  //==============================================================================================
  //        Dragging Code
  //==============================================================================================
  
  let isDragging = $state(false);

  function stopDragging(event : MouseEvent) {
    isDragging = false;
    event.stopPropagation()
    event.preventDefault()
    document.body.style.cursor = "";
  }
  
  function startDrag(event : MouseEvent) {
    isDragging = true
    event.stopPropagation()
    event.preventDefault()
    document.body.style.cursor = "ew-resize";
  }
  
  function handle(event : MouseEvent) {
    if(!isDragging) return;
    event.stopPropagation()
    event.preventDefault()
    width = Math.max(Math.min(event.clientX, maxWidth), minWidth);
  }
</script>

<svelte:window onmousemove={handle} onmouseup={stopDragging} />

<nav class="wrapper relative h-full bg-surface-900 p-2 flex flex-col gap-2" style="--width: {width}px; --max-w: {maxWidth}px; --min-w: {minWidth}px">
  <div
    role="separator"
    aria-orientation="vertical"
    aria-label="Resize sidebar"
    onmousedown={startDrag}
    class={["absolute right-0 h-full hover:border-r-2 hover:border-secondary-500 w-2.5 -mt-2 hover:cursor-ew-resize focus:outline-none", isDragging && "border-r-2 border-secondary-500 cursor-ew-resize"]}
  />
  <div class="flex flex-wrap justify-start gap-1 h-fit border-b-2 border-dashed border-primary-500 pb-4 w-full">
    {#each PageState.shortcuts as shortcut (shortcut.id) }
      <NavbarItem linkedPage={shortcut} expanded={false} />
    {/each}
  </div>
  {#each PageState.pages as page (page.id)}
    <NavbarItem linkedPage={page} />
  {/each}
</nav>

<!-- {#snippet Trigger(title : string, icon : Component<IconProps>)}
	<Tabs.Trigger value={title} class="z-50 p-2"><NavbarItem title={title} icon={ icon } /></Tabs.Trigger>
{/snippet}

<Tabs defaultValue="Characters" orientation="vertical" class="w-screen">
  <Tabs.List class="bg-surface-900 p-2">
    {@render Trigger("Characters", Users)}
    {@render Trigger("Sources", BookCopy)}
    <div class="border-t border-primary-500 w-full"></div>
    <Tabs.Indicator class="bg-primary-500">   </Tabs.Indicator>
  </Tabs.List>
  <Tabs.Content value="Characters" class="w-full h-full">
		A concise overview of the project: usage, goals, and recent highlights. Use this area to orient readers with key metrics and links to
		deeper docs.
	</Tabs.Content>
</Tabs> -->

<style>
  .wrapper {
      width: var(--width);
      min-width: var(--min-w);
      max-width: var(--max-w);
  }
</style>