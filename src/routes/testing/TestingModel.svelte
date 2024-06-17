<script lang="ts">
  import ModelService from "../../services/model-service";
  import ModelCard from "../../lib/components/testing/ModelCard.svelte";
  import Template from "../../lib/components/Template.svelte";
  import NProgress from "nprogress";
  import Loading from "../../lib/components/Loading.svelte";
  import EmptyModelCard from "../../lib/components/testing/EmptyModelCard.svelte";
  import type { Model } from "../../../bindings";
  import SpecificationReport from "../../lib/components/model/SpecificationReport.svelte";
  import ConfusionMatrixReport from "../../lib/components/model/ConfusionMatrixReport.svelte";
  import ClassificationReport from "../../lib/components/model/ClassificationReport.svelte";

  let model: Nullable<Model>;

  const fetchData = async () => {
    NProgress.start();
    const models = await ModelService.getAllModel();
    NProgress.done();
    return models;
  };

  const selectModel = async (name: string) => {
    model = await ModelService.getModel(name);
  };
</script>

<Template
  description="Choose your model from the list below to start testing."
  title="Models">
  <div class="w-full flex flex-wrap justify-center items-center gap-4 p-4">
    {#await fetchData()}
      <Loading />
    {:then models}
      {#if model == null}
        {#if models.length === 0}
          <EmptyModelCard />
        {:else}
          {#each models as model}
            <ModelCard
              {selectModel}
              {model} />
          {/each}
        {/if}
      {:else}
        <div class="w-full h-full flex flex-row justify-center gap-10">
          <SpecificationReport
            isPreview={true}
            {model} />
          <ConfusionMatrixReport {model} />
          <ClassificationReport {model} />
        </div>
      {/if}
    {/await}
  </div>
</Template>
