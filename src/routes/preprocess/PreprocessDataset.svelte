<script lang="ts">
  import DatasetService from "../../services/dataset-service";
  import DatasetCard from "../../lib/components/preprocess/DatasetCard.svelte";
  import EmptyDatasetCard from "../../lib/components/preprocess/EmptyDatasetCard.svelte";
  import Template from "../../lib/components/Template.svelte";
  import NProgress from "nprogress";
  import Loading from "../../lib/components/Loading.svelte";

  const fetchData = async () => {
    NProgress.start();
    const datasets = await DatasetService.getPreprocessDatasets();
    NProgress.done();
    return datasets;
  };
</script>

<Template
  description="Choose your dataset from the list of available datasets to preprocess"
  title="Datasets">
  <div class="w-full flex flex-wrap justify-center items-center gap-4 p-4">
    {#await fetchData()}
      <Loading />
    {:then datasets}
      {#if datasets.length === 0}
        <EmptyDatasetCard />
      {:else}
        {#each datasets as dataset}
          <DatasetCard {dataset} />
        {/each}
      {/if}
    {/await}
  </div>
</Template>
