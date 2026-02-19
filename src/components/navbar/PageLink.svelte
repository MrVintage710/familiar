<script lang="ts" module>
  import type { Page } from "$lib/page/Page.svelte";
  import PageState from "$lib/page/PageState.svelte";
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

<span {...props} class="hover:cursor-pointer" data-href={page.getUrl} tabindex="0" role="link" onclick={() => PageState.gotoPage(page.id)}>
  {@render children?.()}
</span>
