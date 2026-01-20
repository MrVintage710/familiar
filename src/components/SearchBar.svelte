<script lang="ts">
    import { debounceOnInput } from '$lib/util.svelte';
    import {Skull, Search} from '@lucide/svelte';
    let searchText = $state("")
    
    let props = $props();
    
    const oninput = debounceOnInput((value) => {console.log(value)}, 300)
</script>

<div class={[props.class, "h-12 preset-outlined-primary-500 rounded-2xl flex items-center px-2 gap-2"]}>
    <Search/>
    <input 
        {oninput}
        type="text" 
        class="input-ghost" 
        placeholder="Search..."
    />
</div>

{searchText}