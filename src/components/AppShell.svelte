<script lang="ts">
  import Navbar from './navbar/Navbar.svelte';
  
  let { children } = $props();
</script>

<div class="w-full h-screen grid grid-cols-[auto_1fr] items-stretch">
	<Navbar />
  {@render children()}
</div>