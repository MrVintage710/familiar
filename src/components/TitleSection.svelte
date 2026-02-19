<script lang="ts" module>
    import type { HTMLAttributes } from "svelte/elements";

    export type TitleSectionProps = HTMLAttributes<HTMLDivElement> & {
      title : string,
      align? : "left" | "right" | "center"
    }
</script>

<script lang="ts">
    const {title, align = "left", "class" : clazz, children, ...props} : TitleSectionProps = $props()
</script>

<div {...props} class={[clazz, "relative container min-h-20 border-2 border-primary-500 rounded mt-2 pt-6.5"]}>
    <div class={[
      "absolute -top-1/4 bg-surface-800 p-1 text-2xl font-taroca rounded-b", 
      align === "left" && "left-6", 
      align === "center" && "left-1/2 transform -translate-x-1/2",
      align === "right" && "right-6"
    ]}>{title}</div>
    {@render children?.()}
</div>

