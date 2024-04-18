<script lang="ts">
  import TauriService from "../../services/tauri-service";
  import DatasetCard from "../../lib/components/dataset/DatasetCard.svelte";
  import EmptyDatasetCard from "../../lib/components/dataset/EmptyDatasetCard.svelte";

  async function fetchData() {
    return await TauriService.getDatasets();
  }
</script>

<div class="w-full h-full flex flex-col justify-start items-center gap-4 p-4">
  <div class="flex flex-col py-5 gap-5 items-center">
    <h1 class="text-4xl font-bold text-center mt-16">Datasets</h1>
    <p class="text-center text-lg max-w-[40rem]">Choose your dataset from the list of available datasets.</p>
  </div>
  <div class="w-full flex flex-wrap justify-center items-center gap-4 p-4">
    {#await fetchData()}
      <p>Loading</p>
    {:then datasets}
      {#if datasets.length === 0}
        <EmptyDatasetCard />
      {:else}
        {#each datasets as dataset}
          <DatasetCard {dataset} />
          <DatasetCard {dataset} />
        {/each}
      {/if}
    {/await}
  </div>
</div>
