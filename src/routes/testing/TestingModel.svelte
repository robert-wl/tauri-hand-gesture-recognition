<script lang="ts">
  import EmptyDatasetCard from "../../lib/components/preprocess/EmptyDatasetCard.svelte";
  import ModelService from "../../services/model-service";
  import ModelCard from "../../lib/components/testing/ModelCard.svelte";
  import Template from "../../lib/components/Template.svelte";
  import NProgress from "nprogress";
  import Loading from "../../lib/components/Loading.svelte";
  import EmptyModelCard from "../../lib/components/testing/EmptyModelCard.svelte";

  const fetchData = async () => {
    NProgress.start();
    const models = await ModelService.getAllModel();
    NProgress.done();
    return models;
  };
</script>

<Template
  description="Choose your model from the list below to start testing."
  title="Models">
  <div class="w-full flex flex-wrap justify-center items-center gap-4 p-4">
    {#await fetchData()}
      <Loading />
    {:then models}
      {#if models.length === 0}
        <EmptyModelCard />
      {:else}
        {#each models as model}
          <ModelCard {model} />
        {/each}
      {/if}
    {/await}
  </div>
</Template>
