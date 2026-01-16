<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  
  invoke("get_character_constructor").then((data) => console.log("From backend: ", data));
</script>

<div class="">Test</div>