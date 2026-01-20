<script lang="ts">
    import Navbar from '@components/Navbar.svelte';

  
  
  let { children } = $props();
  import("../style.css")
</script>


<Navbar>
  {@render children()}
</Navbar>