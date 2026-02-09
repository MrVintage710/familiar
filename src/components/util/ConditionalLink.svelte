<script lang="ts">
  import type { HTMLAnchorAttributes } from "svelte/elements";
  const { children, href, ...props } : HTMLAnchorAttributes = $props()
</script>

{#if href}
	<a {...props} href={href}>
	  {@render children?.()}
	</a>
{:else}
  {@render children?.()}
{/if}