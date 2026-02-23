<script lang="ts" module>
  import { type ConstructorString, type ConstructorValue } from "$lib/types";
  
  export type ConstructorValueProps = {
    title : string,
    value : ConstructorValue
  }
</script>

<script>
  const {title, value} : ConstructorValueProps = $props();
  
  // const stringValue = () => value["String"] as ConstructorString
</script>



