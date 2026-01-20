<script lang="ts">
    import { debounceOnInput } from '$lib/debounce.svelte';
    import type { Searchable } from '$lib/search';
    import search from '$lib/search';
    import {Search} from '@lucide/svelte';
    
    type Props = { 
      items : Searchable[],
      debounce? : number,
      class? : string
    }
    
    let { items = $bindable(), debounce = 300, 'class' : clazz} : Props = $props();
    let tags : string[] = $state([])
    
    // svelte-ignore state_referenced_locally
    const oninput = debounceOnInput((value) => {
      items = search(items, value, tags)
    }, debounce)
</script>

<div class={[clazz, "h-12 preset-outlined-primary-500 rounded-2xl flex items-center px-2 gap-2"]}>
    <Search/>
    <input 
        {oninput}
        type="text" 
        class="input-ghost" 
        placeholder="Search..."
    />
</div>