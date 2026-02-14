<script lang="ts" module>
    import { Progress } from '@skeletonlabs/skeleton-svelte';
  import type { Snippet } from 'svelte';
  import type { HTMLAttributes } from 'svelte/elements';

  export type WaitDivProps<T> = HTMLAttributes<HTMLDivElement> & {
    done : Snippet<[T]>;
    value? : T;
    loading? : Snippet;
  }
</script>

<script lang="ts" generics="T">
  const {value, done, loading, 'class': classes, ...props} : WaitDivProps<T> = $props()
</script>

<div {...props} class={[classes, "relative"]}>
  {#if value}
    {@render done(value)}
  {:else}
    {#if loading}
      {@render loading()}
    {:else}
      <Progress class="items-center w-fit absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2" value={null}>
       	<Progress.Circle>
      		<Progress.CircleTrack />
      		<Progress.CircleRange />
       	</Progress.Circle>
       	<Progress.ValueText />
      </Progress>
    {/if}
  {/if}
</div>