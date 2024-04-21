<script lang="ts">
  import DatasetService from "../../services/dataset-service";
  import ModelService from "../../services/model-service";
  import ClassificationReport from "../../lib/components/model/ClassificationReport.svelte";
  import SpecificationReport from "../../lib/components/model/SpecificationReport.svelte";
  import type { Model } from "../../../bindings";
  import ConfusionMatrixReport from "../../lib/components/model/ConfusionMatrixReport.svelte";
  import ModelConfiguration from "../../lib/components/model/ModelConfiguration.svelte";
  import type ModelHyperparameter from "../../models/model-hyperparameter";

  export let name: string = "";

  let model: Nullable<Model>;
  const saveModel = async () => {
    model = null;
  };

  const trainModel = async (modelName: string, hyperparameter: ModelHyperparameter) => {
    await ModelService.trainModel(name, modelName, hyperparameter.kernel);
    model = await ModelService.getModel(modelName);
  };

  const retrainModel = async () => {
    await ModelService.removeModel(model.name);
    model = null;
  };
</script>

<div class="w-full h-full flex flex-col justify-start items-center gap-4 p-4">
  <div class="flex flex-col py-5 gap-5 items-center">
    <h1 class="text-4xl font-bold text-center mt-16">Train Model</h1>
    <p class="text-center text-lg max-w-[40rem]">
      Train a model using the dataset of your choice. You can select the dataset, model name, and kernel type.
    </p>
  </div>
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
</div>
