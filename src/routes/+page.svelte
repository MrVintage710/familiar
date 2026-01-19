<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import CharacterAvatar from "@components/CharacterAvatar.svelte";
  
  invoke("get_available_game_systems").then((data) => console.log("From backend: ", data));
</script>

<div class="flex justify-center items-center h-screen">
    <CharacterAvatar/>
</div>