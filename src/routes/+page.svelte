<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import CharacterAvatar from "@components/CharacterAvatar.svelte";
  import SearchBar from "@components/SearchBar.svelte";
  import { Plus } from "@lucide/svelte";
  
  
  
  // invoke("get_available_game_systems").then((data) => console.log("From backend: ", data));
</script>

<div class="w-screen h-screen flex flex-col p-4">
  <div class="flex gap-4 justify-center items-center ">
    <SearchBar items={[]} class="w-full"/>
    <button type="button" class="btn preset-filled-primary-100-900 h-12">
      <span class="">New</span>
      <Plus class="stroke-primary-500"/> 
    </button>
  </div>
  <!-- Character List -->
  <div class="w-full min-h-32">
    
  </div>
</div>
