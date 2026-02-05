<script lang="ts" module>
  import type { HTMLButtonAttributes } from "svelte/elements";

  export type Props = HTMLButtonAttributes & {}
</script>

<script lang="ts">
  const {children, 'class' : clazz, ...props} : Props = $props();
</script>

<button type="button" class={["btn preset-filled-primary-100-900 h-12", clazz]} {...props}>
  {@render children?.()}
</button>