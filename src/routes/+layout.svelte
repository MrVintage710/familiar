<script lang="ts">
  import PageState from '$lib/page/PageState.svelte';
  import Navbar from '@components/navbar/Navbar.svelte';
  import { onMount } from 'svelte';

  let { children } = $props();
  import("../style.css")
  
  onMount(() => {
    // window.addEventListener("beforeunload", PageState.savePages)
    PageState.init()
    // return () => {
    //   window.removeEventListener("beforeunload", PageState.savePages)
    // }
  })
</script>


<div class="w-screen h-screen grid grid-cols-[auto_1fr] items-stretch overflow-x-clip">
	<Navbar />
  {@render children()}
</div>