<script lang="ts">
  import TauriService from "../services/tauri-service";
  import DatasetCard from "../lib/components/DatasetCard.svelte";

  async function fetchData() {
    const res = await TauriService.getCurrentDirectory();
    console.log(res);
    return await TauriService.getDatasets();
  }

</script>


<div class="w-full h-full flex flex-col justify-start items-center gap-4 p-4">
  <div class="flex flex-col py-5 gap-5 items-center">
    <h1 class="text-4xl font-bold text-center mt-24">Datasets</h1>
    <p class="text-center text-lg max-w-[40rem]">
      Choose your dataset from the list of available datasets.
    </p>
  </div>
  <div class="w-full flex flex-wrap justify-center items-center gap-4 p-4">
    {#await fetchData()}
      <p>Loading</p>
    {:then datasets}
      {#if datasets.length === 0}
        <p>No datasets available</p>
      {:else}
        {#each datasets as d}
          <DatasetCard dataset={d} />
          <DatasetCard dataset={d} />
        {/each}
      {/if}
    {/await}
  </div>
</div>