<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import CharacterAvatar from "@components/CharacterAvatar.svelte";
    import SearchBar from "@components/SearchBar.svelte";
  
  invoke("get_available_game_systems").then((data) => console.log("From backend: ", data));
</script>

<div class="flex flex-col gap-4 justify-center items-center h-screen">
    <SearchBar class=""/>
    <button type="button" class="btn preset-outlined-primary-500">Button</button>
</div>