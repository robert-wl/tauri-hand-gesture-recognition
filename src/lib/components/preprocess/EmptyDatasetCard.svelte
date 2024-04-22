<script lang="ts">
  import { scale } from "svelte/transition";
  import FileDirectoryIcon from "../icons/FileDirectoryIcon.svelte";
  import UtilService from "../../../services/util-service";

  const openDirectory = () => UtilService.openDatasetDirectory();
</script>

<div
  class="card w-80 min-h-40 bg-base-100 shadow-xl border-primary border-t-2"
  in:scale|global={{ duration: 200, opacity: 0.5, start: 0.5 }}>
  <div class="card-body items-center">
    <h2 class="card-title text-xl text-center pb-2">Dataset(s) Not Found</h2>
    <p class="text-center">Please insert a dataset into the <code>/datasets</code> folder.</p>

    <button
      class="btn btn-accent min-h-0 h-10 rounded-sm mt-4"
      on:click={openDirectory}>
      <FileDirectoryIcon />
      <span>Open Directory</span>
    </button>
  </div>
</div>
