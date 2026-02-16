<script lang="ts">
    import { debounceOnInput } from '$lib/debounce.svelte';
    import type { Searchable } from '$lib/search';
    import search from '$lib/search';
    import {Search} from '@lucide/svelte';
    
    type Props = { 
      items : Searchable[],
      results : Searchable[],
      debounce? : number,
      class? : string
    }
    
    let { items, results = $bindable([]), debounce = 300, 'class' : clazz} : Props = $props();
    let tags : string[] = $state([])
    
    // svelte-ignore state_referenced_locally
    const oninput = debounceOnInput((value) => {
      results = search(items, value, tags)
    }, debounce)
</script>

<div class={[clazz, "h-12 container preset-outlined-primary-500 rounded-2xl flex items-center px-2 gap-2"]}>
    <Search class="stroke-primary-500"/>
    <input 
        {oninput}
        type="text" 
        class="input-ghost w-full" 
        placeholder="Search..."
    />
</div>