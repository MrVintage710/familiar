<script lang="ts" module>
  import { gotoPage, type Page } from "$lib/PageState.svelte";
  import type { Uuid } from "$lib/types";
  import type { HTMLAttributes } from "svelte/elements";

  export type Props = HTMLAttributes<HTMLSpanElement> & {
    page : Page,
  }
</script>

<script lang="ts">
  const {
    page,
    children,
    ...props
  } : Props = $props()
</script>

<span {...props} class="hover:cursor-pointer" data-href={page.url} tabindex="0" role="link" onclick={() => gotoPage(page.id)}>
  {@render children?.()}
</span>
