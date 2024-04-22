<script lang="ts">
  import ModelService from "../../services/model-service";
  import ClassificationReport from "../../lib/components/model/ClassificationReport.svelte";
  import SpecificationReport from "../../lib/components/model/SpecificationReport.svelte";
  import type { Model, ModelHyperparameter } from "../../../bindings";
  import ConfusionMatrixReport from "../../lib/components/model/ConfusionMatrixReport.svelte";
  import ModelConfiguration from "../../lib/components/model/ModelConfiguration.svelte";
  import Template from "../../lib/components/Template.svelte";
  import NProgress from "nprogress";

  export let name: string = "";

  let model: Nullable<Model>;
  const saveModel = async () => {
    model = null;
  };

  const trainModel = async (modelName: string, hyperparameter: ModelHyperparameter) => {
    NProgress.start();
    await ModelService.trainModel(name, modelName, hyperparameter);
    model = await ModelService.getModel(modelName);
    NProgress.done();
  };

  const retrainModel = async () => {
    NProgress.start();
    await ModelService.removeModel(model!.name);
    model = null;
    NProgress.done();
  };
</script>

<Template
  description="Train a model using the dataset of your choice. You can select the dataset, model name, and kernel type."
  title="Train Model">
  <div class="w-full flex flex-row justify-center items-center gap-10 p-4">
    {#if !model}
      <ModelConfiguration
        {trainModel}
        {name} />
    {:else}
      <div class="w-full h-full flex flex-row justify-center gap-10">
        <SpecificationReport
          {retrainModel}
          {saveModel}
          {model} />
        <ConfusionMatrixReport {model} />
        <ClassificationReport {model} />
      </div>
    {/if}
  </div>
</Template>
