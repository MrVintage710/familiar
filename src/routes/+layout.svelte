<script lang="ts">
  import { initNavstate } from '$lib/page/PageState.svelte';
  import Navbar from '@components/navbar/Navbar.svelte';
  import { onMount } from 'svelte';

  let { children } = $props();
  import("../style.css")
  
  onMount(initNavstate)
</script>


<div class="w-full h-screen grid grid-cols-[auto_1fr] items-stretch">
	<Navbar />
  {@render children()}
</div>