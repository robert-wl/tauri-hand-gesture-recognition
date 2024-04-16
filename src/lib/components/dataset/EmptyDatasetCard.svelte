<script lang="ts">
  import FileDirectoryIcon from "../icons/FileDirectoryIcon.svelte";
  import TauriService from "../../../services/tauri-service";

  function openDirectory() {
    TauriService.openDatasetDirectory();
  }
</script>

<div class="card w-80 min-h-40 bg-base-100 shadow-xl">
  <div class="card-body items-center">
    <h2 class="card-title text-xl text-center pb-2">Dataset Not Found</h2>
    <p class="text-center">Please insert a dataset into the <code>/datasets</code> folder.</p>

    <button
      class="btn btn-accent min-h-0 h-10 rounded-sm mt-4"
      on:click={openDirectory}>
      <FileDirectoryIcon />
      <span>Open Directory</span>
    </button>
  </div>
</div>
