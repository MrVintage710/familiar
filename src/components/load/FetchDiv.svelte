<script lang="ts" module>
  import { Progress } from '@skeletonlabs/skeleton-svelte';
  import { onMount, type Snippet } from 'svelte';
  import type { HTMLAttributes } from 'svelte/elements';
  
  export type Props<T> = HTMLAttributes<HTMLDivElement> & {
    callback : () => Promise<T>,
    done : Snippet<[T]>,
    loading? : Snippet,
    fail? : Snippet,
  };
</script>

<script lang="ts" generics="T">
  const {callback, loading, fail, done, ...otherProps} : Props<T> = $props();
  
  let result : T | null = $state(null);
  let error : any | null = $state(null);
  let isLoading = $state(false);
  
  $effect(() => {
    if(result === null) {
      isLoading = true;
      callback()
        .then(r => result = r)
        .catch(e => error = e)
        .finally(() => isLoading = false)
    }
  })
</script>

<div {...otherProps} >
  {#if isLoading}
    {#if loading}
      {@render loading()}
    {:else}
      <div class="w-full h-full flex items-center justify-center">
        <Progress class="items-center w-fit" value={null}>
         	<Progress.Circle>
        		<Progress.CircleTrack />
        		<Progress.CircleRange />
         	</Progress.Circle>
         	<Progress.ValueText />
        </Progress>
      </div>
    {/if}
  {/if}
  {#if error}
    {#if fail}
      {@render fail()}
    {:else}
      <p>There was a failure: {error}</p>
    {/if}
  {:else}
    {#if result}
      {@render done(result)}
    {/if}
  {/if}
</div>