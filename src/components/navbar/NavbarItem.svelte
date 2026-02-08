<script lang="ts" module>
  import { CircleQuestionMark, type IconProps } from "@lucide/svelte";
    import { Portal, Tooltip, type TooltipRootProps } from "@skeletonlabs/skeleton-svelte";
  import type { Component, Snippet } from "svelte";

  export type Props = {
    icon? : Component<IconProps>,
    selected? : boolean,
    tooltipOptions? : TooltipRootProps,
    expanded? : boolean
    title? : string
    subtitle? : string
  }
</script>

<script lang="ts">
  const {
    icon : Icon = CircleQuestionMark,
    selected = false,
    expanded = true,
    title,
    subtitle,
    tooltipOptions
  } : Props = $props()
  const height = () => expanded ? 12 : 32;
</script>

<Tooltip {...tooltipOptions}>
	<Tooltip.Trigger>
    <div class={["flex items-center gap-2 rounded-lg p-1", expanded ? "w-full" : "w-fit", !selected || "border-2 border-primary-500"]}>
      <Icon size={height() + 12} class="stroke-primary-500 shrink-0 grow-0"/>
      {#if expanded}
        <div class="truncate text-ellipsis w-full"> {title} </div>
      {/if}
    </div>
	</Tooltip.Trigger>
	{#if !expanded}
  	<Portal>
  		<Tooltip.Positioner>
  			<Tooltip.Content class="card preset-filled-primary-900-100 p-2 shadow-xl">
  			  {title}
  			</Tooltip.Content>
  		</Tooltip.Positioner>
  	</Portal>
	{/if}
</Tooltip>


<style>
  .text-comp {
      width: calc(100% - var(--height) + 12px);
  }
</style>