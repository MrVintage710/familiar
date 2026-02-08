<script lang="ts" module>
	import { Portal, Tooltip, type TooltipRootProps } from '@skeletonlabs/skeleton-svelte';
  import type { Snippet } from 'svelte';
  
  export type Props = TooltipRootProps & {
    children? : Snippet<[]>
  }
</script>

<script lang="ts">
  const { children, ...props} : Props = $props() 
</script>

<Tooltip {...props}>
	<Tooltip.Trigger>Hover</Tooltip.Trigger>
	<Portal>
		<Tooltip.Positioner>
			<Tooltip.Content class="card bg-surface-100-900 p-2 shadow-xl">
			  {@render children?.()}
			</Tooltip.Content>
		</Tooltip.Positioner>
	</Portal>
</Tooltip>