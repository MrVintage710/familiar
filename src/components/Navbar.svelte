<script lang="ts">
  import { ArrowLeftRightIcon, BikeIcon, BookIcon, HouseIcon, TreePalmIcon, UserIcon } from '@lucide/svelte';
	import { Navigation } from '@skeletonlabs/skeleton-svelte';
	import { page } from '$app/state';
  
	const links = [
		{ label: 'Characters', href: '/', icon: UserIcon },
	];
  
	const buttonClasses = 'btn hover:preset-tonal';
	let anchorRail = `${buttonClasses} aspect-square w-full max-w-[84px] flex flex-col items-center gap-0.5`;
	let anchorSidebar = `${buttonClasses} justify-start px-2 w-full`;
  
	let layoutRail = $state(true);
  
	function toggleLayout() {
		layoutRail = !layoutRail;
	}
  
  let { children } = $props();
</script>

<div class="w-full h-screen grid grid-cols-[auto_1fr] items-stretch">
  <Navigation layout={layoutRail ? 'rail' : 'sidebar'} class={[layoutRail ? '' : 'grid grid-rows-[1fr_auto] gap-4', '']}>
		<Navigation.Content>
			<Navigation.Header>
				<Navigation.Trigger onclick={toggleLayout}>
					<ArrowLeftRightIcon class={layoutRail ? 'size-5' : 'size-4'} />
					{#if !layoutRail}<span>Resize</span>{/if}
				</Navigation.Trigger>
			</Navigation.Header>
			<Navigation.Menu>
				{#each links as link (link)}
					{@const Icon = link.icon}
					<Navigation.TriggerAnchor class={[page.route.id === link.href ? "outline-primary-500 outline" : ""]}>
						<Icon class={layoutRail ? 'size-5' : 'size-4'} />
						<Navigation.TriggerText>{link.label}</Navigation.TriggerText>
					</Navigation.TriggerAnchor>
				{/each}
			</Navigation.Menu>
		</Navigation.Content>
	</Navigation>
  {@render children()}
</div>