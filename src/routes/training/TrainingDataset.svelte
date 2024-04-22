<script lang="ts">
  import DatasetService from "../../services/dataset-service";
  import EmptyDatasetCard from "../../lib/components/preprocess/EmptyDatasetCard.svelte";
  import TrainingDatasetCard from "../../lib/components/model/TrainingDatasetCard.svelte";
  import Template from "../../lib/components/Template.svelte";
  import NProgress from "nprogress";
  import Loading from "../../lib/components/Loading.svelte";
  import EmptyTrainDatasetCard from "../../lib/components/model/EmptyTrainDatasetCard.svelte";

  const fetchData = async () => {
    NProgress.start();
    const dataset = await DatasetService.getTrainingDatasets();
    NProgress.done();
    return dataset;
  };
</script>

<Template
  description="Choose your dataset from the list of available datasets to train your model."
  title="Datasets">
  <div class="w-full flex flex-wrap justify-center items-center gap-4 p-4">
    {#await fetchData()}
      <Loading />
    {:then datasets}
      {#if datasets.length === 0}
        <EmptyTrainDatasetCard />
      {:else}
        {#each datasets as dataset}
          <TrainingDatasetCard {dataset} />
        {/each}
      {/if}
    {/await}
  </div>
</Template>
