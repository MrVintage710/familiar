<script lang="ts">
  import { SkullIcon, BikeIcon, BookIcon, HouseIcon, TreePalmIcon} from "@lucide/svelte";
  import {Navigation} from "@skeletonlabs/skeleton-svelte"
  
  let { children } = $props();
  
  const links = [
		{ label: 'Home', href: '/#', icon: HouseIcon },
		{ label: 'Entertainment', href: '/#', icon: BookIcon },
		{ label: 'Recreation', href: '/#', icon: BikeIcon },
		{ label: 'Relaxation', href: '/#', icon: TreePalmIcon },
	];
</script>

{#snippet NavInner()}
	<Navigation.Header>
	  <Navigation.TriggerAnchor href="/#" title="View Homepage" aria-label="View Homepage">
			<SkullIcon class="size-8" />
		</Navigation.TriggerAnchor>
		<Navigation.Menu class="grid grid-cols-4 gap-2">
			{#each links as link (link)}
				{@const Icon = link.icon}
				<Navigation.TriggerAnchor href={link.href}>
					<Icon class="size-5" />
					<Navigation.TriggerText>{link.label}</Navigation.TriggerText>
				</Navigation.TriggerAnchor>
			{/each}
		</Navigation.Menu>
	</Navigation.Header>
{/snippet}

<div class="w-screen h-screen grid grid-rows-[1fr_auto] border border-surface-200-800">
  {@render children()}
  
  <Navigation layout="bar" class="md:hidden">
    {@render NavInner()}
  </Navigation>
</div>