<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  
  invoke("get_available_game_systems").then((data) => console.log("From backend: ", data));
</script>

<div class="">Test</div>