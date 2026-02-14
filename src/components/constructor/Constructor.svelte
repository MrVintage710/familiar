<script lang="ts" module>
  export type ConstructorProps = {
    constructor : any
  }
</script>

<script lang="ts">
  const {constructor} : ConstructorProps = $props();
  
  let currentStep = $state(0);
</script>

<p class="w-screen">
  {JSON.stringify(constructor, null, 4)}
</p>